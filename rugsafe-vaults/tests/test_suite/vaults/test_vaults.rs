// use borsh::de::BorshDeserialize;
// use borsh::{BorshDeserialize, BorshSerialize};
use hex;
use rugsafe_vaults::instructions::vaults::VaultInstruction;
use rugsafe_vaults::process_instruction;
// use rugsafe::processor::Processor;
use rugsafe_vaults::instructions::processor::Processor;
use rugsafe_vaults::state::vaults::{Vault, VaultRegistry};
use solana_program::hash::Hash;
use solana_program::program_pack::Pack;
use solana_program::rent::Rent;
use solana_program::system_instruction;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    msg,
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
use spl_associated_token_account::get_associated_token_address;
use spl_token::instruction::initialize_mint;
use spl_token::state::Account as TokenAccount;
use spl_token::state::Mint;
use std::iter;

fn program_error_to_banks_client_error(e: ProgramError) -> BanksClientError {
    BanksClientError::ClientError(Box::leak(Box::new(e.to_string())))
    // return Err(BanksClientError::ClientError(Box::new(e.to_string())));
    // return Err(BanksClientError::ClientError(
    //     Box::leak(Box::new(e.to_string())).as_str(),
    // ));
}

#[tokio::test]
async fn test_process_instruction() -> Result<(), TransportError> {
    // Start the test environment
    let program_id = Pubkey::new_unique();
    let program_test = ProgramTest::new(
        "rugsafe_vaults",
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
    // Setup keys and program
    let program_id = Pubkey::new_unique();

    let rent_key = solana_program::sysvar::rent::ID;
    let spl_key = spl_token::id();
    let associated_token_program: Pubkey = spl_associated_token_account::id();

    // let state_keypair = Keypair::new(); // State account

    // let state_key = state_keypair.pubkey();
    let (state_key, _bump_seed) = Pubkey::find_program_address(&[b"vault_registry"], &program_id);

    let mut program_test = ProgramTest::new(
        "rugsafe_vaults",
        program_id,
        processor!(process_instruction),
    );

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
    let rent_account = banks_client.get_account(rent_key).await?;
    assert!(rent_account.is_some(), "Rent account not found");

    ////////////////////////
    /// ///////////////// TODO
    /// /////////////////////
    // Create token mint
    let mint_tokena_keypair =
        create_token_mint(banks_client, &payer, recent_blockhash, &payer.pubkey()).await?;
    let mint_tokena_key = mint_tokena_keypair.pubkey();

    // NOTE: SHOULD THE A TOKEN MINT BE A NEW KEYPAIR?
    let mint_atokena_keypair = Keypair::new(); // Mint account
    let mint_atokena_key = mint_atokena_keypair.pubkey();

    // NOTE vault key should be associated token account for token a's mint
    // let vault_keypair = Keypair::new(); // Vault account
    // let vault_key = vault_keypair.pubkey();
    // let vault_key: Pubkey = get_associated_token_address(&payer.pubkey(), &mint_tokena_key);
    let vault_key: Pubkey = get_associated_token_address(&program_id, &mint_tokena_key); //associated address for programid and tokenamint
    let user_token_a_key: Pubkey = get_associated_token_address(&payer.pubkey(), &mint_tokena_key);

    // Call the function to create the vault instruction
    println!("Creating vault instruction...");
    let create_vault_instruction = create_vault_instruction(
        &program_id,
        &vault_key,
        &mint_tokena_key,
        &mint_atokena_key,
        &payer.pubkey(),
        &state_key,
        &associated_token_program,
        &user_token_a_key,
        // &[&payer.pubkey(), &mint_key, &vault_key, &state_key],
    );

    println!("after create_vault_instruction");

    let mut transaction =
        Transaction::new_with_payer(&[create_vault_instruction], Some(&payer.pubkey()));
    println!("Transaction created for vault instruction...");

    // Debugging: Print account owners
    println!("Fetching payer account...");
    let payer_account = banks_client.get_account(payer.pubkey()).await?.unwrap();
    println!("Payer account owner: {:?}", payer_account.owner);

    assert!(payer_account.owner == solana_program::system_program::id());

    println!("Signing the transaction...");
    transaction.sign(
        // &[&payer, &mint_keypair, &vault_keypair, &state_keypair],
        // &[&payer, &mint_keypair, &vault_keypair],
        // &[&payer, &mint_tokena_keypair, &mint_atokena_keypair],
        &[&payer, &mint_atokena_keypair],
        recent_blockhash,
    );

    // println!("after sign");

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
    let mint_account = banks_client.get_account(mint_atokena_key).await?;
    assert!(mint_account.is_some(), "Mint account not created");
    let vault_account = banks_client.get_account(vault_key).await?;
    assert!(vault_account.is_some(), "Vault account not created");

    // assertions
    let state_account = banks_client.get_account(state_key).await?;
    assert!(state_account.is_some(), "State account not found");
    let state_data = state_account.unwrap().data;

    // Deserialize the VaultRegistry
    let vault_registry =
        VaultRegistry::deserialize(&state_data).expect("Failed to deserialize VaultRegistry");

    // Ensure the vault registry contains two vaults
    assert_eq!(
        vault_registry.vaults.len(),
        1,
        "Vault registry should contain exactly one vault"
    );

    let first_vault = &vault_registry.vaults[0];
    assert_eq!(
        first_vault.vault_account, vault_key,
        "First vault account mismatch"
    );
    assert_eq!(
        first_vault.mint_token_a, mint_tokena_key,
        "First mint_token_a mismatch"
    );
    assert_eq!(
        first_vault.mint_a_token_a, mint_atokena_key,
        "First mint_a_token_a mismatch"
    );
    assert_eq!(
        first_vault.owner,
        payer.pubkey(),
        "First vault owner mismatch"
    );

    println!("Test completed successfully.");
    Ok(())
}

#[tokio::test]
async fn test_create_two_vaults() -> Result<(), TransportError> {
    // Setup keys and program
    let program_id = Pubkey::new_unique();

    let rent_key = solana_program::sysvar::rent::ID;
    let spl_key = spl_token::id();
    let associated_token_program = spl_associated_token_account::id();

    let mut program_test = ProgramTest::new(
        "rugsafe_vaults",
        program_id,
        processor!(process_instruction),
    );

    // Derive the state account PDA
    let (state_account_pda, _bump_seed) =
        Pubkey::find_program_address(&[b"vault_registry"], &program_id);

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
    let rent_account = banks_client.get_account(rent_key).await?;
    assert!(rent_account.is_some(), "Rent account not found");

    // Create the first token mint (Token A)
    let mint_tokena_keypair1 =
        create_token_mint(banks_client, &payer, recent_blockhash, &payer.pubkey()).await?;
    let mint_tokena_key1 = mint_tokena_keypair1.pubkey();

    // Create a new keypair for AToken A
    let mint_atokena_keypair1 = Keypair::new();
    let mint_atokena_key1 = mint_atokena_keypair1.pubkey();

    // Derive the associated token account for the first vault
    let vault_key1: Pubkey = get_associated_token_address(&program_id, &mint_tokena_key1);
    let user_token_a: Pubkey = get_associated_token_address(&payer.pubkey(), &mint_tokena_key1);

    // Create the first vault instruction
    println!("Creating first vault instruction...");
    let create_vault_instruction1 = create_vault_instruction(
        &program_id,
        &vault_key1,
        &mint_tokena_key1,  // Token A mint
        &mint_atokena_key1, // AToken A mint
        &payer.pubkey(),
        &state_account_pda,
        &associated_token_program,
        &user_token_a,
    );

    let mut transaction1 =
        Transaction::new_with_payer(&[create_vault_instruction1], Some(&payer.pubkey()));
    transaction1.sign(&[&payer, &mint_atokena_keypair1], recent_blockhash);
    banks_client.process_transaction(transaction1).await?;

    // Verify first vault creation
    let vault_account1 = banks_client.get_account(vault_key1).await?;
    assert!(vault_account1.is_some(), "First vault account not created");

    // Create the second token mint (Token A for the second vault)
    let mint_tokena_keypair2 =
        create_token_mint(banks_client, &payer, recent_blockhash, &payer.pubkey()).await?;
    let mint_tokena_key2 = mint_tokena_keypair2.pubkey();

    // Create a new keypair for AToken A for the second vault
    let mint_atokena_keypair2 = Keypair::new();
    let mint_atokena_key2 = mint_atokena_keypair2.pubkey();

    // Derive the associated token account for the second vault
    let vault_key2: Pubkey = get_associated_token_address(&program_id, &mint_tokena_key2);

    // Create the second vault instruction
    println!("Creating second vault instruction...");
    let create_vault_instruction2 = create_vault_instruction(
        &program_id,
        &vault_key2,
        &mint_tokena_key2,  // Token A mint
        &mint_atokena_key2, // AToken A mint
        &payer.pubkey(),
        &state_account_pda,
        &associated_token_program,
        &user_token_a,
    );

    let mut transaction2 =
        Transaction::new_with_payer(&[create_vault_instruction2], Some(&payer.pubkey()));
    transaction2.sign(&[&payer, &mint_atokena_keypair2], recent_blockhash);
    banks_client.process_transaction(transaction2).await?;

    // Verify second vault creation
    let vault_account2 = banks_client.get_account(vault_key2).await?;
    assert!(vault_account2.is_some(), "Second vault account not created");

    //assertions

    // Fetch and verify the state account (VaultRegistry)
    let state_account = banks_client.get_account(state_account_pda).await?;
    assert!(state_account.is_some(), "State account not found");
    let state_data = state_account.unwrap().data;

    // Deserialize the VaultRegistry
    let vault_registry =
        VaultRegistry::deserialize(&state_data).expect("Failed to deserialize VaultRegistry");

    // Ensure the vault registry contains two vaults
    assert_eq!(
        vault_registry.vaults.len(),
        2,
        "Vault registry should contain exactly two vaults"
    );

    // Verify the first vault's details
    let first_vault = &vault_registry.vaults[0];
    assert_eq!(
        first_vault.vault_account, vault_key1,
        "First vault account mismatch"
    );
    assert_eq!(
        first_vault.mint_token_a, mint_tokena_key1,
        "First mint_token_a mismatch"
    );
    assert_eq!(
        first_vault.mint_a_token_a, mint_atokena_key1,
        "First mint_a_token_a mismatch"
    );
    assert_eq!(
        first_vault.owner,
        payer.pubkey(),
        "First vault owner mismatch"
    );

    // Verify the second vault's details
    let second_vault = &vault_registry.vaults[1];
    assert_eq!(
        second_vault.vault_account, vault_key2,
        "Second vault account mismatch"
    );
    assert_eq!(
        second_vault.mint_token_a, mint_tokena_key2,
        "Second mint_token_a mismatch"
    );
    assert_eq!(
        second_vault.mint_a_token_a, mint_atokena_key2,
        "Second mint_a_token_a mismatch"
    );
    assert_eq!(
        second_vault.owner,
        payer.pubkey(),
        "Second vault owner mismatch"
    );

    println!("Both vaults created successfully.");
    Ok(())
}

#[tokio::test]
async fn test_deposit() -> Result<(), BanksClientError> {
    println!("Starting test_deposit");

    let program_id = Pubkey::new_unique();
    let rent_key = solana_program::sysvar::rent::ID;
    let spl_key = spl_token::id();
    let associated_token_program = spl_associated_token_account::id();

    let mut program_test = ProgramTest::new(
        "rugsafe_vaults",
        program_id,
        processor!(process_instruction),
    );

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
    println!("Recent blockhash: {:?}", recent_blockhash);

    let rent = banks_client.get_rent().await?;
    let required_lamports = rent.minimum_balance(Mint::LEN);
    println!("Required lamports: {}", required_lamports);

    // Step 1: Create TokenA mint
    let mint_tokena_keypair =
        create_token_mint(banks_client, &payer, recent_blockhash, &payer.pubkey()).await?;
    let mint_tokena_key = mint_tokena_keypair.pubkey();

    // NOTE: generate the address for the ATokenA mint
    let mint_atokena_keypair = Keypair::new(); // Mint account for TokenA
    let mint_atokena_key = mint_atokena_keypair.pubkey();

    println!("mint_atokena_key: {}", mint_atokena_key);

    // Step 2: Derive the associated token account for the vault
    let vault_key: Pubkey = get_associated_token_address(&program_id, &mint_tokena_key);
    let user_token_a_account: Pubkey =
        get_associated_token_address(&payer.pubkey(), &mint_tokena_key);

    // Step 3: Create the vault using the create_vault functionality
    println!("Creating vault...");
    let (state_key, _bump_seed) = Pubkey::find_program_address(&[b"vault_registry"], &program_id);

    let create_vault_instruction = create_vault_instruction(
        &program_id,
        &vault_key,
        &mint_tokena_key,
        &mint_atokena_key,
        &payer.pubkey(),
        &state_key,
        &associated_token_program,
        &user_token_a_account,
    );
    let transaction = Transaction::new_signed_with_payer(
        &[create_vault_instruction],
        Some(&payer.pubkey()),
        // &[&payer, &mint_tokena_keypair, &mint_atokena_keypair],
        // &[&payer, &mint_tokena_keypair],
        &[&payer, &mint_atokena_keypair],
        recent_blockhash,
    );
    banks_client.process_transaction(transaction).await?;
    println!("Vault created.");

    // Step 4: Create and initialize user TokenA account
    println!("Creating and initializing user TokenA account...");
    let user_token_a_account = Keypair::new();
    let create_user_token_a_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &user_token_a_account.pubkey(),
        rent.minimum_balance(TokenAccount::LEN),
        TokenAccount::LEN as u64,
        &spl_token::id(),
    );
    let initialize_user_token_a_account_ix = spl_token::instruction::initialize_account(
        &spl_token::id(),
        &user_token_a_account.pubkey(),
        &mint_tokena_key,
        &payer.pubkey(),
    )
    .map_err(program_error_to_banks_client_error)?;

    let transaction = Transaction::new_signed_with_payer(
        &[
            create_user_token_a_account_ix,
            initialize_user_token_a_account_ix,
        ],
        Some(&payer.pubkey()),
        &[payer, &user_token_a_account],
        recent_blockhash,
    );
    banks_client.process_transaction(transaction).await?;
    println!("User TokenA account created and initialized.");

    // Step 5: Mint TokenA to the user's account
    println!("Minting TokenA to the user's account...");
    let mint_to_user_token_a_ix = spl_token::instruction::mint_to(
        &spl_token::id(),
        &mint_tokena_key,
        &user_token_a_account.pubkey(),
        &payer.pubkey(),
        &[],
        101,
    )
    .map_err(program_error_to_banks_client_error)?;
    let transaction = Transaction::new_signed_with_payer(
        &[mint_to_user_token_a_ix],
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhash,
    );
    banks_client.process_transaction(transaction).await?;
    println!("TokenA minted to user's account.");

    // Step 5.1: Derive and create the associated token account for ATokenA
    println!("Creating associated token account for ATokenA...");
    let user_atoken_a_account = get_associated_token_address(&payer.pubkey(), &mint_atokena_key);
    println!("user_atoken_a_account: {}", user_atoken_a_account);

    // println!("Associated token account for ATokenA created.");

    // Step 6: Construct and send the deposit instruction
    println!("Constructing deposit instruction...");
    let deposit_amount: u64 = 101;
    let mut deposit_instruction_data = vec![0, 1]; // Instruction ID for "Deposit"
    deposit_instruction_data.extend_from_slice(&deposit_amount.to_le_bytes());

    let deposit_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),    // Payer Account
            AccountMeta::new(mint_tokena_key, false),  // TokenA Mint
            AccountMeta::new(mint_atokena_key, false), // ATokenA Mint
            AccountMeta::new(vault_key, false),        // Vault Account
            AccountMeta::new(user_token_a_account.pubkey(), false), // User's TokenA Account
            AccountMeta::new(user_atoken_a_account, false), // User's ATokenA Account
            AccountMeta::new_readonly(rent_key, false), // Rent Sysvar Account
            AccountMeta::new_readonly(spl_key, false), // SPL Token Program Account
            AccountMeta::new_readonly(solana_program::system_program::id(), false), // System Program Account
            AccountMeta::new(associated_token_program, false),                      //was true
        ],
        data: deposit_instruction_data,
    };
    println!("Deposit instruction constructed.");

    let mut transaction =
        Transaction::new_with_payer(&[deposit_instruction], Some(&payer.pubkey()));
    transaction.sign(&[&payer], recent_blockhash);
    println!("Deposit transaction signed.");

    banks_client.process_transaction(transaction).await?;
    println!("Deposit transaction processed.");

    // Step 7: Check balances and assert expected outcomes
    println!("Checking account balances...");
    let user_token_a_account_info = banks_client
        .get_account(user_token_a_account.pubkey())
        .await?
        .unwrap();
    let vault_token_a_account_info = banks_client.get_account(vault_key).await?.unwrap();
    let user_a_token_a_account_info = banks_client.get_account(vault_key).await?.unwrap();

    let user_token_a_balance = TokenAccount::unpack(&user_token_a_account_info.data)
        .map_err(program_error_to_banks_client_error)?
        .amount;
    let vault_token_a_balance = TokenAccount::unpack(&vault_token_a_account_info.data)
        .map_err(program_error_to_banks_client_error)?
        .amount;
    let user_a_token_a_balance = TokenAccount::unpack(&user_a_token_a_account_info.data)
        .map_err(program_error_to_banks_client_error)?
        .amount;

    println!("User TokenA balance: {}", user_token_a_balance);
    println!("Vault TokenA balance: {}", vault_token_a_balance);
    println!("User aTokenA balance: {}", user_a_token_a_balance);

    assert_eq!(user_token_a_balance, 0); // User should have 0 TokenA
    assert_eq!(vault_token_a_balance, deposit_amount); // Vault should have 101 TokenA
    assert_eq!(user_a_token_a_balance, deposit_amount); // User should have 101 aTokenA

    // println!("Test passed successfully.");
    Ok(())
}

fn create_vault_instruction(
    program_id: &Pubkey,
    vault_key: &Pubkey,
    mint_key_token_a: &Pubkey,   // Mint A for the incoming tokens
    mint_key_a_token_a: &Pubkey, // Mint B for the aTokens
    payer: &Pubkey,
    state: &Pubkey,
    associated_token: &Pubkey,
    user_token_a: &Pubkey,
    // signer_keys: &[&Pubkey],
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*payer, true),
        AccountMeta::new(*mint_key_token_a, false),
        AccountMeta::new(*mint_key_a_token_a, true),
        AccountMeta::new(*vault_key, false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
        AccountMeta::new(*state, false),            //was true
        AccountMeta::new(*associated_token, false), //was true
        AccountMeta::new(*user_token_a, false),
        AccountMeta::new(*program_id, false),
    ];
    println!("Create Vault program_id: {:?}", program_id);

    Instruction {
        program_id: *program_id,
        accounts,
        data: vec![0, 0], // Add any additional data if needed
    }
}

async fn create_token_mint(
    banks_client: &mut BanksClient,
    payer: &Keypair,
    recent_blockhash: Hash,
    mint_authority: &Pubkey,
) -> Result<Keypair, BanksClientError> {
    let mint_keypair = Keypair::new();
    let mint_rent = banks_client
        .get_rent()
        .await?
        .minimum_balance(spl_token::state::Mint::LEN);

    let create_mint_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &mint_keypair.pubkey(),
        mint_rent,
        spl_token::state::Mint::LEN as u64,
        &spl_token::id(),
    );

    let initialize_mint_ix = spl_token::instruction::initialize_mint(
        &spl_token::id(),
        &mint_keypair.pubkey(),
        mint_authority,
        None,
        0,
    )
    .map_err(|e| {
        BanksClientError::ClientError(Box::leak(Box::new(format!(
            "Failed to create initialize_mint instruction: {}",
            e
        ))))
    })?;

    let transaction = Transaction::new_signed_with_payer(
        &[create_mint_account_ix, initialize_mint_ix],
        Some(&payer.pubkey()),
        &[payer, &mint_keypair],
        recent_blockhash,
    );

    banks_client.process_transaction(transaction).await?;

    Ok(mint_keypair)
}

// Log serialized data at the bit level
fn log_bits(bytes: &[u8]) {
    for (i, byte) in bytes.iter().enumerate() {
        println!("Byte {}: {:#010b}", i, byte);
    }
}
fn manual_deserialize(state_data: &[u8]) -> Vec<Vault> {
    let mut rest = state_data;

    // First 4 bytes: length of the vaults vector
    let (vaults_len_bytes, next) = rest.split_at(4);
    let vaults_len = u32::from_le_bytes(vaults_len_bytes.try_into().unwrap());
    println!("Vaults length (u32): {:?}", vaults_len);
    rest = next;

    // Each Vault is 160 bytes (5 Pubkeys of 32 bytes each)
    let mut vaults = Vec::new();
    for i in 0..vaults_len {
        let (vault_bytes, next) = rest.split_at(Vault::LEN);
        rest = next;

        // Refactoring the Vault deserialization into its own function
        let vault = deserialize_vault(vault_bytes);

        println!("Vault {}: {:?}", i, vault);
        vaults.push(vault);
    }

    println!("Manually deserialized vaults: {:?}", vaults);

    // Return the deserialized vaults
    vaults
}

// Refactored vault deserialization function
// Refactored vault deserialization function
fn deserialize_vault(vault_bytes: &[u8]) -> Vault {
    let (vault_account_bytes, vault_bytes) = vault_bytes.split_at(32);
    let vault_account = Pubkey::new_from_array(vault_account_bytes.try_into().unwrap());

    let (mint_account_bytes, vault_bytes) = vault_bytes.split_at(32);
    let mint_account = Pubkey::new_from_array(mint_account_bytes.try_into().unwrap());

    let (mint_atoken_account_bytes, vault_bytes) = vault_bytes.split_at(32);
    let mint_atoken_account = Pubkey::new_from_array(mint_atoken_account_bytes.try_into().unwrap());

    let (owner_bytes, _vault_bytes) = vault_bytes.split_at(32);
    let owner = Pubkey::new_from_array(owner_bytes.try_into().unwrap());

    Vault {
        vault_account: vault_account,
        mint_token_a: mint_account,
        mint_a_token_a: mint_atoken_account, // Include the new field
        owner: owner,
    }
}
#[tokio::test]
async fn test_fetch_vault_from_registry() -> Result<(), TransportError> {
    // Setup keys and program
    let program_id = Pubkey::new_unique();
    let mint_tokena_keypair = Keypair::new(); // Mint account for token A
    let mint_tokena_key = mint_tokena_keypair.pubkey();
    let mint_atokena_keypair = Keypair::new(); // Mint account for aToken A
    let mint_atokena_key = mint_atokena_keypair.pubkey();

    let rent_key = solana_program::sysvar::rent::ID;
    let spl_key = spl_token::id();
    let associated_token_program = spl_associated_token_account::id();

    let (state_key, _bump_seed) = Pubkey::find_program_address(&[b"vault_registry"], &program_id);

    let mut program_test = ProgramTest::new(
        "rugsafe_vaults",
        program_id,
        processor!(process_instruction),
    );

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

    // Create token mint for token A
    let mint_tokena_keypair =
        create_token_mint(banks_client, &payer, recent_blockhash, &payer.pubkey()).await?;
    let mint_tokena_key = mint_tokena_keypair.pubkey();

    // Vault key should be associated token account for token A's mint
    let vault_key: Pubkey = get_associated_token_address(&program_id, &mint_tokena_key);
    let user_token_a_account: Pubkey =
        get_associated_token_address(&payer.pubkey(), &mint_tokena_key);

    // Call the function to create the vault instruction
    println!("Creating vault instruction...");
    let create_vault_instruction = create_vault_instruction(
        &program_id,
        &vault_key,
        &mint_tokena_key,
        &mint_atokena_key,
        &payer.pubkey(),
        &state_key,
        &associated_token_program,
        &user_token_a_account,
    );

    let mut transaction =
        Transaction::new_with_payer(&[create_vault_instruction], Some(&payer.pubkey()));

    println!("Signing the transaction...");
    transaction.sign(
        // &[&payer, &mint_tokena_keypair, &mint_atokena_keypair],
        &[&payer, &mint_atokena_keypair],
        recent_blockhash,
    );

    // Process CreateVault transaction
    println!("Processing CreateVault transaction...");
    banks_client.process_transaction(transaction).await?;

    // Verify vault creation
    println!("Verifying vault creation...");
    let mint_account = banks_client.get_account(mint_atokena_key).await?;
    assert!(mint_account.is_some(), "Mint account not created");
    let vault_account = banks_client.get_account(vault_key).await?;
    assert!(vault_account.is_some(), "Vault account not created");

    // Fetch and verify the vault registry
    println!("Fetching and verifying the vault registry...");
    let state_account = banks_client.get_account(state_key).await?.unwrap();
    let state_data = state_account.data;

    let vault_registry =
        VaultRegistry::deserialize(&state_data).expect("Failed to deserialize VaultRegistry");

    println!("VaultRegistry contents: {:?}", vault_registry);
    assert!(
        vault_registry
            .vaults
            .iter()
            .any(|v| v.vault_account == vault_key),
        "Vault not found in registry"
    );

    Ok(())
}
#[tokio::test]
async fn test_fetch_vault_with_data_from_registry() -> Result<(), TransportError> {
    // Setup keys and program
    let program_id = Pubkey::new_unique();
    let mint_tokena_keypair = Keypair::new(); // Mint account for token A
    let mint_tokena_key = mint_tokena_keypair.pubkey();
    let mint_atokena_keypair = Keypair::new(); // Mint account for aToken A
    let mint_atokena_key = mint_atokena_keypair.pubkey();

    let rent_key = solana_program::sysvar::rent::ID;
    let spl_key = spl_token::id();
    let associated_token_program = spl_associated_token_account::id();

    let (state_key, _bump_seed) = Pubkey::find_program_address(&[b"vault_registry"], &program_id);

    let mut program_test = ProgramTest::new(
        "rugsafe_vaults",
        program_id,
        processor!(process_instruction),
    );

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

    // Create token mint for token A
    let mint_tokena_keypair =
        create_token_mint(banks_client, &payer, recent_blockhash, &payer.pubkey()).await?;
    let mint_tokena_key = mint_tokena_keypair.pubkey();

    // Vault key should be associated token account for token A's mint
    let vault_key: Pubkey = get_associated_token_address(&program_id, &mint_tokena_key);
    let user_token_a_account: Pubkey =
        get_associated_token_address(&payer.pubkey(), &mint_tokena_key);

    // Call the function to create the vault instruction
    println!("Creating vault instruction...");
    let create_vault_instruction = create_vault_instruction(
        &program_id,
        &vault_key,
        &mint_tokena_key,
        &mint_atokena_key,
        &payer.pubkey(),
        &state_key,
        &associated_token_program,
        &user_token_a_account,
    );

    let mut transaction =
        Transaction::new_with_payer(&[create_vault_instruction], Some(&payer.pubkey()));

    println!("Signing the transaction...");
    transaction.sign(
        // &[&payer, &mint_tokena_keypair, &mint_atokena_keypair],
        &[&payer, &mint_atokena_keypair],
        recent_blockhash,
    );

    // Process CreateVault transaction
    println!("Processing CreateVault transaction...");
    banks_client.process_transaction(transaction).await?;

    // Verify vault creation
    println!("Verifying vault creation...");
    let mint_account = banks_client.get_account(mint_atokena_key).await?;
    assert!(mint_account.is_some(), "Mint account not created");
    let vault_account = banks_client.get_account(vault_key).await?;
    assert!(vault_account.is_some(), "Vault account not created");

    // Fetch and verify the vault registry with actual data
    println!("Fetching and verifying the vault registry with actual data...");
    let state_account = banks_client.get_account(state_key).await?.unwrap();
    let state_data = state_account.data;

    println!("State account data length: {}", state_data.len());
    println!("First 32 bytes of state data: {:?}", &state_data[..32]);

    // println!(
    //     "Expected size of VaultRegistry struct: {}",
    //     VaultRegistry::LEN
    // );
    println!(
        "Actual size of data to be deserialized: {}",
        state_data.len()
    );

    manual_deserialize(&state_data);

    let vault_registry =
        VaultRegistry::deserialize(&state_data).expect("Failed to deserialize VaultRegistry");

    println!("VaultRegistry contents: {:?}", vault_registry);
    assert!(
        vault_registry
            .vaults
            .iter()
            .any(|v| v.vault_account == vault_key),
        "Vault not found in registry"
    );

    Ok(())
}
////////////////////////////
//////////////////////////////
#[tokio::test]
async fn test_faucet() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting test_faucet");

    let program_id = Pubkey::new_unique();
    let mut program_test =
        ProgramTest::new("rugsafe_vaults", program_id, processor!(Processor::process));

    // Add SPL Token program
    program_test.add_program(
        "spl_token",
        spl_token::id(),
        processor!(spl_token::processor::Processor::process),
    );

    println!("Starting program test context...");
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    ///////////////////////////////
    // Derive mint PDA as expected by the backend
    let (mint_pubkey, _bump_seed) = Pubkey::find_program_address(&[b"mint"], &program_id);
    // let mint_keypair = Keypair::new(); // Mint account
    // let mint_pubkey = mint_keypair.pubkey();

    //////////////////////////////
    // Create user token account keypair
    // let user_token_keypair: Keypair = Keypair::new();
    let user_token_account = get_associated_token_address(&payer.pubkey(), &mint_pubkey);

    let associated_token_program: Pubkey = spl_associated_token_account::id();

    // Prepare the faucet instruction
    let amount: u64 = 1000;
    let mut data = vec![0, 4]; // Instruction ID
    data.extend_from_slice(&amount.to_le_bytes()); // Append the amount as an 8-byte little-endian value

    let faucet_ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            //
            // AccountMeta::new(user_token_keypair.pubkey(), true),
            AccountMeta::new(user_token_account, false),
            //
            AccountMeta::new(mint_pubkey, false), // Use derived mint pubkey
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(solana_program::sysvar::rent::ID, false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
            AccountMeta::new(spl_associated_token_account::id(), false),
        ],
        data, // Construct data manually here
    };

    // Create and send the faucet transaction
    let faucet_tx = Transaction::new_signed_with_payer(
        &[faucet_ix],
        Some(&payer.pubkey()),
        // &[&payer, &mint_keypair, &user_token_keypair],
        // &[&payer, mint_pubkey],
        // &[&payer],
        // &[&payer, &user_token_keypair], // No mint keypair, backend handles mint creation
        &[&payer],
        recent_blockhash,
    );
    banks_client.process_transaction(faucet_tx).await?;

    // Verify the user token account balance
    println!("Verifying user token account balance...");
    let user_token_account = banks_client
        // .get_account(user_token_keypair.pubkey())
        .get_account(user_token_account)
        .await?
        .expect("user_token_account not found");

    let token_account_data = TokenAccount::unpack(&user_token_account.data)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    assert_eq!(token_account_data.amount, 1000, "Incorrect token balance");

    println!("Test completed successfully.");
    Ok(())
}
