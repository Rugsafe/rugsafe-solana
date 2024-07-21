use solana_program::program_error::ProgramError;

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum VaultInstruction {
    CreateVault,
    Deposit { amount: u64 },
    Withdraw { amount: u64 },
    BurnRToken { amount: u64 },
}

impl VaultInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        Ok(match tag {
            0 => Self::CreateVault,
            1 => {
                let amount = Self::unpack_amount(rest)?;
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
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(ProgramError::InvalidInstructionData)?;
        Ok(amount)
    }
}
