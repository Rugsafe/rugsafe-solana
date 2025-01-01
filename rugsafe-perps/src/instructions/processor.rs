use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        // Determine which module the instruction belongs to
        // Use the first byte to indicate the module
        let (module_tag, rest) = instruction_data
            .split_first()
            .ok_or(solana_program::program_error::ProgramError::InvalidInstructionData)?;

        match module_tag {
            0 => {
                // Perpetuals module
                // crate::instructions::perpetuals::Processor::process(program_id, accounts, rest)
                crate::instructions::perpetuals::processor::Processor::process(
                    program_id, accounts, rest,
                )
            }
            _ => Err(solana_program::program_error::ProgramError::InvalidInstructionData),
        }
    }
}
