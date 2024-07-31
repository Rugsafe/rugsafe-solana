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
        let mint_account = next_account_info(account_info_iter)?;
        // let owner_account = next_account_info(account_info_iter)?;
        let rent_account = next_account_info(account_info_iter)?;
        let spl_account = next_account_info(account_info_iter)?;
        let system_program = next_account_info(account_info_iter)?;
        let mint_account_data_len = mint_account.data_len();
        msg!("Mint account data length: {}", mint_account_data_len);

        msg!("Creating vault...");
        msg!("payer account key: {:?}", payer_account.key);
        msg!("Mint account key: {:?}", mint_account.key);
        msg!("Rent account key: {:?}", rent_account.key);
        msg!("SPL: {}", spl_token::id());
        msg!("Mint account balance: {:?}", mint_account.lamports());
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

        msg!("1");
        let rent = &Rent::from_account_info(rent_account)?;

        msg!("2");
        let program_account_info = AccountInfo::new(
            program_id,
            false,
            false,
            &mut 0,
            &mut [],
            program_id,
            false,
            0,
        );
        /////////////// create account
        if mint_account.lamports() == 0 {
            // // Create the mint account
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
                    system_program.clone(),
                ],
            )?;
        }
        ////////////

        // Initialize the mint account
        msg!("Initializing mint account... v4");

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

        msg!("Vault created successfully");
        Ok(())
    }

    fn process_deposit(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        // Accounts required for the deposit process
        let user_account = next_account_info(account_info_iter)?; // User's main account, which must be a signer
        let user_token_account = next_account_info(account_info_iter)?; // User's TokenA account
        let vault_token_account = next_account_info(account_info_iter)?; // Vault's TokenA account
        let mint_account = next_account_info(account_info_iter)?; // Mint account for aTokenA
        let user_atoken_account = next_account_info(account_info_iter)?; // User's aTokenA account

        // Verify that the user account is a signer
        if !user_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // if token_program.key != &spl_token::id() {
        //     return Err(ProgramError::IncorrectProgramId);
        // }

        // Log the deposit action
        msg!("Depositing {} TokenA from user to vault", amount);

        // Transfer TokenA from user to vault
        invoke(
            &spl_token::instruction::transfer(
                &spl_token::id(),
                user_token_account.key,
                vault_token_account.key,
                user_account.key,
                &[], // No multisig signing required
                amount,
            )?,
            &[
                user_token_account.clone(),
                vault_token_account.clone(),
                user_account.clone(),
            ],
        )?;

        // Mint aTokenA equivalent to the amount of TokenA deposited
        msg!("Minting {} aTokenA to user's aTokenA account", amount);
        invoke(
            &spl_token::instruction::mint_to(
                &spl_token::id(),
                mint_account.key,
                user_atoken_account.key,
                user_account.key,
                &[], // No multisig signing required
                amount,
            )?,
            &[
                mint_account.clone(),
                user_atoken_account.clone(),
                user_account.clone(),
            ],
        )?;

        msg!("Deposit process completed successfully");
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
