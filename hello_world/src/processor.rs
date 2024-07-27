use crate::instruction::VaultInstruction;
use solana_program::sysvar::Sysvar;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar,
    sysvar::rent::Rent,
};
use spl_token::instruction::{burn, initialize_mint, mint_to}; // Import the Sysvar trait

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

    fn process_create_vault(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let payer_account = next_account_info(account_info_iter)?;
        let mint_account: &AccountInfo = next_account_info(account_info_iter)?;
        let owner_account = next_account_info(account_info_iter)?;
        let rent_account = next_account_info(account_info_iter)?;

        msg!("Creating vault...");
        msg!("payer account key: {:?}", payer_account.key);
        msg!("Mint account key: {:?}", mint_account.key);
        msg!("Owner account key: {:?}", owner_account.key);
        msg!("Rent account key: {:?}", rent_account.key);
        msg!("Mint account balance: {:?}", mint_account.lamports());
        msg!("Owner account balance: {:?}", owner_account.lamports());
        msg!("Rent account balance: {:?}", rent_account.lamports());

        // Ensure accounts are rent-exempt
        if !payer_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if !owner_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let rent = &Rent::from_account_info(rent_account)?;
        if !rent.is_exempt(mint_account.lamports(), mint_account.data_len()) {
            msg!("Mint account is not rent-exempt");
            return Err(ProgramError::AccountNotRentExempt);
        }
        if !rent.is_exempt(owner_account.lamports(), owner_account.data_len()) {
            msg!("Owner account is not rent-exempt");
            return Err(ProgramError::AccountNotRentExempt);
        }

        // Initialize the mint account
        msg!("Initializing mint account...");
        match invoke(
            &initialize_mint(
                &spl_token::id(),
                &mint_account.key,
                &owner_account.key,
                Some(&owner_account.key),
                0,
            )?,
            &[
                mint_account.clone(),
                rent_account.clone(),
                owner_account.clone(),
            ],
        ) {
            Ok(_) => msg!("Mint account initialized successfully"),
            Err(e) => {
                msg!("Failed to initialize mint account: {:?}", e);
                return Err(e);
            }
        };

        msg!("Vault created successfully");
        Ok(())
    }

    fn process_deposit(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let user_account = next_account_info(account_info_iter)?;
        let vault_account = next_account_info(account_info_iter)?;

        msg!("Depositing {} lamports", amount);

        invoke(
            &solana_program::system_instruction::transfer(
                user_account.key,
                vault_account.key,
                amount,
            ),
            &[user_account.clone(), vault_account.clone()],
        )?;

        Ok(())
    }

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
