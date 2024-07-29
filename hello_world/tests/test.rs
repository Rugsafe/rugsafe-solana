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
    let program_test = ProgramTest::new("hello_world", program_id, processor!(process_instruction));

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
    // Setup keys and program
    let program_id = Pubkey::new_unique();
    let mint_keypair = Keypair::new(); // Mint account
    let mint_key = mint_keypair.pubkey();

    let owner_keypair = Keypair::new(); // Owner's keypair

    //debug
    // let owner_key = owner_keypair.pubkey();
    let owner_key = Pubkey::new_unique(); // Owner's pubkey (not a keypair)

    let rent_key = solana_program::sysvar::rent::ID;
    let spl_key = spl_token::id();

    let mut program_test =
        ProgramTest::new("hello_world", program_id, processor!(process_instruction));

    // Add SPL Token program
    program_test.add_program("spl_token", spl_key, None);

    // Create mint account
    // program_test.add_account(
    //     mint_key,
    //     Account {
    //         lamports: 1, //1_000_000_000,
    //         data: vec![0; Mint::LEN],
    //         owner: spl_key,
    //         executable: false,
    //         rent_epoch: 0,
    //     },
    // );

    // Create owner account
    program_test.add_account(
        owner_key,
        Account {
            lamports: 1_000_000,
            data: vec![],
            owner: program_id,
            executable: false,
            rent_epoch: 0,
        },
    );

    // Start the context
    let mut context = program_test.start_with_context().await;
    let banks_client = &mut context.banks_client;
    let payer = &context.payer;
    let recent_blockhash = banks_client.get_latest_blockhash().await?;

    // Verify accounts
    // let mint_account = banks_client.get_account(mint_key).await?;
    // assert!(mint_account.is_some(), "Mint account not created or funded");

    let owner_account = banks_client.get_account(owner_key).await?;
    assert!(
        owner_account.is_some(),
        "Owner account not created or funded"
    );

    let rent_account = banks_client.get_account(rent_key).await?;
    assert!(rent_account.is_some(), "Rent account not found");

    // Initialize mint account
    // let init_mint_ix =
    //     initialize_mint(&spl_key, &mint_key, &owner_key, Some(&owner_key), 0).unwrap();
    // let mut init_mint_tx = Transaction::new_with_payer(&[init_mint_ix], Some(&payer.pubkey()));
    // init_mint_tx.sign(&[&payer], recent_blockhash);
    // banks_client.process_transaction(init_mint_tx).await?;

    // Create CreateVault instruction
    let instruction_data = [0u8];
    let accounts = vec![
        AccountMeta::new(payer.pubkey(), true), // Payer account (and signer)
        AccountMeta::new(mint_key, false),
        AccountMeta::new(owner_key, false), // Owner needs to sign
        AccountMeta::new_readonly(rent_key, false),
        AccountMeta::new(spl_key, false),
    ];

    let create_vault_instruction = Instruction {
        program_id,
        accounts,
        data: instruction_data.to_vec(),
    };

    let mut transaction =
        Transaction::new_with_payer(&[create_vault_instruction], Some(&payer.pubkey()));

    // Debugging: Print key information
    println!("Payer Pubkey: {:?}", payer.pubkey());
    println!("Owner Keypair Pubkey: {:?}", owner_keypair.pubkey());
    println!("Owner Key: {:?}", owner_key);
    println!("Mint Keypair Pubkey: {:?}", mint_keypair.pubkey());
    println!("Mint Key: {:?}", mint_key);

    ///// TESTS PASS UP UNTIL HERE
    // transaction.sign(&[payer], recent_blockhash);
    transaction.sign(&[payer], recent_blockhash);
    // transaction.sign(&[&payer, &owner_keypair], recent_blockhash);
    // transaction.sign(&[&payer, &mint_keypair], recent_blockhash);

    println!("after sign");

    // // // Process CreateVault transaction
    match banks_client.process_transaction(transaction).await {
        Ok(_) => println!("Transaction processed successfully"),
        Err(e) => {
            println!("Transaction failed: {:?}", e);
            return Err(e.into());
        }
    }

    // // Verify vault creation
    // let mint_account = banks_client.get_account(mint_key).await?;
    // assert!(mint_account.is_some(), "Mint account not created");

    Ok(())
}
