use anyhow::Result;
use clap::Parser;
use ethers::abi::AbiDecode;
use ethers::prelude::*;
use extractor::{
    extractors::pokemon_red_blue_party_leader::PartyLeaderExtractor, Extractor, Metadata,
};
use gameboy::Gameboy;
use journal::{Journal, KeyState};

mod cli;
mod gameboy;

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

    if expected_metadata == result_metadata {
        println!("Verified successfully!");
    } else {
        println!("Metadata does not match!");
    }

    Ok(())
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
