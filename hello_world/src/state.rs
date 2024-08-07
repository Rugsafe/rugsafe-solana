use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Vault {
    pub vault_account: Pubkey,
    pub mint_account: Pubkey,
    pub user_token_account: Pubkey,
    pub user_atoken_account: Pubkey,
    pub owner: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct VaultRegistry {
    pub vaults: Vec<Vault>,
}

impl VaultRegistry {
    pub const LEN: usize = 4 + (32 * 5) * 100; // Adjust this based on your needs
}
