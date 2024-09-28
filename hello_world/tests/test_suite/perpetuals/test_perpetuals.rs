// Import necessary dependencies
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::{system_instruction, sysvar};
use solana_program_test::*;

use solana_sdk::{
    program_pack::Pack,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

use spl_token::state::Account as TokenAccount;

use borsh::{BorshDeserialize, BorshSerialize};
use rugsafe::instructions::processor::Processor;

use rugsafe::state::perpetuals::{Position, Side, UserPositions};
// use solana_logger;

#[tokio::test]
async fn test_open_position() {
    // solana_logger::setup();

    // Step 1: Initialize the program ID and set up the ProgramTest environment
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new("rugsafe", program_id, processor!(Processor::process));

    // Add the SPL Token program to the test environment
    program_test.add_program(
        "spl_token",
        spl_token::id(),
        processor!(spl_token::processor::Processor::process),
    );

    // Start the test context
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // **Step 2: Create the collateral token mint**
    let collateral_mint = Keypair::new();
    let mint_rent = banks_client
        .get_rent()
        .await
        .unwrap()
        .minimum_balance(spl_token::state::Mint::LEN);

    let create_mint_ix = system_instruction::create_account(
        &payer.pubkey(),
        &collateral_mint.pubkey(),
        mint_rent,
        spl_token::state::Mint::LEN as u64,
        &spl_token::id(),
    );

    let initialize_mint_ix = spl_token::instruction::initialize_mint(
        &spl_token::id(),
        &collateral_mint.pubkey(),
        &payer.pubkey(),
        None,
        6, // Decimals
    )
    .unwrap();

    let transaction = Transaction::new_signed_with_payer(
        &[create_mint_ix, initialize_mint_ix],
        Some(&payer.pubkey()),
        &[&payer, &collateral_mint],
        recent_blockhash,
    );

    banks_client.process_transaction(transaction).await.unwrap();

    // **Step 3: Create user's collateral token account and mint tokens to it**
    let user_collateral_account = Keypair::new();
    let rent = banks_client
        .get_rent()
        .await
        .unwrap()
        .minimum_balance(TokenAccount::LEN);

    let create_user_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &user_collateral_account.pubkey(),
        rent,
        TokenAccount::LEN as u64,
        &spl_token::id(),
    );

    let initialize_user_account_ix = spl_token::instruction::initialize_account(
        &spl_token::id(),
        &user_collateral_account.pubkey(),
        &collateral_mint.pubkey(),
        &payer.pubkey(),
    )
    .unwrap();

    // Mint tokens to user's collateral account
    let mint_to_user_ix = spl_token::instruction::mint_to(
        &spl_token::id(),
        &collateral_mint.pubkey(),
        &user_collateral_account.pubkey(),
        &payer.pubkey(),
        &[],
        1_000_000_000, // Mint 1,000 tokens (assuming 6 decimals)
    )
    .unwrap();

    let transaction = Transaction::new_signed_with_payer(
        &[
            create_user_account_ix,
            initialize_user_account_ix,
            mint_to_user_ix,
        ],
        Some(&payer.pubkey()),
        &[&payer, &user_collateral_account],
        recent_blockhash,
    );

    banks_client.process_transaction(transaction).await.unwrap();

    // **Step 4: Create the custody account (where collateral is stored)**
    let custody_account = Keypair::new();

    let create_custody_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &custody_account.pubkey(),
        rent,
        TokenAccount::LEN as u64,
        &spl_token::id(),
    );

    let initialize_custody_account_ix = spl_token::instruction::initialize_account(
        &spl_token::id(),
        &custody_account.pubkey(),
        &collateral_mint.pubkey(),
        &program_id, // Set the owner to the program ID
    )
    .unwrap();

    let transaction = Transaction::new_signed_with_payer(
        &[create_custody_account_ix, initialize_custody_account_ix],
        Some(&payer.pubkey()),
        &[&payer, &custody_account],
        recent_blockhash,
    );

    banks_client.process_transaction(transaction).await.unwrap();

    // **Step 5: Create the user positions account**
    let user_positions_account = Keypair::new();

    let user_positions_space = UserPositions::LEN;
    let rent = banks_client.get_rent().await.unwrap();
    let required_lamports = rent.minimum_balance(user_positions_space);

    let create_user_positions_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &user_positions_account.pubkey(),
        required_lamports,
        user_positions_space as u64,
        &program_id,
    );

    let transaction = Transaction::new_signed_with_payer(
        &[create_user_positions_account_ix],
        Some(&payer.pubkey()),
        &[&payer, &user_positions_account],
        recent_blockhash,
    );

    banks_client.process_transaction(transaction).await.unwrap();

    // **Step 6: Construct the OpenPosition instruction**
    let side = Side::Long;
    let amount: u64 = 500_000_000; // Amount of collateral to deposit (500 tokens)

    let module_tag = 1; // Replace with the actual module tag for Perpetuals
    let instruction_tag = 0; // Instruction tag for OpenPosition

    let side_byte = match side {
        Side::Long => 1,
        Side::Short => 2,
        _ => panic!("Invalid side"),
    };

    let mut instruction_data = Vec::with_capacity(11);
    instruction_data.push(module_tag); // Module tag for Perpetuals
    instruction_data.push(instruction_tag); // Instruction tag for OpenPosition
    instruction_data.push(side_byte); // Side byte
    instruction_data.extend_from_slice(&amount.to_le_bytes()); // Amount as u64 in little-endian

    // Create the instruction
    let open_position_ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true), // Payer (signer)
            AccountMeta::new(user_positions_account.pubkey(), false), // UserPositions account (writable)
            AccountMeta::new(user_collateral_account.pubkey(), false), // User's collateral token account (writable)
            AccountMeta::new(custody_account.pubkey(), false),         // Custody account (writable)
            AccountMeta::new_readonly(spl_token::id(), false),         // SPL Token Program
        ],
        data: instruction_data,
    };

    // **Step 7: Send the transaction**
    let transaction = Transaction::new_signed_with_payer(
        &[open_position_ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    banks_client.process_transaction(transaction).await.unwrap();

    // **Step 8: Verify the results**
    // Fetch the user positions account
    let user_positions_account_data = banks_client
        .get_account(user_positions_account.pubkey())
        .await
        .unwrap()
        .unwrap();

    // Deserialize the user positions data
    // let user_positions = UserPositions::try_from_slice(&user_positions_account_data.data).unwrap();
    let mut data_slice: &[u8] = &user_positions_account_data.data;
    let user_positions = UserPositions::deserialize(&mut data_slice).unwrap();

    // Assert that a position was added
    assert_eq!(user_positions.positions.len(), 1);

    let position = &user_positions.positions[0];

    // Assert position fields
    assert_eq!(position.owner, payer.pubkey());
    assert_eq!(position.side, Side::Long);
    assert_eq!(position.size_usd, amount);

    // **Check token balances**
    // User's collateral account balance should decrease by 'amount'
    let user_collateral_account_data = banks_client
        .get_account(user_collateral_account.pubkey())
        .await
        .unwrap()
        .unwrap();
    let user_token_account = TokenAccount::unpack(&user_collateral_account_data.data).unwrap();

    assert_eq!(
        user_token_account.amount,
        500_000_000 // 1,000,000,000 - 500,000,000
    );

    // Custody account balance should increase by 'amount'
    let custody_account_data = banks_client
        .get_account(custody_account.pubkey())
        .await
        .unwrap()
        .unwrap();
    let custody_token_account = TokenAccount::unpack(&custody_account_data.data).unwrap();

    assert_eq!(custody_token_account.amount, 500_000_000);

    println!("Test passed: Position opened successfully.");
}
