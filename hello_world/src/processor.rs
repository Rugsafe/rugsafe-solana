use crate::instruction::VaultInstruction;
use solana_program::sysvar::Sysvar;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    sysvar,
    sysvar::rent::Rent,
};
use spl_associated_token_account::get_associated_token_address;
// use spl_associated_token_account::instruction::create_associated_token_account;
use spl_associated_token_account::{self, instruction};

// use solana_sdk::program::invoke_signed;

use spl_token::instruction::{burn, initialize_mint, initialize_mint2, mint_to};
use spl_token::state::Account as TokenAccount;
use spl_token::state::Mint;

// storage
use crate::state::{Vault, VaultRegistry};
use borsh::{BorshDeserialize, BorshSerialize};
use std::io::Cursor;

pub struct Processor;

impl Processor {
    /*
    @name process
    @description Entry point for processing all instructions related to the vault. It dispatches the instruction to the appropriate handler.
    @param program_id - The ID of the currently executing program.
    @param accounts - The accounts involved in the transaction.
    @param instruction_data - The instruction data to be processed.
    */

    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = VaultInstruction::unpack(instruction_data)?;

        match instruction {
            VaultInstruction::CreateVault => Self::process_create_vault(program_id, accounts),
            VaultInstruction::Deposit { amount } => {
                Self::process_deposit(program_id, accounts, amount)
            }
            VaultInstruction::Withdraw { amount } => {
                Self::process_withdraw(program_id, accounts, amount)
            }
            VaultInstruction::BurnRToken { amount } => {
                Self::process_burn_rtoken(program_id, accounts, amount)
            }
            VaultInstruction::Faucet { amount } => {
                Self::process_faucet(program_id, accounts, amount)
            }
        }
    }
    /*
    @name process_create_vault
    @description Handles the creation of a new vault, including initializing the mint and vault accounts, and setting up the state account for the vault registry.
    @param program_id - The ID of the currently executing program.
    @param accounts - The accounts involved in the transaction.
    @ note - creates a MINT, a VAULT, and a STATE account is they arent already made
    */
    fn process_create_vault(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let payer_account = next_account_info(account_info_iter)?;
        // mint for anticoins
        // let mint_account = next_account_info(account_info_iter)?;
        let mint_account_token_a = next_account_info(account_info_iter)?;
        let mint_account_a_token_a = next_account_info(account_info_iter)?;
        // the account that will hold Token A
        let vault_account = next_account_info(account_info_iter)?;
        let rent_account = next_account_info(account_info_iter)?;
        let spl_account = next_account_info(account_info_iter)?;

        let system_program = next_account_info(account_info_iter)?;
        // let state_account = next_account_info(account_info_iter)?; // Add this line

        // Derive the state account PDA
        let (state_account_pda, bump_seed) =
            Pubkey::find_program_address(&[b"vault_registry"], program_id);

        let state_account = next_account_info(account_info_iter)?;

        // Ensure the state account matches the derived PDA
        if state_account.key != &state_account_pda {
            return Err(ProgramError::InvalidAccountData);
        }

        let associated_token_program = next_account_info(account_info_iter)?;
        let user_token_a_account = next_account_info(account_info_iter)?;
        let program_id_account = next_account_info(account_info_iter)?;

        // msg!("Creating vault...");
        msg!("payer account key: {:?}", payer_account.key);
        msg!("Mint account Token A key: {:?}", mint_account_token_a.key);
        msg!(
            "Mint account AToken A key: {:?}",
            mint_account_a_token_a.key
        );
        msg!("Vault account key: {:?}", vault_account.key);
        msg!("Rent account key: {:?}", rent_account.key);
        msg!("State account key: {:?}", state_account.key);
        msg!("SPL: {}", spl_account.key);
        msg!("user_token_a_account: {}", user_token_a_account.key);

        // NOTE: two mint types
        // msg!("Mint account balance: {:?}", mint_account.lamports());
        msg!(
            "Mint Token Aaccount balance: {:?}",
            mint_account_token_a.lamports()
        );
        msg!(
            "Mint AToken A account balance: {:?}",
            mint_account_a_token_a.lamports()
        );

        msg!("Vault account balance: {:?}", vault_account.lamports());
        msg!("Rent account balance: {:?}", rent_account.lamports());
        msg!("Payer account balance: {:?}", payer_account.lamports());
        msg!("spl account balance: {:?}", spl_account.lamports());
        msg!(
            "associated token account balance: {:?}",
            associated_token_program.lamports()
        );
        msg!(
            "user token a account: {:?}",
            user_token_a_account.lamports()
        );

        msg!("state account balance: {:?}", state_account.lamports());

        // Ensure accounts are rent-exempt
        if !payer_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // NOTE: is this needed?
        // if !mint_account.is_signer {
        //     return Err(ProgramError::MissingRequiredSignature);
        // }

        // if !vault_account.is_signer {
        //     return Err(ProgramError::MissingRequiredSignature);
        // }

        let rent = &Rent::from_account_info(rent_account)?;

        // Create and initialize the mint account
        // if mint_account.lamports() == 0 {
        let required_lamports = rent.minimum_balance(Mint::LEN);
        msg!("required_lamports: {}", required_lamports);

        //NOTE: ensure Token A mint is not empty
        // if mint_account_token_a.data_is_empty() {
        //     msg!("ERROR: mint_account_token_a should not be empty at vault creation time");
        //     return Err(ProgramError::Custom(99));
        // }

        // NOTE: check if ATokenA mint is empty, since we need to mint atokens
        if mint_account_a_token_a.data_is_empty() {
            msg!("create mint accct");
            invoke(
                &solana_program::system_instruction::create_account(
                    payer_account.key,
                    mint_account_a_token_a.key,
                    required_lamports,
                    Mint::LEN as u64,
                    spl_account.key,
                ),
                &[
                    payer_account.clone(),
                    mint_account_a_token_a.clone(),
                    system_program.clone(),
                ],
            )?;
            // }

            msg!("after create mint accct, now init mint");
            // Initialize the mint account
            match invoke(
                &initialize_mint(
                    &spl_token::id(),
                    &mint_account_a_token_a.key,
                    &payer_account.key,
                    Some(&payer_account.key),
                    0,
                )?,
                &[
                    mint_account_a_token_a.clone(),
                    rent_account.clone(),
                    payer_account.clone(),
                ],
            ) {
                Ok(_) => msg!("Mint account initialized successfully"),
                Err(e) => {
                    msg!("Failed to initialize mint account: {:?}", e);
                    return Err(e);
                }
            };
        }

        // msg!("after init mint, now lets create vault");

        //////////// NOTE DO WE MAKE A NEW VAULT ACCOUNT? or use an account
        // let vault_required_lamports = rent.minimum_balance(spl_token::state::Account::LEN);
        if vault_account.data_is_empty() {
            // associated attempt
            msg!("Actually Creating Associated Token vault account");

            invoke(
                &spl_associated_token_account::instruction::create_associated_token_account(
                    payer_account.key,
                    program_id_account.key,
                    mint_account_token_a.key, // the mint this associated account should be for is the token a mint
                    // NOTE:
                    spl_account.key, // SPL Token program ID is needed here
                                     // associated_token_program.key,
                ),
                &[
                    payer_account.clone(),        // Funding account
                    vault_account.clone(),        // Associated token account
                    payer_account.clone(),        // Wallet address
                    mint_account_token_a.clone(), // Token mint address
                    system_program.clone(),       // System program
                    // NOTE: spl or associated
                    spl_account.clone(), // SPL Token program

                                         // associated_token_program.clone(), // associated_token_program.clone(),
                ],
            )?;
        } else {
            msg!("Associated Token vault account is NOT EMPTY");
        }

        msg!("about to check if state_account is empty");
        //////////////////////////////////////////
        /// ////////////////////////////////////////////
        /// ////////////////////////////////////
        // Check if state account is empty and initialize it
        if state_account.data_is_empty() {
            // Correctly calculate the required size of the state account
            let mut vault_registry = VaultRegistry::new();
            let state_account_size = vault_registry.len();
            let state_account_required_lamports = rent.minimum_balance(state_account_size);

            // Create the state account
            msg!("about to create state cuz its empty");

            // invoke(
            invoke_signed(
                &solana_program::system_instruction::create_account(
                    payer_account.key,
                    state_account.key,
                    state_account_required_lamports,
                    state_account_size as u64,
                    program_id,
                ),
                &[
                    payer_account.clone(),
                    state_account.clone(),
                    system_program.clone(),
                ],
                // &[&[b"vault_registry", &[bump_seed]]],
                &[&[b"vault_registry".as_ref(), &[bump_seed]]],
            )?;

            // Initialize VaultRegistry and serialize it into the state account's data
            // let mut vault_registry = VaultRegistry { vaults: Vec::new() };

            let new_vault = Vault {
                vault_account: *vault_account.key,
                mint_token_a: *mint_account_token_a.key,
                mint_a_token_a: *mint_account_a_token_a.key,
                owner: *payer_account.key,
            };

            // Use the add_vault method
            if let Err(e) = vault_registry.add_vault(new_vault) {
                msg!("Failed to add vault: {}", e);
                return Err(ProgramError::Custom(0)); // Use appropriate error code
            }

            // Log the number of vaults after successful addition
            let number_of_vaults = vault_registry.vault_count();
            msg!("Number of vaults after addition: {}", number_of_vaults);

            // Debug: Print the vault registry state before serialization
            // msg!("VaultRegistry before serialization: {:?}", vault_registry);

            let mut state_data = state_account.data.borrow_mut();

            // Perform serialization with error handling
            let serialized_data = vault_registry.serialize();

            if serialized_data.len() != vault_registry.len() {
                msg!(
                    "Serialized length mismatch: expected {}, got {}",
                    vault_registry.len(),
                    serialized_data.len()
                );
                return Err(ProgramError::Custom(1));
            }

            // Verify length of serialized data
            let serialized_length = state_data.len();
            let expected_length = vault_registry.len();

            // Log the serialized and expected length

            let actual_serialized_length = 4 + vault_registry.vault_count() * Vault::LEN;
            msg!("Serialized length: {}", serialized_length);
            msg!("Expected length: {}", expected_length);
            msg!("actual_serialized_length: {}", actual_serialized_length);

            if serialized_data.len() != vault_registry.len() {
                msg!(
                    "Serialized length mismatch: expected {}, got {}",
                    vault_registry.len(),
                    serialized_data.len()
                );
                return Err(ProgramError::Custom(1));
            }

            state_data[..serialized_data.len()].copy_from_slice(&serialized_data);

            // Log the serialized data for verification
            msg!(
                "Serialized state data (first 64 bytes): {:?}",
                &state_data[..64]
            );

            state_data[..vault_registry.len()].copy_from_slice(&serialized_data);

            let deserialized_vault_registry = VaultRegistry::deserialize(&state_data[..]);

            // Log the number of vaults after deserialization
            if deserialized_vault_registry.is_ok() {
                let number_of_vaults_after_deserialization =
                    deserialized_vault_registry.unwrap().vault_count();
                msg!(
                    "Number of vaults after deserialization: {}",
                    number_of_vaults_after_deserialization
                );
            } else {
                return Err(ProgramError::Custom(2));
            }

            // msg!("State account initialized successfully");
        } else {
            msg!("State account is already initialized");

            // Step 1: Deserialize the existing vault registry
            let mut state_data = state_account.try_borrow_mut_data()?;
            let mut vault_registry = match VaultRegistry::deserialize(&mut state_data.as_ref()) {
                Ok(vr) => vr,
                Err(_) => {
                    msg!("Failed to deserialize existing VaultRegistry");
                    return Err(ProgramError::InvalidAccountData);
                }
            };

            // Step 2: Add the new vault to the registry
            let new_vault = Vault {
                vault_account: *vault_account.key,
                mint_token_a: *mint_account_token_a.key,
                mint_a_token_a: *mint_account_a_token_a.key,
                owner: *payer_account.key,
            };

            if let Err(e) = vault_registry.add_vault(new_vault) {
                msg!("Failed to add vault: {}", e);
                return Err(ProgramError::Custom(0)); // Use appropriate error code
            }

            // Step 3: Serialize the updated vault registry back into the state account
            let serialized_data = vault_registry.serialize();
            if serialized_data.len() != vault_registry.len() {
                msg!(
                    "Serialized length mismatch: expected {}, got {}",
                    vault_registry.len(),
                    serialized_data.len()
                );
                return Err(ProgramError::Custom(1));
            }

            state_data[..serialized_data.len()].copy_from_slice(&serialized_data);

            // Optional: Log the serialized data for verification
            msg!(
                "Updated serialized state data (first 64 bytes): {:?}",
                &state_data[..64]
            );

            // msg!("VaultRegistry updated successfully");

            // Log the number of vaults in the registry after the update
            let vault_count = vault_registry.vault_count();
            msg!("Number of vaults after update: {}", vault_count);

            // Optional: Log the serialized data for verification
            msg!(
                "Updated serialized state data (first 64 bytes): {:?}",
                &state_data[..64]
            );
        }

        ////////////////////////////////////
        /// ////////////////////////////////
        /// //////////////////////////////
        // msg!("Vault created successfully");
        Ok(())
    }

    ////////////////////////////////////////
    ///
    ///
    ///

    /*
    @name process_deposit
    @description Handles the deposit of tokens into a vault, including transferring the user's tokens to the vault and minting the corresponding amount of aTokens.
    @param _program_id - The ID of the currently executing program.
    @param accounts - The accounts involved in the transaction.
    @param amount - The amount of tokens to be deposited.
    @note creates user_token_a and user_aatoken_a
    */
    fn process_deposit(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64, // Pass the amount directly
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        // Accounts required for the deposit process
        msg!("Getting account info for the deposit process...");
        let payer_account = next_account_info(account_info_iter)?; // Payer account
        msg!("Payer account: {}", payer_account.key);

        // let mint_account = next_account_info(account_info_iter)?; // Mint account
        // msg!("Mint account: {}", mint_account.key);
        let mint_token_a_account = next_account_info(account_info_iter)?;
        msg!("TokenA Mint account: {}", mint_token_a_account.key);

        let mint_atoken_a_account = next_account_info(account_info_iter)?;
        msg!("ATokenA Mint account: {}", mint_atoken_a_account.key);

        let vault_account = next_account_info(account_info_iter)?; // Vault account
        msg!("Vault account: {}", vault_account.key);

        let user_token_a_account = next_account_info(account_info_iter)?; // User's TokenA account
        msg!("User TokenA account: {}", user_token_a_account.key);

        // Check the mint associated with the user's TokenA account
        let user_token_a_account_info =
            spl_token::state::Account::unpack(&user_token_a_account.data.borrow())?;
        msg!(
            "User TokenA account mint: {}",
            user_token_a_account_info.mint
        );

        let user_token_account_info = TokenAccount::unpack(&user_token_a_account.data.borrow())?;
        // let vault_account_info = TokenAccount::unpack(&vault_account.data.borrow())?;

        // msg!("Vault account mint: {}", vault_account_info.mint);

        // SAYING: is the users account mint, is not
        if user_token_account_info.mint != *mint_token_a_account.key {
            msg!("Error: The mint associated with the user's TokenA account does not match the expected mint.");
            return Err(ProgramError::InvalidAccountData);
        }

        if user_token_account_info.owner != *payer_account.key {
            msg!("Error: Payer account does not own the user TokenA account.");
            return Err(ProgramError::IllegalOwner);
        }

        let user_atoken_account = next_account_info(account_info_iter)?; // User's aTokenA account
        msg!("User aTokenA account: {}", user_atoken_account.key);

        // msg!("user_atoken_account.data: {}", user_atoken_account.data);
        // let user_atoken_account_info = TokenAccount::unpack(&user_atoken_account.data.borrow())?;
        // msg!(
        //     "user_atoken_account_info: {}",
        //     user_atoken_account_info.mint
        // );
        // if user_atoken_account_info.mint != *mint_atoken_a_account.key {
        //     msg!("Error: The mint associated with the user's ATokenA account does not match the expected AtokenA mint.");
        //     return Err(ProgramError::Custom(123));
        // }

        let rent_account = next_account_info(account_info_iter)?; // Rent sysvar account
        msg!("Rent account: {}", rent_account.key);

        let spl_account = next_account_info(account_info_iter)?; // SPL Token program account
        msg!("SPL Token program account: {}", spl_account.key);

        let system_program = next_account_info(account_info_iter)?; // System program account
        msg!("System program account: {}", system_program.key);

        let associated_token_program = next_account_info(account_info_iter)?; // System program account
        msg!(
            "Associated Token program account: {}",
            associated_token_program.key
        );

        // NOTE: if the users ATokenA account doesnt exist, then create one
        msg!(
            "user_atoken_account.lamports(): {}",
            user_atoken_account.lamports()
        );
        if user_atoken_account.lamports() == 0 {
            // let rent = &Rent::from_account_info(rent_account)?;
            // let required_lamports = rent.minimum_balance(TokenAccount::LEN);

            msg!("user_atoken_account.lamports() == 0");

            invoke(
                &spl_associated_token_account::instruction::create_associated_token_account(
                    payer_account.key,
                    payer_account.key,
                    mint_atoken_a_account.key, // the mint this associated account should be for is the token a mint
                    // NOTE:
                    &spl_token::id(), // SPL Token program ID is needed here
                                      // associated_token_program.key,
                ),
                &[
                    payer_account.clone(),         // Funding account
                    user_atoken_account.clone(),   // Associated token account
                    payer_account.clone(),         // Wallet address
                    mint_atoken_a_account.clone(), // Token mint address
                    system_program.clone(),        // System program
                    // NOTE: spl or associated
                    spl_account.clone(), // SPL Token program
                    // associated_token_program.clone(), // associated_token_program.clone(),
                    rent_account.clone(),
                ],
            )?;
        }

        /////////////////////////
        /// //////////WORKED!!
        /// ////////////////////////////
        // Log balances before transfer
        msg!("Log balances before transfer");
        let user_token_a_balance_before =
            TokenAccount::unpack(&user_token_a_account.try_borrow_data()?)?.amount;
        let vault_token_a_balance_before =
            TokenAccount::unpack(&vault_account.try_borrow_data()?)?.amount;
        let user_atoken_balance_before =
            TokenAccount::unpack(&user_atoken_account.try_borrow_data()?)?.amount;

        msg!(
            "User TokenA account balance before transfer: {}",
            user_token_a_balance_before
        );
        msg!(
            "Vault TokenA account balance before transfer: {}",
            vault_token_a_balance_before
        );
        msg!(
            "User aTokenA account balance before minting: {}",
            user_atoken_balance_before
        );

        // Transfer TokenA from user to vault
        msg!("Transferring {} TokenA from user to vault", amount);
        invoke(
            &spl_token::instruction::transfer(
                &spl_token::id(),
                user_token_a_account.key,
                vault_account.key,
                payer_account.key,
                &[], // No multisig signing required
                amount,
            )?,
            &[
                user_token_a_account.clone(),
                vault_account.clone(),
                payer_account.clone(),
            ],
        )?;
        msg!("Transfer completed");

        // Log balances after transfer
        let user_token_a_balance_after =
            TokenAccount::unpack(&user_token_a_account.try_borrow_data()?)?.amount;
        let vault_token_a_balance_after =
            TokenAccount::unpack(&vault_account.try_borrow_data()?)?.amount;
        msg!(
            "User TokenA account balance after transfer: {}",
            user_token_a_balance_after
        );
        msg!(
            "Vault TokenA account balance after transfer: {}",
            vault_token_a_balance_after
        );

        // Mint aTokenA equivalent to the amount of TokenA deposited
        msg!("Minting -- {} aTokenA to user's aTokenA account", amount);
        invoke(
            &spl_token::instruction::mint_to(
                // &spl_token::id(),
                &spl_token::id(),
                // &spl_associated_token_account::id(),
                mint_atoken_a_account.key,
                user_atoken_account.key,
                payer_account.key,
                &[], // No multisig signing required
                amount,
            )?,
            &[
                mint_atoken_a_account.clone(),
                user_atoken_account.clone(),
                payer_account.clone(),
            ],
        )?;
        msg!("Minting completed");

        // Log balances after minting
        let user_atoken_balance_after =
            TokenAccount::unpack(&user_atoken_account.try_borrow_data()?)?.amount;
        msg!(
            "User aTokenA account balance after minting: {}",
            user_atoken_balance_after
        );

        msg!("Deposit process completed successfully");
        Ok(())
    }

    /*
    @name process_withdraw
    @description Handles the withdrawal of tokens from a vault, including burning the corresponding rTokens and transferring the requested amount to the user's account.
    @param _program_id - The ID of the currently executing program.
    @param accounts - The accounts involved in the transaction.
    @param amount - The amount of tokens to be withdrawn.
    */
    fn process_withdraw(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let vault_account = next_account_info(account_info_iter)?;
        let user_account = next_account_info(account_info_iter)?;
        let user_rtoken_account = next_account_info(account_info_iter)?;
        let mint_account = next_account_info(account_info_iter)?;
        msg!("Withdrawing {} lamports", amount);

        invoke(
            &burn(
                &spl_token::id(),
                user_rtoken_account.key,
                mint_account.key,
                user_account.key,
                &[],
                amount,
            )?,
            &[
                user_rtoken_account.clone(),
                mint_account.clone(),
                user_account.clone(),
            ],
        )?;

        invoke(
            &solana_program::system_instruction::transfer(
                vault_account.key,
                user_account.key,
                amount,
            ),
            &[vault_account.clone(), user_account.clone()],
        )?;

        Ok(())
    }

    /*
    @name process_burn_rtoken
    @description Handles the burning of rTokens and minting of bTokens to the user's account.
    @param _program_id - The ID of the currently executing program.
    @param accounts - The accounts involved in the transaction.
    @param amount - The amount of rTokens to be burned and bTokens to be minted.
    */
    fn process_burn_rtoken(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let user_rtoken_account = next_account_info(account_info_iter)?;
        let mint_account = next_account_info(account_info_iter)?;
        let user_account = next_account_info(account_info_iter)?;
        let btoken_account = next_account_info(account_info_iter)?;

        msg!("Burning {} RToken", amount);

        invoke(
            &burn(
                &spl_token::id(),
                user_rtoken_account.key,
                mint_account.key,
                user_account.key,
                &[],
                amount,
            )?,
            &[
                user_rtoken_account.clone(),
                mint_account.clone(),
                user_account.clone(),
            ],
        )?;

        invoke(
            &mint_to(
                &spl_token::id(),
                mint_account.key,
                btoken_account.key,
                user_account.key,
                &[],
                amount,
            )?,
            &[
                mint_account.clone(),
                btoken_account.clone(),
                user_account.clone(),
            ],
        )?;

        Ok(())
    }

    fn process_faucet(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let payer_account = next_account_info(account_info_iter)?;

        let user_token_account = next_account_info(account_info_iter)?;
        let mint_account = next_account_info(account_info_iter)?;
        // let user_token_account = get_associated_token_address(&payer_account.key, &mint_account.key);
        let spl_account = next_account_info(account_info_iter)?;
        let rent_account = next_account_info(account_info_iter)?;
        let system_program = next_account_info(account_info_iter)?;

        let associated_token_program = next_account_info(account_info_iter)?; // System program account

        let expected_user_token_pubkey =
            get_associated_token_address(payer_account.key, mint_account.key);

        // Derive the PDA for the mint account
        let (expected_mint_pubkey, bump_seed) =
            Pubkey::find_program_address(&[b"mint"], program_id);

        msg!("expected_mint_pubkey: {}", expected_mint_pubkey);
        msg!("comparing it to: {}", mint_account.key);
        // Ensure the derived mint address matches the passed mint account
        // if *mint_account.key != expected_mint_pubkey {
        //     msg!("Error: Invalid mint account passed");
        //     return Err(ProgramError::InvalidArgument);
        // }

        // Ensure the payer is a signer
        msg!("payer_account.is_signer: {}", payer_account.is_signer);

        // Check if the mint account needs to be created and initialized
        if mint_account.data_is_empty() {
            let rent = Rent::get()?;
            let required_lamports = rent.minimum_balance(Mint::LEN);

            // Create the mint account
            msg!("1");

            invoke_signed(
                &solana_program::system_instruction::create_account(
                    payer_account.key,
                    mint_account.key,
                    required_lamports,
                    Mint::LEN as u64,
                    spl_account.key,
                ),
                &[
                    payer_account.clone(),
                    mint_account.clone(),
                    system_program.clone(),
                ],
                // &[&[b"vault_registry", &[bump_seed]]],
                &[&[b"mint".as_ref(), &[bump_seed]]],
            )?;

            msg!("2");

            // // Initialize the mint account
            invoke(
                &spl_token::instruction::initialize_mint(
                    &spl_token::id(),
                    mint_account.key,
                    payer_account.key,
                    None,
                    0,
                )?,
                &[
                    mint_account.clone(),
                    rent_account.clone(),
                    payer_account.clone(),
                ],
            )?;
        }

        msg!("3");

        // Ensure the user token account is created and initialized
        if user_token_account.data_is_empty() {
            invoke(
                &spl_associated_token_account::instruction::create_associated_token_account(
                    payer_account.key,
                    payer_account.key,
                    mint_account.key,
                    spl_account.key,
                ),
                &[
                    payer_account.clone(),
                    user_token_account.clone(),
                    payer_account.clone(),
                    mint_account.clone(),
                    system_program.clone(),
                    spl_account.clone(),
                    //
                    // payer_account.clone(),        // Funding account
                    // vault_account.clone(),        // Associated token account
                    // payer_account.clone(),        // Wallet address
                    // mint_account_token_a.clone(), // Token mint address
                    // system_program.clone(),       // System program
                    // spl_account.clone(),
                    /*
                    payer_account.clone(),        // Funding account
                    vault_account.clone(),        // Associated token account
                    payer_account.clone(),        // Wallet address
                    mint_account_token_a.clone(), // Token mint address
                    system_program.clone(),       // System program
                    // NOTE: spl or associated
                    spl_account.clone(),
                    */
                ],
            )?;
        }

        msg!("5");

        // Mint the specified amount of tokens to the user's account
        invoke(
            &mint_to(
                &spl_token::id(),
                mint_account.key,
                &user_token_account.key,
                payer_account.key,
                &[],
                amount,
            )?,
            &[
                mint_account.clone(),
                user_token_account.clone(),
                payer_account.clone(),
            ],
        )?;

        msg!(
            "Faucet completed successfully with mint {} to user {}",
            mint_account.key,
            user_token_account.key
        );

        Ok(())
    }
}
