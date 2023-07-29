// use app::{Gameboy, KeyJournal, ProofBoyPlugin, SCALE, VRAM_HEIGHT, VRAM_WIDTH};
// use bevy::{
//     // diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
//     log::LogPlugin,
//     prelude::*,
//     window::*,
// };
// use extractor::{extractors::pokemon_red_blue_party_leader::PartyLeaderExtractor, Extractor};
// use journal::{Journal, KeyState};
// use wasm_bindgen::prelude::*;

// mod app;

// #[wasm_bindgen]
// pub fn run(canvas_selector: &str, output_callback: js_sys::Function, journal: Option<Vec<u8>>) {
//     let mut gb = Gameboy::default();

//     if let Some(journal) = journal {
//         let journal = Journal::from_bytes(&journal);
//         journal.into_iter().for_each(|keys| {
//             gb.kbd.0.replace(KeyState::from_byte(keys));
//             gb.step();
//         });
//     }

//     App::new()
//         .add_plugins(
//             DefaultPlugins
//                 .set(WindowPlugin {
//                     primary_window: Some(Window {
//                         title: "I am a window!".to_string(),
//                         resolution: WindowResolution::new(
//                             VRAM_WIDTH as f32 * SCALE,
//                             VRAM_HEIGHT as f32 * SCALE,
//                         ),
//                         canvas: Some(canvas_selector.to_string()),
//                         ..default()
//                     }),
//                     ..default()
//                 })
//                 .set(LogPlugin {
//                     filter: "rgy=error".into(),
//                     level: bevy::log::Level::INFO,
//                 }),
//         )
//         // .add_plugins((
//         //     LogDiagnosticsPlugin::default(),
//         //     FrameTimeDiagnosticsPlugin::default(),
//         // ))
//         .add_plugins(ProofBoyPlugin)
//         .insert_non_send_resource(output_callback)
//         .add_systems(Update, check_for_dump)
//         .run();
// }

// fn check_for_dump(
//     gb: NonSend<Gameboy>,
//     keys: Res<Input<KeyCode>>,
//     journal: Res<KeyJournal>,
//     callback: NonSend<js_sys::Function>,
// ) {
//     if keys.pressed(KeyCode::Space) {
//         log::info!("{:?}", PartyLeaderExtractor::extract(&gb.sys));
//         callback
//             .call2(
//                 &JsValue::null(),
//                 &JsValue::from(
//                     serde_json::to_string(&PartyLeaderExtractor::extract(&gb.sys)).unwrap(),
//                 ),
//                 &JsValue::from(serde_json::to_string(&journal.0.clone().to_bytes()).unwrap()),
//             )
//             .unwrap();
//     }
// }
