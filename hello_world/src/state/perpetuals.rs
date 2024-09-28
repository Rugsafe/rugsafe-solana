use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

pub const MAX_POSITIONS: usize = 10; // Max number of positions per user

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

#[derive(Default, Debug, Clone, BorshSerialize, BorshDeserialize)]
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
}

impl Position {
    pub const LEN: usize = 32 * 4 + 8 * 11 + 16 + 1; // Adjusted size calculation
}

#[derive(Default, Debug, BorshSerialize, BorshDeserialize)]
pub struct UserPositions {
    pub owner: Pubkey,
    pub positions: Vec<Position>, // Max length MAX_POSITIONS
}

impl UserPositions {
    pub const MAX_POSITIONS: usize = MAX_POSITIONS; // Max number of positions per user
    pub const LEN: usize = 32 + 4 + Position::LEN * Self::MAX_POSITIONS; // 32 bytes for owner, 4 bytes for vector length, rest for positions
}
