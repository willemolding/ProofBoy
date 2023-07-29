use bevy::{
    // diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    log::LogPlugin,
    prelude::*,
    window::*,
};
use extractor::{extractors::pokemon_red_blue_party_leader::PartyLeaderExtractor, Extractor};
use journal::{Journal, KeyState};
use log;
use rgy::{debug::NullDebugger, Config, Key as GBKey, Stream, System, VRAM_HEIGHT, VRAM_WIDTH};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

const SCALE: f32 = 2.0;
const CYCLES_PER_FRAME: usize = 70224;

#[derive(Resource, Default)]
struct KeyJournal(pub Journal);

#[wasm_bindgen]
pub fn run(canvas_selector: &str, output_callback: js_sys::Function, journal: Option<Vec<u8>>) {
    let mut gb = Gameboy::default();

    if let Some(journal) = journal {
        let journal = Journal::from_bytes(&journal);
        journal.into_iter().for_each(|keys| {
            gb.kbd.0.replace(KeyState::from_byte(keys));
            gb.step();
        });
    }

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "I am a window!".to_string(),
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
        .init_resource::<KeyJournal>()
        .insert_non_send_resource(gb)
        .insert_non_send_resource(output_callback)
        .add_systems(Startup, setup_screen)
        .add_systems(Update, (update_gameboy, update_screen, check_for_dump))
        .run();


}


fn setup_screen(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    // spawn all the pixels
    for x in 0..VRAM_WIDTH {
        for y in 0..VRAM_HEIGHT {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::BLACK,
                        custom_size: Some(Vec2::new(SCALE, SCALE)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(
                        SCALE * ((x as f32) - (VRAM_WIDTH as f32) / 2.),
                        -SCALE * ((y as f32) - (VRAM_HEIGHT as f32) / 2.),
                        0.,
                    )),
                    ..default()
                },
                ScreenPosition { x, y },
            ));
        }
    }
}

fn update_gameboy(
    mut gb: NonSendMut<Gameboy>,
    keys: Res<Input<KeyCode>>,
    mut journal: ResMut<KeyJournal>,
    mut focus_changed: EventReader<WindowFocused>,
) {
    gb.kbd.0.borrow_mut().up = keys.pressed(KeyCode::Up);
    gb.kbd.0.borrow_mut().down = keys.pressed(KeyCode::Down);
    gb.kbd.0.borrow_mut().left = keys.pressed(KeyCode::Left);
    gb.kbd.0.borrow_mut().right = keys.pressed(KeyCode::Right);
    gb.kbd.0.borrow_mut().a = keys.pressed(KeyCode::X);
    gb.kbd.0.borrow_mut().b = keys.pressed(KeyCode::Z);
    gb.kbd.0.borrow_mut().start = keys.pressed(KeyCode::Return);
    gb.kbd.0.borrow_mut().select = keys.pressed(KeyCode::ShiftRight);

    let gb = gb.as_mut();
    
    for event in focus_changed.iter() {
        log::info!("Focus changed: {:?}", event);
        gb.active = event.focused;
    }

    if gb.active {
        for _ in 0..CYCLES_PER_FRAME {
            journal.0.tick(gb.cycle_count, gb.kbd.0.borrow().as_byte());
            gb.step();
        }
    }

}

fn check_for_dump(
    gb: NonSend<Gameboy>,
    keys: Res<Input<KeyCode>>,
    journal: Res<KeyJournal>,
    callback: NonSend<js_sys::Function>,
) {
    if keys.pressed(KeyCode::Space) {
        log::info!("{:?}", PartyLeaderExtractor::extract(&gb.sys));
        callback.call2(
            &JsValue::null(),
            &JsValue::from(serde_json::to_string(&PartyLeaderExtractor::extract(&gb.sys)).unwrap()),
            &JsValue::from(serde_json::to_string(&journal.0.clone().to_bytes()).unwrap())
        ).unwrap();
    }
}

fn update_screen(gb: NonSend<Gameboy>, mut query: Query<(&ScreenPosition, &mut Sprite)>) {
    for (screen_pos, mut sprite) in query.iter_mut() {
        let p = gb.display.0.borrow()[screen_pos.x][screen_pos.y];
        sprite.color = Color::rgb_u8(
            (p & 0xffu32) as u8,
            ((p >> 8) & 0xffu32) as u8,
            ((p >> 16) & 0xffu32) as u8,
        );
    }
}

#[derive(Debug, Component)]
struct ScreenPosition {
    x: usize,
    y: usize,
}

struct Gameboy {
    sys: System<NullDebugger>,
    display: Display,
    kbd: Keyboard,
    cycle_count: u64,
    active: bool,
}

impl Gameboy {
    pub fn step(&mut self) {
        self.sys.poll();
        self.cycle_count += 1;
    }
}

impl Default for Gameboy {
    fn default() -> Self {
        let kbd = Keyboard::new();
        let display = Display::new();
        let cfg = Config::new().native_speed(true);
        let hw = Hardware::new(display.clone(), kbd.clone());

        let rom = include_bytes!("../../../roms/pokemon-blue.gb");

        let sys = System::new(cfg, rom, hw, NullDebugger);

        Self {
            sys,
            display,
            kbd,
            cycle_count: 0,
            active: false,
        }
    }
}

#[derive(Clone, Debug)]
struct Display(Rc<RefCell<Vec<Vec<u32>>>>);

#[derive(Clone, Debug)]
struct Keyboard(Rc<RefCell<KeyState>>);

impl Keyboard {
    fn new() -> Self {
        Self(Rc::new(RefCell::new(KeyState::default())))
    }
}

impl Display {
    fn new() -> Self {
        Self(Rc::new(RefCell::new(vec![
            vec![0u32; VRAM_HEIGHT];
            VRAM_WIDTH
        ])))
    }
}

struct Hardware {
    display: Display,
    kbd: Keyboard,
}

impl Hardware {
    fn new(display: Display, kbd: Keyboard) -> Self {
        Self { display, kbd }
    }
}

impl rgy::Hardware for Hardware {
    fn vram_update(&mut self, line: usize, buffer: &[u32]) {
        let y = line;

        for (x, col) in buffer.iter().enumerate() {
            self.display.0.borrow_mut()[x][y] = *col;
        }
    }

    fn joypad_pressed(&mut self, key: GBKey) -> bool {
        match key {
            GBKey::Up => self.kbd.0.borrow().up,
            GBKey::Down => self.kbd.0.borrow().down,
            GBKey::Left => self.kbd.0.borrow().left,
            GBKey::Right => self.kbd.0.borrow().right,
            GBKey::A => self.kbd.0.borrow().a,
            GBKey::B => self.kbd.0.borrow().b,
            GBKey::Start => self.kbd.0.borrow().start,
            GBKey::Select => self.kbd.0.borrow().select,
        }
    }

    fn sound_play(&mut self, _stream: Box<dyn Stream>) {}

    fn clock(&mut self) -> u64 {
        0 // This was nerfing the performance!!
          // let epoch = std::time::SystemTime::now()
          //     .duration_since(std::time::UNIX_EPOCH)
          //     .expect("Couldn't get epoch");
          // epoch.as_micros() as u64
    }

    fn send_byte(&mut self, _b: u8) {}

    fn recv_byte(&mut self) -> Option<u8> {
        None
    }

    fn sched(&mut self) -> bool {
        true
    }

    fn load_ram(&mut self, size: usize) -> Vec<u8> {
        vec![0; size]
    }

    fn save_ram(&mut self, _ram: &[u8]) {}
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = onReceiveJournal)]
    fn send_journal_output(data: Vec<u8>);

    #[wasm_bindgen(js_name = onReceiveData)]
    fn send_extracted_data(data: String);
}
