use clap::Parser;
use std::path::PathBuf;

use bevy::{
    // diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    log::LogPlugin,
    prelude::*,
    window::*,
};

mod app;

/// Simple program to greet a person
#[derive(Parser, Debug)]
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
    let _args = Args::parse();

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
        .add_plugins(app::ProofBoyPlugin)
        .run();
}
