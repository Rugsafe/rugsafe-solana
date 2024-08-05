use crate::instruction::VaultInstruction;
use solana_program::sysvar::Sysvar;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    sysvar,
    sysvar::rent::Rent,
};
use spl_token::instruction::{burn, initialize_mint, initialize_mint2, mint_to};
use spl_token::state::Account as TokenAccount;
use spl_token::state::Mint;

pub struct Processor;

impl Processor {
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
        }
    }
    fn process_create_vault(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let payer_account = next_account_info(account_info_iter)?;
        let mint_account = next_account_info(account_info_iter)?;
        let vault_account = next_account_info(account_info_iter)?;
        let rent_account = next_account_info(account_info_iter)?;
        let spl_account = next_account_info(account_info_iter)?;
        let system_program = next_account_info(account_info_iter)?;

        msg!("Creating vault...");
        msg!("payer account key: {:?}", payer_account.key);
        msg!("Mint account key: {:?}", mint_account.key);
        msg!("Vault account key: {:?}", vault_account.key);
        msg!("Rent account key: {:?}", rent_account.key);
        msg!("SPL: {}", spl_token::id());
        msg!("Mint account balance: {:?}", mint_account.lamports());
        msg!("Vault account balance: {:?}", vault_account.lamports());
        msg!("Rent account balance: {:?}", rent_account.lamports());
        msg!("Payer account balance: {:?}", payer_account.lamports());
        msg!("spl account balance: {:?}", spl_account.lamports());

        // Ensure accounts are rent-exempt
        if !payer_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if !mint_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // if !vault_account.is_signer {
        //     return Err(ProgramError::MissingRequiredSignature);
        // }

        let rent = &Rent::from_account_info(rent_account)?;

        // Create and initialize the mint account
        // if mint_account.lamports() == 0 {
        let required_lamports = rent.minimum_balance(Mint::LEN);
        msg!("required_lamports: {}", required_lamports);

        msg!("create mint accct");
        invoke(
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
        )?;
        // }

        msg!("after create mint accct, now init mint");
        // Initialize the mint account
        match invoke(
            &initialize_mint(
                &spl_token::id(),
                &mint_account.key,
                &payer_account.key,
                Some(&payer_account.key),
                0,
            )?,
            &[
                mint_account.clone(),
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

        msg!("after init mint, now lets create vault");

        let vault_required_lamports = rent.minimum_balance(spl_token::state::Account::LEN);
        msg!("Creating vault account");
        invoke(
            &solana_program::system_instruction::create_account(
                payer_account.key,
                vault_account.key,
                vault_required_lamports,
                spl_token::state::Account::LEN as u64,
                spl_account.key,
            ),
            &[
                payer_account.clone(),
                vault_account.clone(),
                system_program.clone(),
            ],
        )?;
        msg!("after create vault, now lets init vault");
        // Initialize the vault account (assuming it's a token account)
        invoke(
            &spl_token::instruction::initialize_account(
                &spl_token::id(),
                vault_account.key,
                mint_account.key,
                payer_account.key,
            )?,
            &[
                vault_account.clone(),
                mint_account.clone(),
                rent_account.clone(),
                payer_account.clone(),
                spl_account.clone(),
            ],
        )?;

        msg!("Vault created successfully");
        Ok(())
    }

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

        let mint_account = next_account_info(account_info_iter)?; // Mint account
        msg!("Mint account: {}", mint_account.key);

        let vault_account = next_account_info(account_info_iter)?; // Vault account
        msg!("Vault account: {}", vault_account.key);

        let user_token_account = next_account_info(account_info_iter)?; // User's TokenA account
        msg!("User TokenA account: {}", user_token_account.key);

        // Check the mint associated with the user's TokenA account
        let user_token_account_info =
            spl_token::state::Account::unpack(&user_token_account.data.borrow())?;
        msg!("User TokenA account mint: {}", user_token_account_info.mint);
        msg!("mint_account.key: {}", mint_account.key);

        let user_token_account_info = TokenAccount::unpack(&user_token_account.data.borrow())?;
        let vault_account_info = TokenAccount::unpack(&vault_account.data.borrow())?;

        msg!("User TokenA account mint: {}", user_token_account_info.mint);
        msg!("Vault account mint: {}", vault_account_info.mint);

        if user_token_account_info.mint != *mint_account.key {
            msg!("Error: The mint associated with the user's TokenA account does not match the expected mint.");
            return Err(ProgramError::InvalidAccountData);
        }

        if user_token_account_info.owner != *payer_account.key {
            msg!("Error: Payer account does not own the user TokenA account.");
            return Err(ProgramError::IllegalOwner);
        }

        let user_atoken_account = next_account_info(account_info_iter)?; // User's aTokenA account
        msg!("User aTokenA account: {}", user_atoken_account.key);

        let rent_account = next_account_info(account_info_iter)?; // Rent sysvar account
        msg!("Rent account: {}", rent_account.key);

        let spl_account = next_account_info(account_info_iter)?; // SPL Token program account
        msg!("SPL Token program account: {}", spl_account.key);

        let system_program = next_account_info(account_info_iter)?; // System program account
        msg!("System program account: {}", system_program.key);

        // Verify that the user account is a signer
        if !payer_account.is_signer {
            msg!("Error: Payer account is not a signer");
            return Err(ProgramError::MissingRequiredSignature);
        }
        msg!("Payer account is a signer");

        // Log balances before transfer
        let user_token_a_balance_before =
            TokenAccount::unpack(&user_token_account.try_borrow_data()?)?.amount;
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
                user_token_account.key,
                vault_account.key,
                payer_account.key,
                &[], // No multisig signing required
                amount,
            )?,
            &[
                user_token_account.clone(),
                vault_account.clone(),
                payer_account.clone(),
            ],
        )?;
        msg!("Transfer completed");

        // Log balances after transfer
        let user_token_a_balance_after =
            TokenAccount::unpack(&user_token_account.try_borrow_data()?)?.amount;
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
        msg!("Minting {} aTokenA to user's aTokenA account", amount);
        invoke(
            &spl_token::instruction::mint_to(
                &spl_token::id(),
                mint_account.key,
                user_atoken_account.key,
                payer_account.key,
                &[], // No multisig signing required
                amount,
            )?,
            &[
                mint_account.clone(),
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

    //////////////////////////////////////////////////
    /// ////////////////////////////////////////////////
    ///
    ///
    ///
    ///
    ///
    ///
    ///
    ///
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
}
