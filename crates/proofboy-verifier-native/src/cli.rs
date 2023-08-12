use clap::Parser;
use ethers::prelude::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// URL of RPC endpoint
    #[arg(long, env = "RPC_URL")]
    pub rpc: url::Url,

    /// Hash of the mint transaction we want to check. Must be hex encoded 32 byte hash
    #[arg(long, env = "TXN_HASH")]
    #[arg(value_parser = parse_txn_hash)]
    pub txn_hash: TxHash,

    #[arg(long, env = "SERVE_PREIMAGES")]
    pub serve_preimages: bool
}

fn parse_txn_hash(arg: &str) -> Result<TxHash, anyhow::Error> {
    let mut tx_hash_bytes = [0_u8; 32];
    if &arg[0..2] != "0x" {
        return Err(anyhow::anyhow!(
            "Transaction hash must be hex encoded with 0x prefix"
        ));
    }
    tx_hash_bytes.copy_from_slice(&hex::decode(&arg[2..])?);
    Ok(tx_hash_bytes.into())
}
