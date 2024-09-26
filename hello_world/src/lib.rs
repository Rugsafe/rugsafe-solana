// pub mod instruction;
pub mod instructions;
// pub mod processor;
// pub mod instructions::
pub mod state;

// deterministically designate program ID
// declare_id!("FobNvbQsK5BAniZC2oJhXakjcPiArpsthTGDnX9eHDVY");

use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // instructions::processor::Processor::process(program_id, accounts, instruction_data)
    crate::instructions::processor::Processor::process(program_id, accounts, instruction_data)
}
