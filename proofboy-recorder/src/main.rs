use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use log;
use rgy::{debug::NullDebugger, Config, Key as GBKey, Stream, System, VRAM_HEIGHT, VRAM_WIDTH};
use std::cell::RefCell;
use std::rc::Rc;

const SCALE: f32 = 2.0;
const CYCLES_PER_FRAME: usize = 70224;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .add_systems(Startup, (setup_screen, setup_gameboy))
        .add_systems(Update, (update_gameboy, update_screen))
        .run();
}

// need to do it this way rather than using a resource because it is non-send
fn setup_gameboy(world: &mut World) {
    let gb = Gameboy::default();
    world.insert_non_send_resource(gb);
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
                        SCALE * (x as f32),
                        -SCALE * (y as f32),
                        0.,
                    )),
                    ..default()
                },
                ScreenPosition { x, y },
            ));
        }
    }
}

fn update_gameboy(mut gb: NonSendMut<Gameboy>, keys: Res<Input<KeyCode>>) {
    if keys.pressed(KeyCode::Up) {
        gb.kbd.0.borrow_mut().up = true;
    } else {
        gb.kbd.0.borrow_mut().up = false;
    }

    if keys.pressed(KeyCode::Return) {
        gb.kbd.0.borrow_mut().start = true;
    } else {
        gb.kbd.0.borrow_mut().start = false;
    }

    if keys.pressed(KeyCode::Down) {
        gb.kbd.0.borrow_mut().down = true;
    } else {
        gb.kbd.0.borrow_mut().down = false;
    }

    if keys.pressed(KeyCode::Left) {
        gb.kbd.0.borrow_mut().left = true;
    } else {
        gb.kbd.0.borrow_mut().left = false;
    }

    if keys.pressed(KeyCode::Right) {
        gb.kbd.0.borrow_mut().right = true;
    } else {
        gb.kbd.0.borrow_mut().right = false;
    }

    if keys.pressed(KeyCode::Z) {
        gb.kbd.0.borrow_mut().a = true;
    } else {
        gb.kbd.0.borrow_mut().a = false;
    }

    if keys.pressed(KeyCode::X) {
        gb.kbd.0.borrow_mut().b = true;
    } else {
        gb.kbd.0.borrow_mut().b = false;
    }

    if keys.pressed(KeyCode::Space) {
        gb.kbd.0.borrow_mut().select = true;
    } else {
        gb.kbd.0.borrow_mut().select = false;
    }

    let gb = gb.as_mut();
    for _ in 0..CYCLES_PER_FRAME/3 {
        gb.sys.poll();
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
}

impl Default for Gameboy {
    fn default() -> Self {
        let kbd = Keyboard::new();
        let display = Display::new();
        let cfg = Config::new();
        let hw = Hardware::new(display.clone(), kbd.clone());

        let rom = include_bytes!("../../roms/pokemon-blue.gb");

        let sys = System::new(cfg, rom, hw, NullDebugger);

        Self { sys, display, kbd }
    }
}

#[derive(Clone, Debug)]
struct Display(Rc<RefCell<Vec<Vec<u32>>>>);

#[derive(Clone, Debug)]
struct Keyboard(Rc<RefCell<Inner>>);

#[derive(Default, Debug)]
struct Inner {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    a: bool,
    b: bool,
    start: bool,
    select: bool,
}

impl Keyboard {
    fn new() -> Self {
        Self(Rc::new(RefCell::new(Inner::default())))
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
