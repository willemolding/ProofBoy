use anyhow::Result;
use clap::Parser;
use ethers::abi::AbiDecode;
use ethers::prelude::*;
use extractor::{
    extractors::pokemon_red_blue_party_leader::PartyLeaderExtractor, Extractor, Metadata,
};
use gameboy::Gameboy;
use journal::{Journal, KeyState};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::collections::HashMap;
use preimage_provider::{PreimageKey, PreimageProvider};
use std::os::fd::FromRawFd;

mod cli;
mod gameboy;
mod preimage_provider;

// preimage file descriptors
const PCLIENT_RFD: i32 = 5;
const PCLIENT_WFD: i32 = 6;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args = cli::Cli::parse();

    let web3_provider = Provider::<Http>::try_from(args.rpc.as_str())?;
    let tx = web3_provider
        .get_transaction(args.txn_hash)
        .await?
        .ok_or(anyhow::anyhow!("Transaction not found"))?;

    let expected_metadata = metadata_from_calldata(&tx.input)?;
    let journal = witness_from_calldata(&tx.input)?;

    if args.serve_preimages { //serve data so MIPS emulator can verify
        let mut preimages =
            HashMap::<[u8; 32], Vec<u8>>::new();

            preimages.insert(PreimageKey::new_local(&[0x0]).into(), serde_json::to_string(&expected_metadata)?.into_bytes());
            preimages.insert(PreimageKey::new_local(&[0x1]).into(), journal.to_bytes());
        

        let reader = unsafe { File::from_raw_fd(PCLIENT_RFD) };
        let writer = unsafe { File::from_raw_fd(PCLIENT_WFD) };

        wait_for_requests(reader, writer, preimages).await?;

    } else { // verify natively
        // apply the journal to our emulator and get the final memory state
        let rom = include_bytes!("../../../roms/pokemon-blue.gb");
        let mut gb: Gameboy = Gameboy::new(rom);
    
        journal.into_iter().for_each(|keys| {
            gb.kbd.0.replace(KeyState::from_byte(keys));
            gb.step();
        });
    
        // extract using the given extractor and compare with the expected output
        let result_metadata =
            PartyLeaderExtractor::extract(&gb.sys).expect("Failed to extract metadata");
    
        assert_eq!(expected_metadata, result_metadata, "Metadata does not match!");
    
        println!("Verified successfully!");
    }


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
        log::debug!("Received key bytes: {:?}", &key_buffer);

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


fn metadata_from_calldata(calldata: &Bytes) -> Result<Metadata> {
    let decoded = ProposeMintCall::decode(calldata)?;
    let bytes: Vec<u8> = decoded.metadata_json.into();
    let metadata: Metadata = serde_json::from_slice(&bytes)?;
    Ok(metadata)
}

fn witness_from_calldata(calldata: &Bytes) -> Result<Journal> {
    let decoded = ProposeMintCall::decode(calldata)?;
    Ok(Journal::from_bytes(&decoded.witness.to_vec()))
}

abigen!(
    ERC1155ChallengeableMint,
    r#"[
        ProposeMint(address to, string calldata metadataJson, bytes calldata witness)
    ]"#,
);
