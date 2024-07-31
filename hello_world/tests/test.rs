use hello_world::process_instruction;
use solana_program::program_pack::Pack;
use solana_program::system_instruction;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    sysvar::rent::ID as RentSysvarId,
};
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
    transport::TransportError,
};
use spl_token::instruction::initialize_mint;
use spl_token::state::Account as TokenAccount;
use spl_token::state::Mint; // Import for unpacking account data

fn program_error_to_banks_client_error(e: ProgramError) -> BanksClientError {
    BanksClientError::ClientError(Box::leak(Box::new(e.to_string())))
}

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

    let rent_key = solana_program::sysvar::rent::ID;
    let spl_key = spl_token::id();

    let mut program_test =
        ProgramTest::new("hello_world", program_id, processor!(process_instruction));

    // Add SPL Token program
    program_test.add_program("spl_token", spl_key, None);

    // Start the context
    let mut context = program_test.start_with_context().await;
    let banks_client = &mut context.banks_client;
    let payer = &context.payer;
    let recent_blockhash = banks_client.get_latest_blockhash().await?;

    let rent_account = banks_client.get_account(rent_key).await?;
    assert!(rent_account.is_some(), "Rent account not found");

    // Create CreateVault instruction
    let instruction_data = [0u8];
    let accounts = vec![
        AccountMeta::new(payer.pubkey(), true),
        AccountMeta::new(mint_key, true),
        AccountMeta::new(rent_key, false),
        AccountMeta::new(spl_key, false),
        AccountMeta::new(solana_program::system_program::id(), false),
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
    println!("Mint Keypair Pubkey: {:?}", mint_keypair.pubkey());
    println!("Mint Key: {:?}", mint_key);

    transaction.sign(&[&payer, &mint_keypair], recent_blockhash);

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
    let mint_account = banks_client.get_account(mint_key).await?;
    assert!(mint_account.is_some(), "Mint account not created");

    Ok(())
}

#[tokio::test]
async fn test_process_deposit() -> Result<(), BanksClientError> {
    let program_id = Pubkey::new_unique();
    let mint_keypair = Keypair::new(); // Mint account for aTokenA
    let mint_key = mint_keypair.pubkey();

    let token_a_mint_keypair = Keypair::new(); // Mint account for TokenA
    let token_a_mint_key = token_a_mint_keypair.pubkey();

    println!("program_id: {:?}", program_id);
    println!("mint_key: {:?}", mint_key);
    println!("token_a_mint_key: {:?}", token_a_mint_key);

    let rent_key = solana_program::sysvar::rent::ID;
    let spl_key = spl_token::id();

    let mut program_test =
        ProgramTest::new("hello_world", program_id, processor!(process_instruction));

    program_test.add_program(
        "spl_token",
        spl_key,
        processor!(spl_token::processor::Processor::process), // HERE
    );

    println!("Starting program test context...");

    let mut context = program_test.start_with_context().await;
    let banks_client = &mut context.banks_client;
    let payer = &context.payer;
    let recent_blockhash = banks_client.get_latest_blockhash().await?;

    println!("recent_blockhash: {:?}", recent_blockhash);

    // Initialize TokenA mint
    println!("Creating TokenA mint account...");

    // Create TokenA mint account
    let rent = banks_client.get_rent().await?;
    let required_lamports = rent.minimum_balance(Mint::LEN);

    let create_token_a_mint_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &token_a_mint_key,
        required_lamports,
        Mint::LEN as u64,
        &spl_token::id(),
    );
    let transaction = Transaction::new_signed_with_payer(
        &[create_token_a_mint_account_ix],
        Some(&payer.pubkey()),
        &[payer, &token_a_mint_keypair],
        recent_blockhash,
    );
    banks_client.process_transaction(transaction).await?;

    println!("Initializing TokenA mint...");

    let init_token_a_mint_ix = spl_token::instruction::initialize_mint(
        &spl_token::id(),
        &token_a_mint_key,
        &payer.pubkey(),
        Some(&payer.pubkey()),
        0,
    )
    .map_err(program_error_to_banks_client_error)?;
    let transaction = Transaction::new_signed_with_payer(
        &[init_token_a_mint_ix],
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhash,
    );
    banks_client.process_transaction(transaction).await?;

    // Initialize aTokenA mint
    println!("Creating aTokenA mint account...");

    // Create aTokenA mint account
    let create_a_token_a_mint_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &mint_key,
        required_lamports,
        Mint::LEN as u64,
        &spl_token::id(),
    );
    let transaction = Transaction::new_signed_with_payer(
        &[create_a_token_a_mint_account_ix],
        Some(&payer.pubkey()),
        &[payer, &mint_keypair],
        recent_blockhash,
    );
    banks_client.process_transaction(transaction).await?;

    println!("Initializing aTokenA mint...");

    let init_a_token_a_mint_ix = spl_token::instruction::initialize_mint(
        &spl_token::id(),
        &mint_key,
        &payer.pubkey(),
        Some(&payer.pubkey()),
        0,
    )
    .map_err(program_error_to_banks_client_error)?;
    let transaction = Transaction::new_signed_with_payer(
        &[init_a_token_a_mint_ix],
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhash,
    );
    banks_client.process_transaction(transaction).await?;

    /////////////////////////////////////////////
    /// /////////////////////////////////////////////
    /// /////////////////////////////////////////////
    /// /////////////////////////////////////////////
    /// /////////////////////////////////////////////

    println!("Creating user TokenA account...");
    // Create user TokenA account
    let user_token_a_account = Keypair::new();
    let create_user_token_a_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &user_token_a_account.pubkey(),
        rent.minimum_balance(TokenAccount::LEN),
        TokenAccount::LEN as u64,
        &spl_token::id(),
    );
    let transaction = Transaction::new_signed_with_payer(
        &[create_user_token_a_account_ix],
        Some(&payer.pubkey()),
        &[payer, &user_token_a_account],
        recent_blockhash,
    );
    banks_client.process_transaction(transaction).await?;

    let initialize_user_token_a_account_ix = spl_token::instruction::initialize_account(
        &spl_token::id(),
        &user_token_a_account.pubkey(),
        &token_a_mint_key,
        &payer.pubkey(),
    )
    .map_err(program_error_to_banks_client_error)?;
    let transaction = Transaction::new_signed_with_payer(
        &[initialize_user_token_a_account_ix],
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhash,
    );
    banks_client.process_transaction(transaction).await?;

    println!("Creating vault TokenA account...");

    // Create vault TokenA account
    let vault_token_a_account = Keypair::new();
    let create_vault_token_a_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &vault_token_a_account.pubkey(),
        rent.minimum_balance(TokenAccount::LEN),
        TokenAccount::LEN as u64,
        &spl_token::id(),
    );
    let transaction = Transaction::new_signed_with_payer(
        &[create_vault_token_a_account_ix],
        Some(&payer.pubkey()),
        &[payer, &vault_token_a_account],
        recent_blockhash,
    );
    banks_client.process_transaction(transaction).await?;

    let initialize_vault_token_a_account_ix = spl_token::instruction::initialize_account(
        &spl_token::id(),
        &vault_token_a_account.pubkey(),
        &token_a_mint_key,
        &payer.pubkey(),
    )
    .map_err(program_error_to_banks_client_error)?;
    let transaction = Transaction::new_signed_with_payer(
        &[initialize_vault_token_a_account_ix],
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhash,
    );
    banks_client.process_transaction(transaction).await?;

    println!("Creating user aTokenA account...");

    // Create user aTokenA account
    let user_a_token_a_account = Keypair::new();
    let create_user_a_token_a_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &user_a_token_a_account.pubkey(),
        rent.minimum_balance(TokenAccount::LEN),
        TokenAccount::LEN as u64,
        &spl_token::id(),
    );
    let transaction = Transaction::new_signed_with_payer(
        &[create_user_a_token_a_account_ix],
        Some(&payer.pubkey()),
        &[payer, &user_a_token_a_account],
        recent_blockhash,
    );
    banks_client.process_transaction(transaction).await?;

    let initialize_user_a_token_a_account_ix = spl_token::instruction::initialize_account(
        &spl_token::id(),
        &user_a_token_a_account.pubkey(),
        &mint_key,
        &payer.pubkey(),
    )
    .map_err(program_error_to_banks_client_error)?;
    let transaction = Transaction::new_signed_with_payer(
        &[initialize_user_a_token_a_account_ix],
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhash,
    );
    banks_client.process_transaction(transaction).await?;

    println!("Minting TokenA to the user's account...");

    // Mint TokenA to the user's account
    let mint_to_user_token_a_ix = spl_token::instruction::mint_to(
        &spl_token::id(),
        &token_a_mint_key,
        &user_token_a_account.pubkey(),
        &payer.pubkey(),
        &[],
        100, // Mint 100 TokenA for testing
    )
    .map_err(program_error_to_banks_client_error)?;
    let transaction = Transaction::new_signed_with_payer(
        &[mint_to_user_token_a_ix],
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhash,
    );
    banks_client.process_transaction(transaction).await?;

    // Construct deposit instruction
    println!("Constructing deposit instruction...");

    let deposit_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(user_token_a_account.pubkey(), true),
            AccountMeta::new(vault_token_a_account.pubkey(), false),
            AccountMeta::new(mint_key, false),
            AccountMeta::new(user_a_token_a_account.pubkey(), false),
            AccountMeta::new_readonly(rent_key, false),
            AccountMeta::new_readonly(spl_key, false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
        ],
        data: vec![0, 0, 0, 100], // Assume the deposit amount is packed in the instruction data
    };

    ////////////////////////////////////////////////////////?
    ///
    ////////////////////////////////////////////////////////?
    ///
    ////////////////////////////////////////////////////////?
    ///
    ///

    println!("Signing and processing deposit transaction...");

    let mut transaction =
        Transaction::new_with_payer(&[deposit_instruction], Some(&payer.pubkey()));
    // transaction.sign(&[&payer], recent_blockhash); // dont work
    // transaction.sign(&[&payer, &user_token_a_account], recent_blockhash);
    // transaction.sign(&[&payer, &useraccount], recent_blockhash);
    transaction.sign(
        &[
            &payer,
            &user_token_a_account,
            // &vault_token_a_account,
            // &user_a_token_a_account,
            // &mint_key,
            // &user_a_token_a_account,
        ],
        recent_blockhash,
    );

    banks_client.process_transaction(transaction).await?;

    println!("Checking balances and asserting expected outcomes...");

    // Check balances and assert expected outcomes
    let user_token_a_account_info = banks_client
        .get_account(user_token_a_account.pubkey())
        .await?
        .unwrap();
    let vault_token_a_account_info = banks_client
        .get_account(vault_token_a_account.pubkey())
        .await?
        .unwrap();
    let user_a_token_a_account_info = banks_client
        .get_account(user_a_token_a_account.pubkey())
        .await?
        .unwrap();

    println!("user_token_a_account_info: {:?}", user_token_a_account_info);
    println!(
        "vault_token_a_account_info: {:?}",
        vault_token_a_account_info
    );
    println!(
        "user_a_token_a_account_info: {:?}",
        user_a_token_a_account_info
    );

    println!("Unpacking token balances...");
    // Assert expected token balances
    let user_token_a_balance = TokenAccount::unpack(&user_token_a_account_info.data)
        .map_err(program_error_to_banks_client_error)?
        .amount;
    let vault_token_a_balance = TokenAccount::unpack(&vault_token_a_account_info.data)
        .map_err(program_error_to_banks_client_error)?
        .amount;
    let user_a_token_a_balance = TokenAccount::unpack(&user_a_token_a_account_info.data)
        .map_err(program_error_to_banks_client_error)?
        .amount;

    println!("user_token_a_balance: {:?}", user_token_a_balance);
    println!("vault_token_a_balance: {:?}", vault_token_a_balance);
    println!("user_a_token_a_balance: {:?}", user_a_token_a_balance);

    println!("Asserting expected token balances...");
    assert_eq!(user_token_a_balance, 0); // User should have 0 TokenA
    assert_eq!(vault_token_a_balance, 100); // Vault should have 100 TokenA
    assert_eq!(user_a_token_a_balance, 100); // User should have 100 aTokenA

    Ok(())
}
