use clap::Parser;
use ethers::prelude::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// URL of RPC endpoint
    #[arg(long)]
    pub rpc: url::Url,

    /// Hash of the mint transaction we want to check. Must be hex encoded 32 byte hash
    #[arg(long)]
    #[arg(value_parser = parse_txn_hash)]
    pub txn_hash: TxHash,
}

fn parse_txn_hash(arg: &str) -> Result<TxHash, anyhow::Error> {
    let mut tx_hash_bytes = [0_u8; 32];
    tx_hash_bytes.copy_from_slice(&hex::decode(arg)?);
    Ok(tx_hash_bytes.into())
}
