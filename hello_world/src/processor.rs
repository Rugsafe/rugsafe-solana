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
        msg!("v2 length of accounts: {}", account_info_iter.len());
        let payer_account = next_account_info(account_info_iter)?;
        let mint_account: &AccountInfo = next_account_info(account_info_iter)?;
        let owner_account = next_account_info(account_info_iter)?;
        // let mint_account = payer_account;
        // let owner_account = payer_account;
        let rent_account = next_account_info(account_info_iter)?;
        let spl_account = next_account_info(account_info_iter)?;

        let mint_account_data_len = mint_account.data_len();
        msg!("Mint account data length: {}", mint_account_data_len);
        // if mint_account_data_len != Mint {
        //     msg!("Mint account data size mismatch!");
        //     return Err(ProgramError::InvalidAccountData);
        // }

        msg!("Creating vault...");
        msg!("payer account key: {:?}", payer_account.key);
        msg!("Mint account key: {:?}", mint_account.key);
        msg!("Owner account key: {:?}", owner_account.key);
        msg!("Rent account key: {:?}", rent_account.key);
        msg!("SPL: {}", spl_token::id());
        msg!("Mint account balance: {:?}", mint_account.lamports());
        msg!("Owner account balance: {:?}", owner_account.lamports());
        msg!("Rent account balance: {:?}", rent_account.lamports());
        msg!("Payer account balance: {:?}", payer_account.lamports());
        msg!("spl account balance: {:?}", spl_account.lamports());

        //// Ensure the owner is a PDA derived from the program ID
        // let (derived_owner_pubkey, bump_seed) = Pubkey::find_program_address(&[b"vault"], program_id);

        // if *owner_account.key != derived_owner_pubkey {
        //     return Err(ProgramError::InvalidAccountData);
        // }

        // Ensure accounts are rent-exempt
        if !payer_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // if !owner_account.is_signer {
        //     return Err(ProgramError::MissingRequiredSignature);
        // }

        let rent = &Rent::from_account_info(rent_account)?;
        // if !rent.is_exempt(mint_account.lamports(), mint_account.data_len()) {
        //     msg!("Mint account is not rent-exempt");
        //     return Err(ProgramError::AccountNotRentExempt);
        // }

        // if !rent.is_exempt(owner_account.lamports(), owner_account.data_len()) {
        //     msg!("Owner account is not rent-exempt");
        //     return Err(ProgramError::AccountNotRentExempt);
        // }

        /////////////// create account
        if mint_account.lamports() == 0 {
            // let (mint_pda, mint_bump) = Pubkey::find_program_address(&[b"mint"], program_id);

            // // Create the mint account
            // let mint_account = next_account_info(account_info_iter)?;
            let required_lamports = rent.minimum_balance(Mint::LEN);
            msg!("required_lamports: {}", required_lamports);
            // // Ensure payer_account funds the mint account
            msg!("before create account");
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
                    // spl_account.clone(),
                    // rent_account.clone(),
                ],
                // &[payer_account.clone()],
            )?;
        }
        ////////////

        // Initialize the mint account
        msg!("Initializing mint account... v4");
        match invoke(
            &initialize_mint(
                &spl_token::id(),
                // &_program_id,
                &mint_account.key,
                &owner_account.key,
                Some(&owner_account.key),
                0,
            )?,
            &[
                mint_account.clone(),
                rent_account.clone(),
                owner_account.clone(),
                payer_account.clone(),
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
