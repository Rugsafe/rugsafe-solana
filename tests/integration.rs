use rugsafe_perps::instructions::processor::Processor as PerpsProcessor;
use rugsafe_vaults::instructions::processor::Processor as VaultProcessor;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::program_error::ProgramError;
use solana_program::program_pack::Pack;
use solana_program::{system_instruction, sysvar};
use solana_program_test::*;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
    transport::TransportError,
};
use spl_associated_token_account::get_associated_token_address;
use spl_token::state::Account as TokenAccount;

#[tokio::test]
async fn test_integration_perps_vaults() -> Result<(), TransportError> {
    // Initialize the test environment
    let vaults_program_id = Pubkey::new_unique();
    let perps_program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "rugsafe_vaults",
        vaults_program_id,
        processor!(VaultProcessor::process),
    );

    program_test.add_program(
        "rugsafe_perps",
        perps_program_id,
        processor!(PerpsProcessor::process),
    );

    program_test.add_program(
        "spl_token",
        spl_token::id(),
        processor!(spl_token::processor::Processor::process),
    );

    let mut context = program_test.start_with_context().await;
    let banks_client = &mut context.banks_client;
    let payer = &context.payer;
    let recent_blockhash = banks_client.get_latest_blockhash().await?;

    // Step 1: Create token mint for vault
    let mint_keypair = Keypair::new();
    let mint_rent = banks_client
        .get_rent()
        .await?
        .minimum_balance(TokenAccount::LEN);
    let create_mint_ix = system_instruction::create_account(
        &payer.pubkey(),
        &mint_keypair.pubkey(),
        mint_rent,
        spl_token::state::Mint::LEN as u64,
        &spl_token::id(),
    );
    let initialize_mint_ix = spl_token::instruction::initialize_mint(
        &spl_token::id(),
        &mint_keypair.pubkey(),
        &payer.pubkey(),
        None,
        6, // 6 decimals
    )
    .unwrap();

    let transaction = Transaction::new_signed_with_payer(
        &[create_mint_ix, initialize_mint_ix],
        Some(&payer.pubkey()),
        &[&payer, &mint_keypair],
        recent_blockhash,
    );
    banks_client.process_transaction(transaction).await?;

    // Step 2: Create vault and associated token accounts
    let (vault_pda, _vault_bump) = Pubkey::find_program_address(&[b"vault"], &vaults_program_id);
    let user_token_account = get_associated_token_address(&payer.pubkey(), &mint_keypair.pubkey());

    let create_vault_instruction = create_vault_instruction(
        &vaults_program_id,
        &vault_pda,
        &mint_keypair.pubkey(),
        &payer.pubkey(),
        &payer.pubkey(),
        &spl_token::id(),
        &user_token_account,
    );

    // let mut transaction = Transaction::new_signed_with_payer(
    //     &[create_vault_instruction],
    //     Some(&payer.pubkey()),
    //     &[&payer],
    //     recent_blockhash,
    // );
    // banks_client.process_transaction(transaction).await?;

    // // Step 3: Deposit tokens into the vault and mint anticoins
    // let deposit_amount: u64 = 1_000_000; // 1 token with 6 decimals
    // let deposit_ix = create_deposit_instruction(
    //     &vaults_program_id,
    //     &vault_pda,
    //     &mint_keypair.pubkey(),
    //     &payer.pubkey(),
    //     &user_token_account,
    //     deposit_amount,
    // );

    // transaction = Transaction::new_signed_with_payer(
    //     &[deposit_ix],
    //     Some(&payer.pubkey()),
    //     &[&payer],
    //     recent_blockhash,
    // );
    // banks_client.process_transaction(transaction).await?;

    // // Step 4: Use anticoins as collateral in perpetuals
    // let (user_positions_pda, _user_positions_bump) = Pubkey::find_program_address(
    //     &[b"user_positions", payer.pubkey().as_ref()],
    //     &perps_program_id,
    // );
    // let custody_account = get_associated_token_address(&perps_program_id, &mint_keypair.pubkey());

    // let open_position_ix = create_open_position_instruction(
    //     &perps_program_id,
    //     &user_positions_pda,
    //     &user_token_account,
    //     &custody_account,
    //     deposit_amount, // Using the anticoins minted from the vault
    // );

    // transaction = Transaction::new_signed_with_payer(
    //     &[open_position_ix],
    //     Some(&payer.pubkey()),
    //     &[&payer],
    //     recent_blockhash,
    // );
    // banks_client.process_transaction(transaction).await?;

    // // Step 5: Verify final balances and states
    // let vault_account_data = banks_client.get_account(vault_pda).await?.unwrap();
    // let vault_token_account = TokenAccount::unpack(&vault_account_data.data)
    //     .map_err(|_| BanksClientError::ClientError("Failed to unpack vault account data"))?;

    // let custody_account_data = banks_client.get_account(custody_account).await?.unwrap();
    // let custody_token_account = TokenAccount::unpack(&custody_account_data.data)
    //     .map_err(|_| BanksClientError::ClientError("Failed to unpack custody account data"))?;
    // // Verify that the vault has the deposited amount and custody has the anticoins
    // assert_eq!(vault_token_account.amount, 0); // User deposited all tokens
    // assert_eq!(custody_token_account.amount, deposit_amount); // Custody holds the deposited tokens as collateral

    Ok(())
}

// Helper function to create a vault instruction
fn create_vault_instruction(
    program_id: &Pubkey,
    vault_pda: &Pubkey,
    mint: &Pubkey,
    owner: &Pubkey,
    payer: &Pubkey,
    token_program: &Pubkey,
    user_token_account: &Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*vault_pda, false),
        AccountMeta::new(*mint, false),
        AccountMeta::new(*owner, true),
        AccountMeta::new(*payer, true),
        AccountMeta::new(*user_token_account, false),
        AccountMeta::new_readonly(*token_program, false),
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
        AccountMeta::new_readonly(solana_program::sysvar::rent::id(), false),
    ];
    Instruction {
        program_id: *program_id,
        accounts,
        data: vec![0, 0], // Vault creation data
    }
}

// Helper function to create a deposit instruction
fn create_deposit_instruction(
    program_id: &Pubkey,
    vault_pda: &Pubkey,
    mint: &Pubkey,
    user: &Pubkey,
    user_token_account: &Pubkey,
    amount: u64,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*vault_pda, false),
        AccountMeta::new(*mint, false),
        AccountMeta::new(*user, true),
        AccountMeta::new(*user_token_account, false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(solana_program::sysvar::rent::id(), false),
    ];
    let mut data = vec![0, 1]; // Instruction ID for deposit
    data.extend_from_slice(&amount.to_le_bytes());
    Instruction {
        program_id: *program_id,
        accounts,
        data,
    }
}

// Helper function to create an open position instruction
fn create_open_position_instruction(
    program_id: &Pubkey,
    user_positions_pda: &Pubkey,
    user_token_account: &Pubkey,
    custody_account: &Pubkey,
    collateral_amount: u64,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*user_positions_pda, false),
        AccountMeta::new(*user_token_account, false),
        AccountMeta::new(*custody_account, false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
        AccountMeta::new_readonly(solana_program::sysvar::rent::id(), false),
    ];
    let mut data = vec![0, 0]; // Instruction ID for open position
    data.extend_from_slice(&collateral_amount.to_le_bytes());
    Instruction {
        program_id: *program_id,
        accounts,
        data,
    }
}
