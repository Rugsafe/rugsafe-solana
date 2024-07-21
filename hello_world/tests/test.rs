use hello_world::process_instruction;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::program_pack::Pack;
use solana_program::system_instruction;
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
    transport::TransportError,
};
use spl_token::instruction::initialize_mint;
use spl_token::state::Mint;

#[tokio::test]
async fn test_process_instruction() -> Result<(), TransportError> {
    // Start the test environment
    let program_id = Pubkey::new_unique();
    let program_test = ProgramTest::new(
        "hello_world_program",
        program_id,
        processor!(process_instruction),
    );

    // Add accounts to the test environment
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Create a new Keypair for the account
    let new_account = Keypair::new();

    // Create a transaction to invoke the program
    let mut transaction = Transaction::new_with_payer(
        &[system_instruction::create_account(
            &payer.pubkey(),
            &new_account.pubkey(),
            1_000_000,
            0,
            &program_id,
        )],
        Some(&payer.pubkey()),
    );

    // Sign the transaction with both the payer and the new account keypair
    transaction.sign(&[&payer, &new_account], recent_blockhash);

    // Process the transaction
    banks_client.process_transaction(transaction).await?;

    Ok(())
}
#[tokio::test]
async fn test_create_vault() -> Result<(), TransportError> {
    // Start the test environment
    let program_id = Pubkey::new_unique();
    let mint_key = Pubkey::new_unique();
    let owner_keypair = Keypair::new();
    let owner_key = owner_keypair.pubkey();
    let rent_key = solana_program::sysvar::rent::ID;

    let mut program_test = ProgramTest::new(
        "hello_world_program",
        program_id,
        processor!(process_instruction),
    );

    // Add the SPL Token program to the test environment with its processor
    program_test.add_program(
        "spl_token",
        spl_token::id(),
        processor!(spl_token::processor::Processor::process),
    );

    // Create a rent-exempt mint account with increased lamports
    program_test.add_account(
        mint_key,
        Account {
            lamports: 1_000_000_000, // Ensure the account is rent-exempt
            data: vec![0; Mint::LEN],
            owner: spl_token::id(),
            executable: false,
            rent_epoch: 0,
        },
    );

    // Create a rent-exempt owner account
    program_test.add_account(
        owner_key,
        Account {
            lamports: 1_000_000, // Ensure the account is funded
            data: vec![],
            owner: program_id,
            executable: false,
            rent_epoch: 0,
        },
    );

    // Start the context
    let mut context = program_test.start_with_context().await;

    // Warp to a new slot to avoid race conditions
    context.warp_to_slot(5).unwrap();

    let banks_client = &mut context.banks_client;
    let payer = &context.payer;

    // Refresh blockhash before creating the transaction
    let recent_blockhash = banks_client
        .get_new_latest_blockhash(&context.last_blockhash)
        .await?;
    println!("Updated blockhash: {:?}", recent_blockhash);

    // Debugging: Verify that the accounts are created and funded
    println!("Checking if mint account is created...");
    let mint_account = banks_client.get_account(mint_key).await?;
    assert!(mint_account.is_some(), "Mint account not created or funded");
    println!("Mint account balance: {:?}", mint_account.unwrap().lamports);

    println!("Checking if owner account is created...");
    let owner_account = banks_client.get_account(owner_key).await?;
    assert!(
        owner_account.is_some(),
        "Owner account not created or funded"
    );
    println!(
        "Owner account balance: {:?}",
        owner_account.unwrap().lamports
    );

    println!("Checking if rent account is created...");
    let rent_account = banks_client.get_account(rent_key).await?;
    assert!(rent_account.is_some(), "Rent account not found");
    println!("Rent account balance: {:?}", rent_account.unwrap().lamports);

    println!("Mint account key: {:?}", mint_key);
    println!("Owner account key: {:?}", owner_key);
    println!("Rent account key: {:?}", rent_key);

    // Create a transaction to send the CreateVault instruction
    let instruction_data = [0u8]; // CreateVault instruction identifier

    let accounts = vec![
        AccountMeta::new(mint_key, false),
        AccountMeta::new(owner_key, true), // Make the owner a signer
        AccountMeta::new_readonly(rent_key, false),
        AccountMeta::new_readonly(spl_token::id(), false), // Add SPL Token program
    ];

    let create_vault_instruction = Instruction {
        program_id,
        accounts,
        data: instruction_data.to_vec(),
    };

    let mut transaction =
        Transaction::new_with_payer(&[create_vault_instruction], Some(&payer.pubkey()));

    // Sign the transaction with the payer and owner_keypair
    transaction.sign(&[payer, &owner_keypair], recent_blockhash);

    // Debugging: Log the transaction details
    println!("Transaction details: {:?}", transaction);

    // Process the transaction
    match banks_client.process_transaction(transaction).await {
        Ok(_) => println!("Transaction processed successfully"),
        Err(e) => {
            println!("Transaction failed: {:?}", e);
            return Err(e.into());
        }
    }

    // Verify the vault was created (additional checks can be added here)
    println!("Verifying the vault creation...");
    let mint_account = banks_client.get_account(mint_key).await?;
    assert!(mint_account.is_some(), "Mint account not created");

    Ok(())
}
