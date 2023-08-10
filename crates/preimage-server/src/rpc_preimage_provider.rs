use anyhow::Result;
use ethers::prelude::*;
use ethers::{
    contract::abigen,
    core::{abi::AbiDecode, types::Bytes},
};
use std::collections::HashMap;

/// Connect to the given RPC and produce a pre-image map by extracting the calldata from the given transaction
pub async fn new_proof_preimages<T: Send + Sync + Into<TxHash>>(
    rpc_url: &url::Url,
    txn_hash: T,
) -> Result<HashMap<[u8; 32], Vec<u8>>> {
    let web3_provider = Provider::<Http>::try_from(rpc_url.as_str())?;
    let tx = web3_provider
        .get_transaction(txn_hash)
        .await?
        .ok_or(anyhow::anyhow!("Transaction not found"))?;

    let mut cache = HashMap::<[u8; 32], Vec<u8>>::new();

    let metadata = metadata_from_calldata(&tx.input)?;
    let witness = witness_from_calldata(&tx.input)?;

    cache.insert(PreimageKey::new_local(&[0x0]).into(), metadata);
    cache.insert(PreimageKey::new_local(&[0x1]).into(), witness);

    Ok(cache)
}

abigen!(
    ERC1155ChallengeableMint,
    r#"[
        ProposeMint(address to, string calldata metadataJson, bytes calldata witness)
    ]"#,
);

fn metadata_from_calldata(calldata: &Bytes) -> Result<Vec<u8>> {
    let decoded = ProposeMintCall::decode(calldata)?;
    log::debug!("metadata: {}", String::from(&decoded.metadata_json));
    Ok(decoded.metadata_json.into())
}

fn witness_from_calldata(calldata: &Bytes) -> Result<Vec<u8>> {
    let decoded = ProposeMintCall::decode(calldata)?;
    log::debug!("witness: {}", decoded.witness);
    Ok(hex::decode(decoded.witness)?)
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
