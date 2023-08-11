use clap::Parser;
use std::path::PathBuf;

use bevy::{
    // diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    log::LogPlugin,
    prelude::*,
    window::*,
};

use extractor::{extractors::pokemon_red_blue_party_leader::PartyLeaderExtractor, Extractor};

const CYCLES_PER_FRAME: usize = 70224/5;

mod app;

/// Simple program to greet a person
#[derive(Parser, Debug, Resource, Clone)]
#[command(author, version, about)]
struct Args {
    /// place to write the output journal to
    #[arg(short = 'o', long)]
    journal_out: Option<PathBuf>,

    /// place to write the metadata extracted from memory which can be used to mint an NFT
    #[arg(short = 'm', long)]
    metadata_out: Option<PathBuf>,

    /// place to write the RAM dump which can be used to restart the emulator from the given point
    #[arg(short = 'r', long)]
    ram_out: Option<PathBuf>,
}


fn main() {
    let args = Args::parse();

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "ProofBoy Recorder".to_string(),
                        resolution: WindowResolution::new(
                            app::VRAM_WIDTH as f32 * app::SCALE,
                            app::VRAM_HEIGHT as f32 * app::SCALE,
                        ),
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    filter: "rgy=error".into(),
                    level: bevy::log::Level::INFO,
                }),
        )
        // .add_plugins((
        //     LogDiagnosticsPlugin::default(),
        //     FrameTimeDiagnosticsPlugin::default(),
        // ))
        .add_plugins(app::ProofBoyPlugin {
            rom: include_bytes!("../../../roms/pokemon-blue.gb").to_vec(),
            startup_journal: None,
            cycles_per_frame: CYCLES_PER_FRAME,
        })
        .insert_resource(args)
        .add_systems(Update, check_for_dump)
        .run();
}

fn check_for_dump(
    gb: NonSend<app::Gameboy>,
    keys: Res<Input<KeyCode>>,
    mut journal: ResMut<app::KeyJournal>,
    args: Res<Args>,
) {
    if keys.just_pressed(KeyCode::Space) {
        journal.0.close();
        let metadata = PartyLeaderExtractor::extract(&gb.sys).expect("failed to extract metadata");
        log::info!("{:?}", &metadata);
        log::info!("{:?}", journal);
        let journal_bytes = journal.0.clone().to_bytes();
        if let Some(journal_out) = args.journal_out.clone() {
            std::fs::write(journal_out, journal_bytes).expect("failed to write journal to file");
        }
        if let Some(metadata_out) = args.metadata_out.clone() {
            std::fs::write(
                metadata_out,
                serde_json::to_string(&metadata).expect("Failed to serialize metadata"),
            )
            .expect("failed to write journal to file");
        }
        if let Some(ram_out) = args.ram_out.clone() {
            std::fs::write(
                ram_out,
                gb.sys.mmu_dump(),
            )
            .expect("failed to write journal to file");
        }
    }
}
