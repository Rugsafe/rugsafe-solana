use borsh::de::BorshDeserialize;
use hello_world::process_instruction;
use hello_world::state::{Vault, VaultRegistry};
use hex;
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
use spl_token::instruction::initialize_mint;
use spl_token::state::Account as TokenAccount;
use spl_token::state::Mint;

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

    let state_keypair = Keypair::new(); // State account
    let state_key = state_keypair.pubkey();

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
    let rent_account = banks_client.get_account(rent_key).await?;
    assert!(rent_account.is_some(), "Rent account not found");

    // Call the function to create the vault instruction
    println!("Creating vault instruction...");
    let create_vault_instruction = create_vault_instruction(
        &program_id,
        &vault_key,
        &mint_key,
        &payer.pubkey(),
        &state_key,
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
        &[&payer, &mint_keypair, &vault_keypair, &state_keypair],
        recent_blockhash,
    );

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
}
#[tokio::test]
async fn test_deposit() -> Result<(), BanksClientError> {
    println!("Starting test_deposit");

    let program_id = Pubkey::new_unique();
    let token_a_mint_keypair = Keypair::new(); // Mint account for TokenA
    let token_a_mint_key = token_a_mint_keypair.pubkey();

    let rent_key = solana_program::sysvar::rent::ID;
    let spl_key = spl_token::id();

    let state_keypair = Keypair::new(); // State account
    let state_key = state_keypair.pubkey();

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
    println!("Recent blockhash: {:?}", recent_blockhash);

    let rent = banks_client.get_rent().await?;
    let required_lamports = rent.minimum_balance(Mint::LEN);
    println!("Required lamports: {}", required_lamports);

    // Step 3: Create the vault using the create_vault functionality
    println!("Creating vault...");
    let vault_keypair = Keypair::new();
    let vault_key = vault_keypair.pubkey();

    let create_vault_instruction = create_vault_instruction(
        &program_id,
        &vault_key,
        &token_a_mint_key,
        &payer.pubkey(),
        &state_key,
        // &[&payer.pubkey(), &token_a_mint_key, &vault_key, &state_key],
    );
    let transaction = Transaction::new_signed_with_payer(
        &[create_vault_instruction],
        Some(&payer.pubkey()),
        &[
            &payer,
            &token_a_mint_keypair,
            &vault_keypair,
            &state_keypair,
        ],
        recent_blockhash,
    );
    banks_client.process_transaction(transaction).await?;
    println!("Vault created.");

    // // Step 4: Create user TokenA account
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
    println!("User TokenA account created.");

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
    println!("User TokenA account initialized.");

    // Step 5: Create user aTokenA account
    println!("Creating user aTokenA account...");
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
    println!("User aTokenA account created.");

    let initialize_user_a_token_a_account_ix = spl_token::instruction::initialize_account(
        &spl_token::id(),
        &user_a_token_a_account.pubkey(),
        &token_a_mint_key,
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
    println!("User aTokenA account initialized.");

    // Step 6: Mint TokenA to the user's account
    println!("Minting TokenA to the user's account...");
    let mint_to_user_token_a_ix = spl_token::instruction::mint_to(
        &spl_token::id(),
        &token_a_mint_key,
        &user_token_a_account.pubkey(),
        &payer.pubkey(),
        &[],
        101, // Mint 100 TokenA for testing
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

    // // Step 7: Construct and send the deposit instruction
    println!("Constructing deposit instruction...");
    let deposit_amount: u64 = 101;
    let mut deposit_instruction_data = vec![1]; // Instruction ID for "Deposit"
    deposit_instruction_data.extend_from_slice(&deposit_amount.to_le_bytes());

    let deposit_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true), // Payer Account
            AccountMeta::new(token_a_mint_key, false),
            AccountMeta::new(vault_key, false), // Vault Account
            AccountMeta::new(user_token_a_account.pubkey(), false), // User's TokenA Account - true
            // end user account
            AccountMeta::new(user_a_token_a_account.pubkey(), false),
            AccountMeta::new_readonly(rent_key, false), // Rent Sysvar Account
            AccountMeta::new_readonly(spl_key, false),  // SPL Token Program Account
            AccountMeta::new_readonly(solana_program::system_program::id(), false), // System Program Account
        ],
        data: deposit_instruction_data.to_vec(), // Correctly serialized data
    };
    println!("Deposit instruction constructed.");

    let mut transaction =
        Transaction::new_with_payer(&[deposit_instruction], Some(&payer.pubkey()));
    // transaction.sign(&[&payer, &user_token_a_account], recent_blockhash);
    transaction.sign(&[&payer], recent_blockhash);
    println!("Deposit transaction signed.");

    banks_client.process_transaction(transaction).await?;
    println!("Deposit transaction processed.");

    // Step 8: Check balances and assert expected outcomes
    println!("Checking account balances...");
    let user_token_a_account_info = banks_client
        .get_account(user_token_a_account.pubkey())
        .await?
        .unwrap();
    let vault_token_a_account_info = banks_client.get_account(vault_key).await?.unwrap();
    let user_a_token_a_account_info = banks_client
        .get_account(user_a_token_a_account.pubkey())
        .await?
        .unwrap();

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
    assert_eq!(vault_token_a_balance, deposit_amount); // Vault should have 100 TokenA
    assert_eq!(user_a_token_a_balance, deposit_amount); // User should have 100 aTokenA

    println!("Test passed successfully.");
    Ok(())
}

fn create_vault_instruction(
    program_id: &Pubkey,
    vault_key: &Pubkey,
    mint_key: &Pubkey,
    payer: &Pubkey,
    state: &Pubkey,
    // signer_keys: &[&Pubkey],
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*payer, true),
        AccountMeta::new(*mint_key, true),
        AccountMeta::new(*vault_key, true),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
        AccountMeta::new(*state, true),
    ];

    Instruction {
        program_id: *program_id,
        accounts,
        data: vec![0], // Add any additional data if needed
    }
}
///////////////////////
/// /////////////////////
/// //////////////////////
///
///
///
// #[tokio::test]
// async fn test_fetch_vault_from_registry() -> Result<(), TransportError> {
//     // Setup the context and accounts
//     let program_id = Pubkey::new_unique();
//     let program_test = ProgramTest::new("hello_world", program_id, processor!(process_instruction));

//     println!("Starting context...");
//     let mut context = program_test.start_with_context().await;
//     let banks_client = &mut context.banks_client;
//     let payer = &context.payer;
//     let recent_blockhash = banks_client.get_latest_blockhash().await?;
//     println!("Context started. Recent blockhash: {:?}", recent_blockhash);

//     // Get the rent minimum balance
//     let rent = solana_program::sysvar::rent::Rent::default();

//     let vault_rent_lamports = rent.minimum_balance(0);
//     let state_rent_lamports = rent.minimum_balance(VaultRegistry::LEN);
//     let mint_rent_lamports = rent.minimum_balance(Mint::LEN);

//     println!("Rent-exempt balances calculated:");
//     println!("vault_rent_lamports: {}", vault_rent_lamports);
//     println!("state_rent_lamports: {}", state_rent_lamports);
//     println!("mint_rent_lamports: {}", mint_rent_lamports);

//     // Create necessary accounts
//     let vault_keypair = Keypair::new();
//     let state_keypair = Keypair::new();
//     let mint_keypair = Keypair::new();

//     println!("Keypairs generated:");
//     println!("vault_keypair: {:?}", vault_keypair.pubkey());
//     println!("state_keypair: {:?}", state_keypair.pubkey());
//     println!("mint_keypair: {:?}", mint_keypair.pubkey());

//     // Ensure accounts are properly created
//     let mut create_accounts_transaction = Transaction::new_with_payer(
//         &[
//             system_instruction::create_account(
//                 &payer.pubkey(),
//                 &vault_keypair.pubkey(),
//                 vault_rent_lamports,
//                 0,
//                 &program_id,
//             ),
//             system_instruction::create_account(
//                 &payer.pubkey(),
//                 &state_keypair.pubkey(),
//                 state_rent_lamports,
//                 VaultRegistry::LEN as u64,
//                 &program_id,
//             ),
//             system_instruction::create_account(
//                 &payer.pubkey(),
//                 &mint_keypair.pubkey(),
//                 mint_rent_lamports,
//                 Mint::LEN as u64,
//                 &spl_token::id(),
//             ),
//         ],
//         Some(&payer.pubkey()),
//     );
//     create_accounts_transaction.sign(
//         &[&payer, &vault_keypair, &state_keypair, &mint_keypair],
//         recent_blockhash,
//     );

//     println!("Processing create accounts transaction...");
//     banks_client
//         .process_transaction(create_accounts_transaction)
//         .await
//         .map_err(|err| {
//             println!("Failed to process create accounts transaction: {:?}", err);
//             err
//         })?;

//     println!("Accounts created successfully.");

//     // Run the test to create the vault
//     let create_vault_instruction = create_vault_instruction(
//         &program_id,
//         &vault_keypair.pubkey(),
//         &mint_keypair.pubkey(),
//         &payer.pubkey(),
//         &state_keypair.pubkey(),
//     );

//     println!("Creating vault transaction...");
//     let mut transaction =
//         Transaction::new_with_payer(&[create_vault_instruction], Some(&payer.pubkey()));
//     transaction.sign(
//         &[&payer, &vault_keypair, &mint_keypair, &state_keypair],
//         recent_blockhash,
//     );

//     println!("Processing create vault transaction...");
//     banks_client
//         .process_transaction(transaction)
//         .await
//         .map_err(|err| {
//             println!("Failed to process create vault transaction: {:?}", err);
//             err
//         })?;

//     println!("Vault created successfully.");

//     // Fetch and verify the state account
//     println!("Fetching state account...");
//     let state_account = banks_client.get_account(state_keypair.pubkey()).await?;
//     assert!(state_account.is_some(), "State account not created");

//     let state_data = state_account.unwrap().data;
//     let expected_size = VaultRegistry::LEN;
//     assert_eq!(
//         state_data.len(),
//         expected_size,
//         "State account size mismatch"
//     );

//     println!("State account size is correct.");

//     // Deserialize and validate the data
//     match VaultRegistry::try_from_slice(&state_data) {
//         Ok(vault_registry) => {
//             println!(
//                 "VaultRegistry deserialized successfully: {:?}",
//                 vault_registry
//             );
//         }
//         Err(e) => {
//             println!("Deserialization error: {:?}", e);
//             panic!("Failed to deserialize VaultRegistry");
//         }
//     }

//     println!("Test completed successfully.");
//     Ok(())
// }

#[tokio::test]
async fn test_fetch_vault_from_registry() -> Result<(), TransportError> {
    // Setup keys and program
    let program_id = Pubkey::new_unique();
    let mint_keypair = Keypair::new(); // Mint account
    let mint_key = mint_keypair.pubkey();
    let vault_keypair = Keypair::new(); // Vault account
    let vault_key = vault_keypair.pubkey();

    let rent_key = solana_program::sysvar::rent::ID;
    let spl_key = spl_token::id();

    let state_keypair = Keypair::new(); // State account
    let state_key = state_keypair.pubkey();

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
    let rent_account = banks_client.get_account(rent_key).await?;
    assert!(rent_account.is_some(), "Rent account not found");

    // Call the function to create the vault instruction
    println!("Creating vault instruction...");
    let create_vault_instruction = create_vault_instruction(
        &program_id,
        &vault_key,
        &mint_key,
        &payer.pubkey(),
        &state_key,
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
        &[&payer, &mint_keypair, &vault_keypair, &state_keypair],
        recent_blockhash,
    );

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

    ///////////////////////////////

    println!("Fetching and verifying the vault registry...");

    println!("Fetching state account with state_key: {:?}", state_key);
    let state_account = banks_client.get_account(state_key).await?;
    if state_account.is_none() {
        println!(
            "Error: State account not found for state_key: {:?}",
            state_key
        );
        panic!("State account not created");
    }
    println!("State account found!");

    let state_data = state_account.unwrap().data;
    println!("State account data length: {}", state_data.len());
    //from claude
    println!("First 32 bytes of state data: {:?}", &state_data[..32]);

    /////////// work until here

    println!(
        "Expected VaultRegistry size: {}",
        std::mem::size_of::<VaultRegistry>()
    );

    let expected_size = std::mem::size_of::<VaultRegistry>();
    println!("Expected VaultRegistry size: {}", expected_size);

    // Ensure that we're working with the correct slice of data
    let registry_data = &state_data[..expected_size];
    println!(
        "Adjusted state data length for deserialization: {}",
        registry_data.len()
    );

    println!("Attempting to deserialize state account data into VaultRegistry...");
    // let vault_registry: VaultRegistry =
    // match borsh::BorshDeserialize::try_from_slice(&registry_data) {
    //     Ok(vr) => {
    //         println!("VaultRegistry deserialized successfully: {:?}", vr);
    //         vr
    //     }
    //     Err(e) => {
    //         println!("Error deserializing VaultRegistry: {:?}", e);
    //         panic!("Failed to deserialize state account data");
    //     }
    // };

    //claude
    println!("Attempting to deserialize entire state account data...");
    // match VaultRegistry::try_from_slice(&state_data) {
    //     Ok(vr) => {
    //         println!("VaultRegistry deserialized successfully: {:?}", vr);
    //     }
    //     Err(e) => {
    //         println!("Error deserializing full VaultRegistry: {:?}", e);
    //     }
    // }
    //
    // match VaultRegistry::try_from_slice(&state_data) {
    //     Ok(vr) => {
    //         println!("VaultRegistry deserialized successfully:");
    //         println!("Number of vaults: {}", vr.vaults.len());
    //         for (i, vault) in vr.vaults.iter().enumerate() {
    //             println!("Vault {}: {:?}", i, vault);
    //         }
    //     }
    //     Err(e) => {
    //         println!("Error deserializing VaultRegistry: {:?}", e);
    //         // Print the first 64 bytes of the state data for debugging
    //         println!("First 64 bytes of state data: {:?}", &state_data[..64]);
    //     }
    // }

    println!("Attempting to deserialize first 24 bytes of state account data...");
    // match VaultRegistry::try_from_slice(&state_data[..24]) {
    //     Ok(vr) => {
    //         println!(
    //             "VaultRegistry (24 bytes) deserialized successfully: {:?}",
    //             vr
    //         );
    //     }
    //     Err(e) => {
    //         println!("Error deserializing VaultRegistry (24 bytes): {:?}", e);
    //     }
    // }

    // match VaultRegistry::try_from_slice(&state_data) {
    //     Ok(vr) => {
    //         println!("VaultRegistry deserialized successfully:");
    //         println!("Number of vaults: {}", vr.vaults.len());
    //         for (i, vault) in vr.vaults.iter().enumerate() {
    //             println!("Vault {}: {:?}", i, vault);
    //         }
    //     }
    //     Err(e) => {
    //         println!("Error deserializing VaultRegistry: {:?}", e);
    //         println!("First 64 bytes of state data: {:?}", &state_data[..64]);
    //     }
    // }
    println!("State data length: {}", state_data.len());
    println!("First 32 bytes of state data: {:?}", &state_data[..32]);

    match VaultRegistry::try_from_slice(&state_data) {
        Ok(vr) => {
            println!("VaultRegistry deserialized successfully:");
            println!("Number of vaults: {}", vr.vaults.len());
            for (i, vault) in vr.vaults.iter().enumerate() {
                println!("Vault {}: {:?}", i, vault);
            }
        }
        Err(e) => {
            println!("Error deserializing VaultRegistry: {:?}", e);
        }
    }

    Ok(())
}
