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
use rugsafe_perps::instructions::processor::Processor;

use rugsafe_perps::state::perpetuals::{Position, Side, UserPositions};
use spl_associated_token_account::get_associated_token_address;

#[tokio::test]
async fn test_open_position() {
    // Initialize logger if needed
    // solana_logger::setup();

    // Step 1: Initialize the program ID and set up the ProgramTest environment
    // println!("Initializing ProgramTest environment...");
    let program_id = Pubkey::new_unique();
    let mut program_test =
        ProgramTest::new("rugsafe_perps", program_id, processor!(Processor::process));

    // Add the SPL Token program to the test environment
    // println!("Adding SPL Token program...");
    program_test.add_program(
        "spl_token",
        spl_token::id(),
        processor!(spl_token::processor::Processor::process),
    );

    // Start the test context
    // println!("Starting test context...");
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // **Step 2: Create the collateral token mint**
    // println!("Creating collateral token mint...");
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

    // println!("Processing mint transaction...");
    banks_client.process_transaction(transaction).await.unwrap();

    // **Step 3: Create user's collateral token account and mint tokens to it**
    // println!("Creating user's collateral token account and minting tokens...");
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

    // println!("Processing user token minting transaction...");
    banks_client.process_transaction(transaction).await.unwrap();

    // **Step 4: Derive the custody associated token account**
    // println!("Deriving custody associated token account...");
    let custody_account = spl_associated_token_account::get_associated_token_address(
        &program_id,               // Custody is owned by the program
        &collateral_mint.pubkey(), // Associated with collateral mint
    );
    // println!("Custody account: {:?}", custody_account);

    // **Step 5: Derive the user positions PDA**
    // println!("Deriving user positions PDA...");
    let (user_positions_pda, _user_positions_bump) =
        Pubkey::find_program_address(&[b"user_positions", payer.pubkey().as_ref()], &program_id);
    // println!("User positions PDA: {:?}", user_positions_pda);

    // **Step 6: Construct the OpenPosition instruction**
    // println!("Constructing OpenPosition instruction...");
    let side = Side::Long;
    let amount: u64 = 500_000_000; // Amount of collateral to deposit (500 tokens)

    let module_tag = 0; // Replace with the actual module tag for Perpetuals
    let instruction_tag = 0; // Instruction tag for OpenPosition

    let side_byte = match side {
        Side::Long => 1,
        Side::Short => 2,
        _ => panic!("Invalid side"), // Handle any unexpected values
    };

    let mut instruction_data = Vec::with_capacity(11);
    instruction_data.push(module_tag); // Module tag for Perpetuals
    instruction_data.push(instruction_tag); // Instruction tag for OpenPosition
    instruction_data.push(side_byte); // Side byte
    instruction_data.extend_from_slice(&amount.to_le_bytes()); // Amount as u64 in little-endian
    println!("Instruction data: {:?}", instruction_data);

    // **Derive the position PDA based on the next position index, assuming it's 0 for now**
    let (position_pda, _position_bump) = Pubkey::find_program_address(
        &[
            b"position",
            payer.pubkey().as_ref(),
            &0u64.to_le_bytes(), // Assuming next position index is 0 initially
        ],
        &program_id,
    );
    // println!("Position PDA: {:?}", position_pda);

    // **Step 7: Send the OpenPosition transaction**
    let open_position_ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),      // Payer (signer)
            AccountMeta::new(user_positions_pda, false), // UserPositions account (PDA, writable)
            AccountMeta::new(user_collateral_account.pubkey(), false), // User's collateral token account (writable)
            AccountMeta::new(collateral_mint.pubkey(), false), // Collateral mint account (readonly)
            AccountMeta::new(custody_account, false), // Custody associated token account (writable)
            AccountMeta::new_readonly(spl_token::id(), false), // SPL Token Program
            AccountMeta::new_readonly(solana_program::system_program::id(), false), // System program
            AccountMeta::new_readonly(solana_program::sysvar::rent::id(), false),   // Rent sysvar
            AccountMeta::new_readonly(spl_associated_token_account::id(), false), // Associated token program
            AccountMeta::new_readonly(program_id, false),                         // Program account
            AccountMeta::new(position_pda, false), // Add the position PDA here (writable)
        ],
        data: instruction_data,
    };

    let transaction = Transaction::new_signed_with_payer(
        &[open_position_ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    // println!("Processing open position transaction...");
    banks_client.process_transaction(transaction).await.unwrap();

    // **Step 8: Fetch and verify user position details**
    // println!("Fetching UserPositions account data...");
    let user_positions_account_data = banks_client
        .get_account(user_positions_pda)
        .await
        .unwrap()
        .unwrap();

    // Deserialize the user positions data
    // println!(
    //     "user_positions_account_data: {:?}",
    //     user_positions_account_data
    // );
    let mut data_slice: &[u8] = &user_positions_account_data.data;
    let user_positions = UserPositions::deserialize(&mut data_slice).unwrap();

    // Log and assert that the next_position_idx has been incremented
    // println!(
    //     "User positions next index: {:?}",
    //     user_positions.next_position_idx
    // );
    assert_eq!(user_positions.next_position_idx, 1); // Assert that the next_position_idx was incremented

    // **Fetch position data for verification**
    let (position_pda, _) = Pubkey::find_program_address(
        &[
            b"position",
            payer.pubkey().as_ref(),
            &(user_positions.next_position_idx - 1).to_le_bytes(), // Use the last added position
        ],
        &program_id,
    );

    let position_account_data = banks_client
        .get_account(position_pda)
        .await
        .unwrap()
        .unwrap(); // Fetch the individual position account data

    let mut position_data_slice: &[u8] = &position_account_data.data;
    let position = Position::deserialize(&mut position_data_slice).unwrap();

    // Log and assert that a position was added correctly
    // println!("Position: {:?}", position);

    // Assert the fields of the position
    assert_eq!(position.owner, payer.pubkey());
    assert_eq!(position.side, Side::Long);
    assert_eq!(position.size_usd, amount);

    // **Check token balances**
    // println!("Checking token balances...");
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

    // Custody associated token account balance should increase by 'amount'
    let custody_account_data = banks_client
        .get_account(custody_account)
        .await
        .unwrap()
        .unwrap();
    let custody_token_account = TokenAccount::unpack(&custody_account_data.data).unwrap();

    assert_eq!(custody_token_account.amount, 500_000_000);

    // println!("Test passed: Position opened successfully.");
}
