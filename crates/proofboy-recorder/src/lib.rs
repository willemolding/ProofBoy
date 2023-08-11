use app::{Gameboy, KeyJournal, SCALE, VRAM_HEIGHT, VRAM_WIDTH};
use bevy::{
    // diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    log::LogPlugin,
    prelude::*,
    window::*,
};
use extractor::{extractors::pokemon_red_blue_party_leader::PartyLeaderExtractor, Extractor};
use wasm_bindgen::prelude::*;

mod app;

const CYCLES_PER_FRAME: usize = 70224/5;

#[wasm_bindgen]
pub fn run(canvas_selector: &str, output_callback: js_sys::Function) {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "ProofBoy Recorder".to_string(),
                        resolution: WindowResolution::new(
                            VRAM_WIDTH as f32 * SCALE,
                            VRAM_HEIGHT as f32 * SCALE,
                        ),
                        canvas: Some(canvas_selector.to_string()),
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
        .insert_non_send_resource(output_callback)
        .add_systems(Update, check_for_dump)
        .run();
}

fn check_for_dump(
    gb: NonSend<Gameboy>,
    keys: Res<Input<KeyCode>>,
    mut journal: ResMut<KeyJournal>,
    callback: NonSend<js_sys::Function>,
) {
    if keys.just_pressed(KeyCode::Space) {
        journal.0.close();
        let metadata = PartyLeaderExtractor::extract(&gb.sys).expect("failed to extract metadata");
        log::info!("{:?}", &metadata);
        callback
            .call2(
                &JsValue::null(),
                &JsValue::from(
                    serde_json::to_string(&metadata).unwrap(),
                ),
                &JsValue::from(serde_json::to_string(&journal.0.clone().to_bytes()).unwrap()),
            )
            .unwrap();
    }
}
