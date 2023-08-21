pub trait PreimageProvider {
    fn get(&self, key: &[u8; 32]) -> Option<Vec<u8>>;
}

impl PreimageProvider for std::collections::HashMap<[u8; 32], Vec<u8>> {
    fn get(&self, key: &[u8; 32]) -> Option<Vec<u8>> {
        self.get(key).cloned()
    }
}

#[derive(Debug, Default, Clone, Copy)]
/// Types of preimage oracle keys. See https://github.com/ethereum-optimism/optimism/blob/develop/specs/fault-proof.md#pre-image-key-types
pub enum KeyType {
    /// Local key types are local and context dependent.
    /// E.g. they might be different depending on the details of what is running the program and why
    #[default]
    Local = 1,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct PreimageKey {
    pub key_type: KeyType,
    pub x: [u8; 31],
}

impl PreimageKey {
    pub fn new(key_type: KeyType, x: [u8; 31]) -> Self {
        Self { key_type, x }
    }

    /// This will panic if using a slice greater than 31 bytes
    pub fn new_local(x: &[u8]) -> Self {
        let mut key_bytes = [0_u8; 31];
        // slice is inserted at the end
        key_bytes[31 - x.len()..31].clone_from_slice(x);
        Self::new(KeyType::Local, key_bytes)
    }
}

impl From<PreimageKey> for [u8; 32] {
    fn from(key: PreimageKey) -> Self {
        let mut result = [0; 32];
        result[0] = key.key_type as u8;
        result[1..].copy_from_slice(&key.x);
        result
    }
}
