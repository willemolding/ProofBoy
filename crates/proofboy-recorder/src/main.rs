use clap::Parser;
use journal::Journal;
use std::path::PathBuf;

use bevy::{
    // diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    log::LogPlugin,
    prelude::*,
    window::*,
};

use extractor::{extractors::pokemon_red_blue_party_leader::PartyLeaderExtractor, Extractor};

mod app;

/// Simple program to greet a person
#[derive(Parser, Debug, Resource, Clone)]
#[command(author, version, about)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    journal: Option<PathBuf>,

    /// Number of times to greet
    #[arg(short = 'o', long)]
    journal_out: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    // load journal from file if provided
    let journal = if let Some(journal) = args.journal.clone() {
        let journal_bytes = std::fs::read(journal).expect("failed to read journal from file");
        Some(Journal::from_bytes(&journal_bytes))
    } else {
        None
    };

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
            startup_journal: journal,
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
    if keys.pressed(KeyCode::Space) {
        journal.0.close();
        log::info!("{:?}", PartyLeaderExtractor::extract(&gb.sys));
        log::info!("{:?}", journal);
        let journal_bytes = journal.0.clone().to_bytes();
        if let Some(journal_out) = args.journal_out.clone() {
            std::fs::write(journal_out, journal_bytes).expect("failed to write journal to file");
        }
    }
}