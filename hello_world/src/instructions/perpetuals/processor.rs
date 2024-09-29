use crate::instructions::perpetuals::PerpetualsInstruction;
use crate::state::perpetuals::{Position, Side, UserPositions};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    clock::Clock,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};
use spl_associated_token_account::get_associated_token_address;

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = PerpetualsInstruction::unpack(instruction_data)?;

        match instruction {
            PerpetualsInstruction::OpenPosition { side, amount } => {
                Self::process_open_position(program_id, accounts, side, amount)
            }
            PerpetualsInstruction::ClosePosition { position_id } => {
                Self::process_close_position(program_id, accounts, position_id)
            }
            PerpetualsInstruction::AddCollateral {
                position_id,
                amount,
            } => Self::process_add_collateral(program_id, accounts, position_id, amount),
            PerpetualsInstruction::RemoveCollateral {
                position_id,
                amount,
            } => Self::process_remove_collateral(program_id, accounts, position_id, amount),
            PerpetualsInstruction::LiquidatePosition { position_id } => {
                Self::process_liquidate_position(program_id, accounts, position_id)
            }
        }
    }

    fn process_open_position(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        side: Side,
        amount: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let payer_account = next_account_info(account_info_iter)?; // User who opens the position
        let user_positions_account = next_account_info(account_info_iter)?; // User's positions account (PDA)
        let user_collateral_account = next_account_info(account_info_iter)?; // User's collateral token account
        let collateral_mint_account = next_account_info(account_info_iter)?; // Collateral mint account
        let custody_account = next_account_info(account_info_iter)?; // Custody account (associated token account)
        let spl_account = next_account_info(account_info_iter)?; // Token program
        let system_program = next_account_info(account_info_iter)?; // System program
        let rent_account = next_account_info(account_info_iter)?; // Rent sysvar
        let associated_token_program = next_account_info(account_info_iter)?; // Associated token account program
        let program_account = next_account_info(account_info_iter)?; // Program's AccountInfo

        msg!("OpenPosition Account: Program ID: {:?}", program_id);
        msg!(
            "OpenPosition Account: User Positions: {:?}",
            user_positions_account.key
        );
        msg!(
            "OpenPosition Account: User Collateral: {:?}",
            user_collateral_account.key
        );
        msg!(
            "OpenPosition Account: Collateral Mint: {:?}",
            collateral_mint_account.key
        );
        msg!(
            "OpenPosition Account: Custody ATA: {:?}",
            custody_account.key
        );
        msg!(
            "OpenPosition Account: Passed Custody Account: {:?}",
            custody_account.key
        );
        msg!(
            "OpenPosition Account: Collateral Mint Account: {:?}",
            collateral_mint_account.key
        );
        msg!("OpenPosition Account: SPL Token: {:?}", spl_account.key);
        msg!(
            "OpenPosition Account: System Program: {:?}",
            system_program.key
        );
        msg!("OpenPosition Account: Rent: {:?}", rent_account.key);
        msg!(
            "OpenPosition Account: Associated Token Program: {:?}",
            associated_token_program.key
        );
        msg!(
            "OpenPosition Account: Program Accout: {:?}",
            program_account.key
        );
        // Ensure the payer is a signer
        if !payer_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Derive the user_positions_account PDA
        let (user_positions_pda, user_positions_bump) = Pubkey::find_program_address(
            &[b"user_positions", payer_account.key.as_ref()],
            program_id,
        );

        // Check that the provided user_positions_account matches the derived PDA
        if user_positions_account.key != &user_positions_pda {
            msg!("User positions account key does not match the expected PDA");
            return Err(ProgramError::InvalidArgument);
        }

        // If the user_positions_account data is empty, create it
        if user_positions_account.data_is_empty() {
            let rent = &Rent::from_account_info(rent_account)?;
            msg!("rent: {:?}", rent);
            let required_lamports = rent.minimum_balance(UserPositions::LEN as usize);
            msg!("required_lamports: {:?}", required_lamports);
            msg!(
                "Required lamports for rent exemption: {}",
                required_lamports
            );

            let seeds = &[
                b"user_positions",
                payer_account.key.as_ref(),
                &[user_positions_bump],
            ];

            msg!("PDA seeds: {:?}", seeds);

            invoke_signed(
                &solana_program::system_instruction::create_account(
                    payer_account.key,
                    user_positions_account.key,
                    required_lamports,
                    UserPositions::LEN as u64,
                    program_id,
                ),
                &[
                    payer_account.clone(),
                    user_positions_account.clone(),
                    system_program.clone(),
                ],
                &[seeds],
            )?;
        }

        if custody_account.data_is_empty() {
            // Create the associated token account for custody
            msg!("Custody Account is empty");
            invoke(
                // &spl_associated_token_account::create_associated_token_account(
                &spl_associated_token_account::instruction::create_associated_token_account(
                    payer_account.key,
                    program_id, // Owner is the program
                    collateral_mint_account.key,
                    spl_account.key,
                ),
                &[
                    payer_account.clone(),           // Funding account
                    custody_account.clone(),         // Associated token account
                    program_account.clone(),         // Wallet address (program as the owner)
                    collateral_mint_account.clone(), // Token mint account
                    system_program.clone(),          // System program
                    spl_account.clone(),             // SPL Token program
                    rent_account.clone(),            // Rent account
                ],
            )?;
        }

        // Deserialize the UserPositions account data
        let mut user_positions_data = user_positions_account.try_borrow_mut_data()?;
        let mut data_slice: &[u8] = &user_positions_data;

        let mut user_positions = if data_slice.iter().all(|&x| x == 0) {
            // Account data is uninitialized, initialize UserPositions
            UserPositions {
                owner: *payer_account.key,
                positions: Vec::new(),
            }
        } else {
            // Account data is initialized, deserialize
            let user_positions = UserPositions::deserialize(&mut data_slice)?;

            // Check if the owner matches
            if user_positions.owner != *payer_account.key {
                msg!("User positions account owner does not match payer");
                return Err(ProgramError::InvalidAccountData);
            }
            user_positions
        };

        // Check if positions vector is not full
        if user_positions.positions.len() >= UserPositions::MAX_POSITIONS {
            msg!("Maximum number of positions reached");
            return Err(ProgramError::AccountDataTooSmall);
        }

        // Derive the associated token account for the custody account (associated with the collateral mint)
        let custody_ata = get_associated_token_address(&program_id, collateral_mint_account.key);

        // Check that the provided custody account matches the derived associated token account
        if custody_account.key != &custody_ata {
            msg!("Custody account key does not match the expected associated token account");
            return Err(ProgramError::InvalidArgument);
        } else {
            msg!("Custody Account matches expected Associated Token Account");
        }

        // Check that the provided custody account matches the derived associated token account
        if custody_account.key != &custody_ata {
            // return Err(ProgramError::InvalidArgument);
            invoke(
                // &spl_associated_token_account::create_associated_token_account(
                &spl_associated_token_account::instruction::create_associated_token_account(
                    payer_account.key,
                    program_id, // Owner is the program
                    collateral_mint_account.key,
                    spl_account.key,
                ),
                &[
                    payer_account.clone(),
                    custody_account.clone(),
                    collateral_mint_account.clone(),
                    system_program.clone(),
                    spl_account.clone(),
                    associated_token_program.clone(),
                    rent_account.clone(),
                ],
            )?;
        } else {
            msg!("Custody Account matches expected Associated Token Account");
        }

        // Create a new Position
        let position = Position {
            owner: *payer_account.key,
            side,
            size_usd: amount,
            open_time: Clock::get()?.unix_timestamp,
            update_time: Clock::get()?.unix_timestamp,
            // Initialize other fields as needed
            ..Position::default()
        };

        // Add the new position to the positions vector
        user_positions.positions.push(position);

        // Serialize back to the account data
        user_positions.serialize(&mut *user_positions_data)?;

        msg!("Position added successfully");

        // Transfer collateral from user's account to custody account (associated token account)
        let transfer_ix = spl_token::instruction::transfer(
            spl_account.key,
            user_collateral_account.key,
            custody_account.key,
            payer_account.key,
            &[],
            amount,
        )?;

        msg!("Transferring collateral from user to custody account");

        invoke(
            &transfer_ix,
            &[
                user_collateral_account.clone(),
                custody_account.clone(),
                payer_account.clone(),
                spl_account.clone(),
            ],
        )?;

        msg!("Collateral transferred successfully");

        Ok(())
    }

    fn process_close_position(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        _position_id: u64,
    ) -> ProgramResult {
        // Implement logic to close a position
        msg!("Processing ClosePosition instruction");
        Ok(())
    }

    fn process_add_collateral(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        _position_id: u64,
        _amount: u64,
    ) -> ProgramResult {
        // Implement logic to add collateral to a position
        msg!("Processing AddCollateral instruction");
        Ok(())
    }

    fn process_remove_collateral(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        _position_id: u64,
        _amount: u64,
    ) -> ProgramResult {
        // Implement logic to remove collateral from a position
        msg!("Processing RemoveCollateral instruction");
        Ok(())
    }

    fn process_liquidate_position(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        _position_id: u64,
    ) -> ProgramResult {
        // Implement logic to liquidate a position
        msg!("Processing LiquidatePosition instruction");
        Ok(())
    }
}
