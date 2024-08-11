use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use std::io::Read;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BorshPubkey(pub Pubkey);

impl BorshSerialize for BorshPubkey {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(self.0.as_ref())
    }
}

impl BorshDeserialize for BorshPubkey {
    fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut pubkey_bytes = [0u8; 32];
        buf.read_exact(&mut pubkey_bytes)?;
        Ok(BorshPubkey(Pubkey::new_from_array(pubkey_bytes)))
    }

    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let mut pubkey_bytes = [0u8; 32];
        reader.read_exact(&mut pubkey_bytes)?;
        Ok(BorshPubkey(Pubkey::new_from_array(pubkey_bytes)))
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Vault {
    pub vault_account: BorshPubkey,
    pub mint_account: BorshPubkey,
    pub user_token_account: BorshPubkey,
    pub user_atoken_account: BorshPubkey,
    pub owner: BorshPubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct VaultRegistry {
    pub vaults: Vec<Vault>,
}

impl VaultRegistry {
    pub const MAX_VAULTS: usize = 100; // Example maximum
    pub const LEN: usize = 8 + (32 * 5) * Self::MAX_VAULTS; // Adjust based on your needs
}
