use hello_world::process_instruction;
use solana_program::program_pack::Pack;
use solana_program::system_instruction;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    sysvar,
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
    let vault_keypair = Keypair::new(); // Vault account
    let vault_key = vault_keypair.pubkey();

    let rent_key = solana_program::sysvar::rent::ID;
    let spl_key = spl_token::id();

    let mut program_test =
        ProgramTest::new("hello_world", program_id, processor!(process_instruction));

    // Add SPL Token program
    program_test.add_program(
        "spl_token",
        spl_key,
        processor!(spl_token::processor::Processor::process),
    );

    // Start the context
    let mut context = program_test.start_with_context().await;
    let banks_client = &mut context.banks_client;
    let payer = &context.payer;
    let recent_blockhash = banks_client.get_latest_blockhash().await?;

    // Retrieve rent details
    println!("Retrieving rent details...");
    let rent = banks_client.get_rent().await?;
    let rent_account = banks_client.get_account(rent_key).await?;
    assert!(rent_account.is_some(), "Rent account not found");

    // Create and initialize the mint account before using it in the transaction
    println!("Creating and initializing the mint account...");
    let required_lamports = rent.minimum_balance(Mint::LEN);
    let create_mint_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &mint_key,
        required_lamports,
        Mint::LEN as u64,
        &spl_token::id(),
    );
    let init_mint_ix = spl_token::instruction::initialize_mint(
        &spl_token::id(),
        &mint_key,
        &payer.pubkey(),
        None,
        0,
    )
    .map_err(|e| TransportError::Custom(e.to_string()))?;

    let transaction = Transaction::new_signed_with_payer(
        // &[create_mint_account_ix, init_mint_ix],
        &[create_mint_account_ix],
        Some(&payer.pubkey()),
        &[&payer, &mint_keypair],
        recent_blockhash,
    );

    println!("Processing mint account transaction...");

    banks_client.process_transaction(transaction).await?;

    // Create and initialize the vault account before using it in the transaction
    println!("Creating and initializing the vault account...");
    let create_vault_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &vault_key,
        rent.minimum_balance(TokenAccount::LEN),
        TokenAccount::LEN as u64,
        &spl_token::id(),
    );
    let create_vault_transaction = Transaction::new_signed_with_payer(
        &[create_vault_account_ix],
        Some(&payer.pubkey()),
        &[&payer, &vault_keypair],
        recent_blockhash,
    );
    println!("Processing vault account transaction...");
    banks_client
        .process_transaction(create_vault_transaction)
        .await?;

    // Call the function to create the vault instruction
    println!("Creating vault instruction...");
    let create_vault_instruction = create_vault_instruction(
        &program_id,
        &vault_key,
        &mint_key,
        &payer.pubkey(),
        // &[&payer.pubkey(), &mint_key],
        &[&payer.pubkey(), &mint_key, &vault_key],
    );
    println!("after create_vault_instruction");

    let mut transaction =
        Transaction::new_with_payer(&[create_vault_instruction], Some(&payer.pubkey()));
    println!("Transaction created for vault instruction...");

    // Debugging: Print account owners
    println!("Fetching payer account...");
    let payer_account = banks_client.get_account(payer.pubkey()).await?.unwrap();
    println!("Payer account owner: {:?}", payer_account.owner);

    // Fetch and verify the mint account
    println!("Fetching and verifying the mint account...");
    let mint_account_option = banks_client.get_account(mint_key).await?;
    let mint_account = if let Some(account) = mint_account_option {
        println!("Mint account owner: {:?}", account.owner);
        assert!(account.owner == spl_token::id());
        account
    } else {
        println!("Mint account does not exist or was not created successfully.");
        return Err(TransportError::Custom("Mint account not found".into()));
    };

    // Fetch and verify the vault account
    println!("Fetching and verifying the vault account...");
    let vault_account_option = banks_client.get_account(vault_key).await?;
    if let Some(vault_account) = vault_account_option {
        assert!(vault_account.owner == spl_token::id());

        println!("Payer account owner: {:?}", payer_account.owner);
        println!("Mint account owner: {:?}", mint_account.owner);
        println!("Vault account owner: {:?}", vault_account.owner);

        assert!(payer_account.owner == solana_program::system_program::id());
        assert!(mint_account.owner == spl_token::id());
        assert!(vault_account.owner == spl_token::id());

        println!("Signing the transaction...");
        transaction.sign(&[&payer, &mint_keypair, &vault_keypair], recent_blockhash);
        // transaction.sign(&[&payer, &mint_keypair], recent_blockhash);

        println!("after sign");

        // Process CreateVault transaction
        println!("Processing CreateVault transaction...");
        match banks_client.process_transaction(transaction).await {
            Ok(_) => println!("Transaction processed successfully"),
            Err(e) => {
                println!("Transaction failed: {:?}", e);
                return Err(e.into());
            }
        }

        // Verify vault creation
        println!("Verifying vault creation...");
        let mint_account = banks_client.get_account(mint_key).await?;
        assert!(mint_account.is_some(), "Mint account not created");
        let vault_account = banks_client.get_account(vault_key).await?;
        assert!(vault_account.is_some(), "Vault account not created");

        println!("Test completed successfully.");
        Ok(())
    } else {
        println!("Vault account does not exist or was not created successfully.");
        return Err(TransportError::Custom("Vault account not found".into()));
    }
}

fn create_vault_instruction(
    program_id: &Pubkey,
    vault_key: &Pubkey,
    mint_key: &Pubkey,
    payer: &Pubkey,
    signer_keys: &[&Pubkey],
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*payer, true),
        AccountMeta::new(*mint_key, true),
        AccountMeta::new(*vault_key, true),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
    ];

    Instruction {
        program_id: *program_id,
        accounts,
        data: vec![0], // Add any additional data if needed
    }
}

#[tokio::test]
async fn test_deposit() -> Result<(), BanksClientError> {
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
        processor!(spl_token::processor::Processor::process),
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

    // Create the vault using the create_vault functionality
    println!("Creating the vault...");
    let vault_keypair = Keypair::new();
    let vault_key = vault_keypair.pubkey();
    let create_vault_instruction = create_vault_instruction(
        &program_id,
        &vault_key,
        &mint_key,
        &payer.pubkey(),
        &[&payer.pubkey(), &mint_key], // Ensure the 5th argument is supplied
    );

    let mut transaction = Transaction::new_signed_with_payer(
        &[create_vault_instruction],
        Some(&payer.pubkey()),
        &[&payer, &mint_keypair, &vault_keypair],
        recent_blockhash,
    );

    // Debugging: Print key information
    println!("Payer Pubkey: {:?}", payer.pubkey());
    println!("Vault Keypair Pubkey: {:?}", vault_keypair.pubkey());
    println!("Mint Key: {:?}", mint_key);

    banks_client.process_transaction(transaction).await?;

    // Creating user TokenA account
    println!("Creating user TokenA account...");
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
            AccountMeta::new(vault_key, false),
            AccountMeta::new(mint_key, false),
            AccountMeta::new(user_a_token_a_account.pubkey(), false),
            AccountMeta::new_readonly(rent_key, false),
            AccountMeta::new_readonly(spl_key, false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
        ],
        data: vec![0, 0, 0, 100], // Assume the deposit amount is packed in the instruction data
    };

    println!("Signing and processing deposit transaction...");

    let mut transaction =
        Transaction::new_with_payer(&[deposit_instruction], Some(&payer.pubkey()));
    transaction.sign(&[&payer, &user_token_a_account], recent_blockhash);

    banks_client.process_transaction(transaction).await?;

    println!("Checking balances and asserting expected outcomes...");

    // Check balances and assert expected outcomes
    let user_token_a_account_info = banks_client
        .get_account(user_token_a_account.pubkey())
        .await?
        .unwrap();
    let vault_token_a_account_info = banks_client.get_account(vault_key).await?.unwrap();
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
