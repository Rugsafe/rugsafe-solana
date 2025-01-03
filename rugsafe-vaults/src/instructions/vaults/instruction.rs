use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::msg;
use solana_program::program_error::ProgramError;
#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize)]
pub enum VaultInstruction {
    CreateVault,
    Deposit { amount: u64 },
    Withdraw { amount: u64 },
    BurnRToken { amount: u64 },
    Faucet { amount: u64 },
}

impl VaultInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        msg!("input: {:?}", hex::encode(input));

        let (&tag, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        msg!("tag: {}", tag);
        msg!("Rest: {:?}", hex::encode(rest));

        Ok(match tag {
            0 => Self::CreateVault,
            1 => {
                let amount = Self::unpack_amount(rest)?;
                msg!("amount from inside unpack 1: {}", &amount.to_string());
                Self::Deposit { amount }
            }
            2 => {
                let amount = Self::unpack_amount(rest)?;
                Self::Withdraw { amount }
            }
            3 => {
                let amount = Self::unpack_amount(rest)?;
                Self::BurnRToken { amount }
            }
            4 => {
                let amount = Self::unpack_amount(rest)?;
                msg!("amount from inside unpack 2: {}", &amount.to_string());
                Self::Faucet { amount }
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(ProgramError::InvalidInstructionData)?;
        msg!("amount from inside unpack_amount: {}", amount);
        Ok(amount)
    }
}
