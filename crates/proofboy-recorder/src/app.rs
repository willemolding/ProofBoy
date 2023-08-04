use bevy::{
    // diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::*,
};
use journal::{Journal, KeyState};

use rgy::{debug::NullDebugger, Config, Key as GBKey, Stream, System};
pub use rgy::{VRAM_HEIGHT, VRAM_WIDTH};
use serde_json::de::Read;
use std::cell::RefCell;
use std::rc::Rc;

pub const SCALE: f32 = 2.0;
const CYCLES_PER_FRAME: usize = 70224;

#[derive(Resource, Default, Debug)]
pub struct KeyJournal(pub Journal);

pub struct ProofBoyPlugin{
    /// ROM to load into the gameboy
    pub rom: Vec<u8>,
    /// This journal is used to fast-forward the game to the given state
    pub startup_journal: Option<Journal>,
}

impl Plugin for ProofBoyPlugin {
    fn build(&self, app: &mut App) {

        let mut gb = Gameboy::new(&self.rom);

        if let Some(startup_journal) = self.startup_journal.clone() {
            log::info!("Input journal detected {:?}. Applying key presses...", startup_journal);

            startup_journal.into_iter().for_each(|keys| {
                gb.kbd.0.replace(KeyState::from_byte(keys));
                gb.step();
            });
            log::info!("CPU now at cycle count: {}", gb.cycle_count);
        }

        app.init_resource::<KeyJournal>()
            .insert_non_send_resource(gb)
            .add_systems(Startup, setup_screen)
            .add_systems(Update, (update_gameboy, update_screen));
    }
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
            journal.0.tick(gb.kbd.0.borrow().as_byte());
            gb.step();
        }
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

pub struct Gameboy {
    pub sys: System<NullDebugger>,
    pub display: Display,
    pub kbd: Keyboard,
    pub cycle_count: u64,
    pub active: bool,
}

impl Gameboy {
    pub fn step(&mut self) {
        self.sys.poll(true);
        self.cycle_count += 1;
    }

    pub fn step_nogpu(&mut self) {
        self.sys.poll(false);
        self.cycle_count += 1;
    }

    pub fn new(rom: &[u8]) -> Self {
        let kbd = Keyboard::new();
        let display = Display::new();
        let cfg = Config::new().native_speed(true);
        let hw = Hardware::new(display.clone(), kbd.clone());
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
pub struct Display(pub Rc<RefCell<Vec<Vec<u32>>>>);

#[derive(Clone, Debug)]
pub struct Keyboard(pub Rc<RefCell<KeyState>>);

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
    pub display: Display,
    pub kbd: Keyboard,
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
