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

// OLD COMMENTED OUT IMPLEMENTATION
// impl VaultRegistry {
//     pub const MAX_VAULTS: usize = 10;
//     pub const LEN: usize = 8 + (32 * 5) * Self::MAX_VAULTS;
// }

// impl VaultRegistry {
//     pub const MAX_VAULTS: usize = 10;
//     pub const LEN: usize = 8 + (32 * 5) * Self::MAX_VAULTS + 4; // Add +4 for the length prefix of the vector
// }

impl VaultRegistry {
    pub const MAX_VAULTS: usize = 10;
    // pub const LEN: usize = 4 + (Vault::LEN * Self::MAX_VAULTS); // 4 bytes for vec length
}

impl Vault {
    pub const LEN: usize = 32 * 5; // 5 Pubkeys, each 32 bytes
}

impl VaultRegistry {
    // Add a vault if the length is less than MAX_VAULTS
    pub fn add_vault(&mut self, vault: Vault) -> Result<(), &'static str> {
        if self.vaults.len() >= Self::MAX_VAULTS {
            return Err("Max vaults reached");
        }
        self.vaults.push(vault);
        Ok(())
    }

    // Remove a vault by index
    pub fn remove_vault(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= self.vaults.len() {
            return Err("Index out of bounds");
        }
        self.vaults.remove(index);
        Ok(())
    }

    // Get the number of vaults
    pub fn vault_count(&self) -> usize {
        self.vaults.len()
    }

    // pub fn try_from_slice(data: &[u8]) -> Result<Self, std::io::Error> {
    //     let mut vaults = Vec::new();
    //     let mut cursor = std::io::Cursor::new(data);

    //     let vault_count = cursor.read_u32::<LittleEndian>()?;
    //     for _ in 0..vault_count {
    //         let vault = Vault::deserialize(&mut cursor)?;
    //         vaults.push(vault);
    //     }

    //     Ok(VaultRegistry { vaults })
    // }
}
