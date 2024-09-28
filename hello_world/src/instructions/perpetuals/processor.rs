use crate::instructions::perpetuals::PerpetualsInstruction;
use crate::state::perpetuals::{Position, Side, UserPositions};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    clock::Clock,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::Sysvar,
};

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

    // fn process_open_position(
    //     program_id: &Pubkey,
    //     accounts: &[AccountInfo],
    //     side: Side,
    //     amount: u64,
    // ) -> ProgramResult {
    //     msg!("Processing OpenPosition instruction");

    //     let account_info_iter = &mut accounts.iter();

    //     let payer_account = next_account_info(account_info_iter)?; // User who opens the position
    //     let position_account = next_account_info(account_info_iter)?; // PDA for the position
    //     let user_collateral_account = next_account_info(account_info_iter)?; // User's collateral token account
    //     let custody_account = next_account_info(account_info_iter)?; // Vault's custody account
    //     let token_program = next_account_info(account_info_iter)?; // Token program
    //     let system_program = next_account_info(account_info_iter)?; // System program
    //     let rent_account = next_account_info(account_info_iter)?; // Rent sysvar

    //     msg!("Payer account: {}", payer_account.key);
    //     msg!("Position account: {}", position_account.key);
    //     msg!("User collateral account: {}", user_collateral_account.key);
    //     msg!("Custody account: {}", custody_account.key);

    //     // Ensure the payer is a signer
    //     if !payer_account.is_signer {
    //         return Err(ProgramError::MissingRequiredSignature);
    //     }

    //     // Derive the expected position PDA
    //     let (expected_position_pubkey, bump_seed) =
    //         Pubkey::find_program_address(&[b"position", payer_account.key.as_ref()], program_id);

    //     msg!("Expected position PDA: {}", expected_position_pubkey);

    //     if position_account.key != &expected_position_pubkey {
    //         msg!("Invalid position account");
    //         return Err(ProgramError::InvalidArgument);
    //     }

    //     // Create the position account if it's empty
    //     if position_account.data_is_empty() {
    //         msg!("Position account is empty, creating account");

    //         let rent = &solana_program::rent::Rent::from_account_info(rent_account)?;
    //         let required_lamports = rent.minimum_balance(Position::LEN);
    //         msg!(
    //             "Required lamports for position account: {}",
    //             required_lamports
    //         );

    //         let seeds = &[b"position", payer_account.key.as_ref(), &[bump_seed]];

    //         // Create the position account using the PDA
    //         match solana_program::program::invoke_signed(
    //             &solana_program::system_instruction::create_account(
    //                 payer_account.key,
    //                 position_account.key,
    //                 required_lamports,
    //                 Position::LEN as u64,
    //                 program_id,
    //             ),
    //             &[
    //                 payer_account.clone(),
    //                 position_account.clone(),
    //                 system_program.clone(),
    //             ],
    //             &[seeds],
    //         ) {
    //             Ok(_) => msg!("Position account created successfully"),
    //             Err(e) => {
    //                 msg!("Error creating position account: {}", e);
    //                 return Err(e);
    //             }
    //         }
    //     } else {
    //         msg!("Position account already exists");
    //     }

    //     // Initialize the Position struct
    //     let mut position_data = position_account.try_borrow_mut_data()?;
    //     let mut position = Position::try_from_slice(&position_data).unwrap_or_default();
    //     position.owner = *payer_account.key;
    //     position.side = side;
    //     position.size_usd = amount;
    //     position.open_time = Clock::get()?.unix_timestamp;
    //     position.bump = bump_seed;

    //     // Serialize the position data into the account data
    //     position.serialize(&mut *position_data)?;
    //     msg!("Position data serialized successfully");

    //     // Transfer collateral from user's account to custody account
    //     let transfer_ix = spl_token::instruction::transfer(
    //         token_program.key,
    //         user_collateral_account.key,
    //         custody_account.key,
    //         payer_account.key,
    //         &[],
    //         amount,
    //     )?;

    //     msg!("Transferring collateral from user to custody account");

    //     match solana_program::program::invoke(
    //         &transfer_ix,
    //         &[
    //             user_collateral_account.clone(),
    //             custody_account.clone(),
    //             payer_account.clone(),
    //             token_program.clone(),
    //         ],
    //     ) {
    //         Ok(_) => msg!("Collateral transferred successfully"),
    //         Err(e) => {
    //             msg!("Error transferring collateral: {}", e);
    //             return Err(e);
    //         }
    //     }

    //     msg!("Position opened successfully");

    //     Ok(())
    // }
    fn process_open_position(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        side: Side,
        amount: u64,
    ) -> ProgramResult {
        msg!("Processing OpenPosition instruction");

        let account_info_iter = &mut accounts.iter();

        let payer_account = next_account_info(account_info_iter)?; // User who opens the position
        let user_positions_account = next_account_info(account_info_iter)?; // User's positions account
        let user_collateral_account = next_account_info(account_info_iter)?; // User's collateral token account
        let custody_account = next_account_info(account_info_iter)?; // Vault's custody account
        let token_program = next_account_info(account_info_iter)?; // Token program

        // Ensure the payer is a signer
        if !payer_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Ensure the user_positions_account is owned by the program
        if user_positions_account.owner != program_id {
            msg!("User positions account not owned by the program");
            return Err(ProgramError::IllegalOwner);
        }

        // Deserialize the UserPositions account data
        let mut user_positions_data = user_positions_account.try_borrow_mut_data()?;
        let mut data_slice: &[u8] = &user_positions_data;

        // let mut user_positions: UserPositions;
        let mut user_positions = if data_slice.iter().all(|&x| x == 0) {
            // Account data is uninitialized, initialize UserPositions
            UserPositions {
                owner: *payer_account.key,
                positions: Vec::new(),
            }
        } else {
            // Account data is initialized, deserialize
            let mut user_positions = UserPositions::deserialize(&mut data_slice)?;

            // If the owner is default (zeros), set owner
            if user_positions.owner == Pubkey::default() {
                user_positions.owner = *payer_account.key;
            } else if user_positions.owner != *payer_account.key {
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

        // Transfer collateral from user's account to custody account
        let transfer_ix = spl_token::instruction::transfer(
            token_program.key,
            user_collateral_account.key,
            custody_account.key,
            payer_account.key,
            &[],
            amount,
        )?;

        msg!("Transferring collateral from user to custody account");

        solana_program::program::invoke(
            &transfer_ix,
            &[
                user_collateral_account.clone(),
                custody_account.clone(),
                payer_account.clone(),
                token_program.clone(),
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
