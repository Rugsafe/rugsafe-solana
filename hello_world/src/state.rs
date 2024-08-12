use solana_program::pubkey::Pubkey;

#[derive(Debug, PartialEq)]
pub struct Vault {
    pub vault_account: Pubkey,
    pub mint_account: Pubkey,
    pub owner: Pubkey,
}

#[derive(Debug, PartialEq)]
pub struct VaultRegistry {
    pub vaults: Vec<Vault>,
}

impl Vault {
    pub const LEN: usize = 32 * 3; // 5 Pubkeys, each 32 bytes

    /*
    @name serialize
    @description Serializes a `Vault` instance into a vector of bytes, converting the internal Pubkey fields into a byte representation.
    @param &self - A reference to the `Vault` instance being serialized.
    */
    pub fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(Self::LEN);
        data.extend_from_slice(self.vault_account.as_ref());
        data.extend_from_slice(self.mint_account.as_ref());
        data.extend_from_slice(self.owner.as_ref());
        data
    }

    /*
    @name deserialize
    @description Deserializes a slice of bytes into a `Vault` instance, reconstructing the internal Pubkey fields from their byte representation.
    @param input - A byte slice representing the serialized `Vault` data.
    */
    pub fn deserialize(input: &[u8]) -> Self {
        let vault_account = Pubkey::new_from_array(input[0..32].try_into().unwrap());
        let mint_account = Pubkey::new_from_array(input[32..64].try_into().unwrap());
        let owner = Pubkey::new_from_array(input[64..96].try_into().unwrap());

        Vault {
            vault_account,
            mint_account,
            owner,
        }
    }
}

impl VaultRegistry {
    pub const MAX_VAULTS: usize = 10;
    pub const LEN: usize = 4 + (Vault::LEN * Self::MAX_VAULTS); // 4 bytes for vec length

    /*
    @name serialize
    @description Serializes a `VaultRegistry` instance into a vector of bytes, converting the internal vault list into a byte representation.
    @param &self - A reference to the `VaultRegistry` instance being serialized.
    */

    pub fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(Self::LEN);
        let vaults_len = self.vaults.len() as u32;
        data.extend_from_slice(&vaults_len.to_le_bytes());

        for vault in &self.vaults {
            data.extend_from_slice(&vault.serialize());
        }

        // Pad the remaining space with zeros
        data.resize(Self::LEN, 0);

        data
    }

    /*
    @name deserialize
    @description Deserializes a slice of bytes into a `VaultRegistry` instance, reconstructing the internal vault list from its byte representation.
    @param input - A byte slice representing the serialized `VaultRegistry` data.
    */
    pub fn deserialize(input: &[u8]) -> Result<Self, &'static str> {
        let (len_bytes, mut rest) = input.split_at(4);
        let vaults_len = u32::from_le_bytes(len_bytes.try_into().unwrap()) as usize;

        if vaults_len * Vault::LEN > rest.len() {
            return Err("Input data is too short for deserialization");
        }

        let mut vaults = Vec::with_capacity(vaults_len);
        for _ in 0..vaults_len {
            let (vault_bytes, remaining) = rest.split_at(Vault::LEN);
            vaults.push(Vault::deserialize(vault_bytes));
            rest = remaining;
        }

        Ok(VaultRegistry { vaults })
    }

    /*
    @name add_vault
    @description Adds a new `Vault` to the `VaultRegistry` if the number of vaults is less than the maximum allowed. Returns an error if the maximum is reached.
    @param &mut self - A mutable reference to the `VaultRegistry` instance.
    @param vault - The `Vault` instance to be added to the registry.
    */
    pub fn add_vault(&mut self, vault: Vault) -> Result<(), &'static str> {
        if self.vaults.len() >= Self::MAX_VAULTS {
            return Err("Max vaults reached");
        }
        self.vaults.push(vault);
        Ok(())
    }

    /*
    @name remove_vault
    @description Removes a `Vault` from the `VaultRegistry` by its index. Returns an error if the index is out of bounds.
    @param &mut self - A mutable reference to the `VaultRegistry` instance.
    @param index - The index of the `Vault` to be removed.
    */
    pub fn remove_vault(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= self.vaults.len() {
            return Err("Index out of bounds");
        }
        self.vaults.remove(index);
        Ok(())
    }

    /*
    @name vault_count
    @description Returns the number of vaults currently stored in the `VaultRegistry`.
    @param &self - A reference to the `VaultRegistry` instance.
    */
    pub fn vault_count(&self) -> usize {
        self.vaults.len()
    }
}
