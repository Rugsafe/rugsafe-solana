use crate::state::perpetuals::Side;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize)]
pub enum PerpetualsInstruction {
    OpenPosition { side: Side, amount: u64 },
    ClosePosition { position_id: u64 },
    AddCollateral { position_id: u64, amount: u64 },
    RemoveCollateral { position_id: u64, amount: u64 },
    LiquidatePosition { position_id: u64 },
}

impl PerpetualsInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match tag {
            0 => {
                let (side_byte, rest) = rest
                    .split_first()
                    .ok_or(ProgramError::InvalidInstructionData)?;
                let side = match side_byte {
                    1 => Side::Long,
                    2 => Side::Short,
                    _ => return Err(ProgramError::InvalidInstructionData),
                };
                let amount = Self::unpack_amount(rest)?;
                Self::OpenPosition { side, amount }
            }
            1 => {
                let position_id = Self::unpack_u64(rest)?;
                Self::ClosePosition { position_id }
            }
            2 => {
                let position_id = Self::unpack_u64(rest)?;
                let amount = Self::unpack_amount(&rest[8..])?;
                Self::AddCollateral {
                    position_id,
                    amount,
                }
            }
            3 => {
                let position_id = Self::unpack_u64(rest)?;
                let amount = Self::unpack_amount(&rest[8..])?;
                Self::RemoveCollateral {
                    position_id,
                    amount,
                }
            }
            4 => {
                let position_id = Self::unpack_u64(rest)?;
                Self::LiquidatePosition { position_id }
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }

    fn unpack_u64(input: &[u8]) -> Result<u64, ProgramError> {
        input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(ProgramError::InvalidInstructionData)
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        Self::unpack_u64(input)
    }
}
