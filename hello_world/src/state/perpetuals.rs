use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(Copy, Clone, PartialEq, Debug, BorshSerialize, BorshDeserialize)]
pub enum Side {
    None,
    Long,
    Short,
}

impl Default for Side {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Default, Debug, BorshSerialize, BorshDeserialize)]
pub struct Position {
    pub owner: Pubkey,
    pub pool: Pubkey,
    pub custody: Pubkey,
    pub collateral_custody: Pubkey,
    pub open_time: i64,
    pub update_time: i64,
    pub side: Side,
    pub price: u64,
    pub size_usd: u64,
    pub borrow_size_usd: u64,
    pub collateral_usd: u64,
    pub unrealized_profit_usd: u64,
    pub unrealized_loss_usd: u64,
    pub cumulative_interest_snapshot: u128,
    pub locked_amount: u64,
    pub collateral_amount: u64,
    pub bump: u8,
}

impl Position {
    pub const LEN: usize = 32 * 4 + 8 * 8 + 16 + 1 + 1; // todo calc final size
}
