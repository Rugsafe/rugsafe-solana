use solana_program::pubkey::Pubkey;

#[derive(Debug, PartialEq)]
pub struct Vault {
    pub vault_account: Pubkey,
    pub mint_token_a: Pubkey,
    pub mint_a_token_a: Pubkey,
    pub owner: Pubkey,
}

#[derive(Debug, PartialEq)]
pub struct VaultRegistry {
    pub vaults: Vec<Vault>,
    pub capacity: usize,
}

impl Vault {
    pub const LEN: usize = 32 * 4; // 4 Pubkeys, each 32 bytes

    pub fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(Self::LEN);
        data.extend_from_slice(self.vault_account.as_ref());
        data.extend_from_slice(self.mint_token_a.as_ref());
        data.extend_from_slice(self.mint_a_token_a.as_ref());
        data.extend_from_slice(self.owner.as_ref());
        data
    }

    pub fn deserialize(input: &[u8]) -> Self {
        let vault_account = Pubkey::new_from_array(input[0..32].try_into().unwrap());
        let mint_token_a = Pubkey::new_from_array(input[32..64].try_into().unwrap());
        let mint_a_token_a = Pubkey::new_from_array(input[64..96].try_into().unwrap());
        let owner = Pubkey::new_from_array(input[96..128].try_into().unwrap());

        Vault {
            vault_account,
            mint_token_a,
            mint_a_token_a,
            owner,
        }
    }
}

impl VaultRegistry {
    pub const INITIAL_CAPACITY: usize = 10;

    pub fn new() -> Self {
        VaultRegistry {
            vaults: Vec::new(),
            capacity: Self::INITIAL_CAPACITY,
        }
    }

    pub fn len(&self) -> usize {
        8 + 8 + (Vault::LEN * self.capacity) // 8 bytes for vec length, 8 bytes for capacity
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(self.len());
        let vaults_len = self.vaults.len() as u64;
        data.extend_from_slice(&vaults_len.to_le_bytes());
        data.extend_from_slice(&(self.capacity as u64).to_le_bytes());

        for vault in &self.vaults {
            data.extend_from_slice(&vault.serialize());
        }

        // Pad the remaining space with zeros
        data.resize(self.len(), 0);

        data
    }

    pub fn deserialize(input: &[u8]) -> Result<Self, &'static str> {
        if input.len() < 16 {
            return Err("Input data is too short for deserialization");
        }

        let (len_bytes, rest) = input.split_at(8);
        let vaults_len = u64::from_le_bytes(len_bytes.try_into().unwrap()) as usize;

        let (capacity_bytes, mut rest) = rest.split_at(8);
        let capacity = u64::from_le_bytes(capacity_bytes.try_into().unwrap()) as usize;

        if vaults_len > capacity {
            return Err("Invalid data: vault count exceeds capacity");
        }

        let mut vaults = Vec::with_capacity(vaults_len);
        for _ in 0..vaults_len {
            let (vault_bytes, remaining) = rest.split_at(Vault::LEN);
            vaults.push(Vault::deserialize(vault_bytes));
            rest = remaining;
        }

        Ok(VaultRegistry { vaults, capacity })
    }

    pub fn add_vault(&mut self, vault: Vault) -> Result<(), &'static str> {
        if self.vaults.len() >= self.capacity {
            return Err("Max capacity reached. Reallocation needed.");
        }
        self.vaults.push(vault);
        Ok(())
    }

    pub fn remove_vault(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= self.vaults.len() {
            return Err("Index out of bounds");
        }
        self.vaults.remove(index);
        Ok(())
    }

    pub fn vault_count(&self) -> usize {
        self.vaults.len()
    }

    pub fn grow(&mut self) {
        self.capacity *= 2;
    }
}
