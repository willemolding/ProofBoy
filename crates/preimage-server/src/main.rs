use anyhow::Result;
use clap::Parser;
use log::debug;
use preimage_provider::PreimageProvider;
use std::collections::HashMap;
use std::os::fd::FromRawFd;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

mod cli;
mod preimage_provider;
mod rpc_preimage_provider;

// preimage file descriptors
const PCLIENT_RFD: i32 = 5;
const PCLIENT_WFD: i32 = 6;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    env_logger::init();
    let args = cli::Cli::parse();

    let preimage_provider = rpc_preimage_provider::new_proof_preimages(&args.rpc, args.txn_hash).await?;

    let reader = unsafe { File::from_raw_fd(PCLIENT_RFD) };
    let writer = unsafe { File::from_raw_fd(PCLIENT_WFD) };

    wait_for_requests(reader, writer, preimage_provider).await?;

    Ok(())
}

/// Infinitely wait for new requests to be forwarded from the emulator on the reader channel
/// On each received request try and retrieve a pre-image and send it to the guest on the writer channel
async fn wait_for_requests(
    mut reader: File,
    mut writer: File,
    preimages: impl PreimageProvider,
) -> Result<()> {
    loop {
        let mut key_buffer = [0; 32];
        reader.read(&mut key_buffer).await?;
        debug!("Received key bytes: {:?}", &key_buffer);

        if let Some(data) = preimages.get(&key_buffer) {
            // first it needs to write the length as a u64 big-endian
            let length: u64 = data.len() as u64;
            writer.write(&length.to_be_bytes()).await?;

            // then write the actual data
            writer.write(&data).await?;
        } else {
            panic!("Guest requested preimage that does not exist")
        }
    }
}
