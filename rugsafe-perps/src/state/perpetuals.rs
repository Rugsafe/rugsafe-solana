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

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, Default)]
pub struct Position {
    pub owner: Pubkey, // The public key of the user who owns this position. It uniquely identifies the user.

    pub pool: Pubkey, // Represents the liquidity pool in which this position operates. It's used to link the position to a specific market or trading pool.

    pub custody: Pubkey, // The public key of the custody account holding the assets for this position. Custody ensures that assets are stored securely.

    pub collateral_custody: Pubkey, // The public key of the collateral custody account. This holds the collateral deposited to secure the position, protecting against liquidation.

    pub open_time: i64, // Timestamp indicating when the position was first opened. It records the exact moment the position was initiated.

    pub update_time: i64, // Timestamp of the most recent update to the position. It changes whenever new collateral is added, or a partial close happens.

    pub side: Side, // Indicates whether the position is long (betting on price going up) or short (betting on price going down).

    pub price: u64, // The price at which the position was opened. This is the reference price used to calculate profit and loss.

    pub size_usd: u64, // The total size of the position in USD. Represents the notional value of the position.

    pub borrow_size_usd: u64, // The amount borrowed in USD to leverage the position. Borrowing increases potential returns but also risk.

    pub collateral_usd: u64, // The USD value of the collateral held for this position. This collateral is used to maintain the position and mitigate liquidation risk.

    pub unrealized_profit_usd: u64, // The potential profit on the position if it were to be closed at the current market price. It is 'unrealized' because the position is still open.

    pub unrealized_loss_usd: u64, // The potential loss on the position if it were to be closed at the current market price. This is the risk side, also 'unrealized' because the position is still open.

    pub cumulative_interest_snapshot: u128, // Tracks the cumulative interest accumulated on the borrowed amount. Useful for calculating interest owed over time.

    pub locked_amount: u64, // The amount of collateral or liquidity locked in the position. This portion of collateral is inaccessible until the position is closed.

    pub collateral_amount: u64, // The total amount of collateral provided for the position. It includes both locked collateral and any excess collateral that can be withdrawn or adjusted.
}

impl Position {
    pub const LEN: usize = 32 * 4 + 8 * 11 + 16 + 1; // Adjusted size calculation
}

#[derive(Default, Debug, BorshSerialize, BorshDeserialize)]
pub struct UserPositions {
    pub owner: Pubkey,          // Owner's public key, no change
    pub next_position_idx: u64, // Pointer to the next position index
}

impl UserPositions {
    pub const LEN: usize = 32 + 8; // 32 bytes for owner, 8 bytes for position index
}
