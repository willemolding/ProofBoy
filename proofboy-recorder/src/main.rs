use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rgy::{debug::NullDebugger, Config, Key as GBKey, Stream, System, VRAM_HEIGHT, VRAM_WIDTH};

const SCALE: f32 = 2.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_gameboy)
        .add_systems(Update, update)
        .run();
}

// need to do it this way rather than using a resource because it is non-send
fn setup_gameboy(world: &mut World) {
    let gb = Gameboy::default();
    world.insert_non_send_resource(gb);
}

fn update(gb: NonSendMut<Gameboy>, keyboard: Res<Input<KeyCode>>, time: Res<Time>) {
    
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

        let rom = b"xxx";//include_bytes!(env!("WABOY_ROM"));

        let sys = System::new(cfg, rom, hw, NullDebugger);

        Self { sys, display, kbd }
    }
}

impl Gameboy {
    fn poll(&mut self) {
        for _ in 0..3 {
            self.sys.poll();
        }
    }
}

#[derive(Debug, Clone)]
struct Display(Vec<Vec<u32>>);

#[derive(Debug, Clone)]
struct Keyboard(Inner);

#[derive(Default, Debug, Clone)]
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

impl Display {
    fn new() -> Self {
        Self(vec![
            vec![0u32; VRAM_HEIGHT];
            VRAM_WIDTH
        ])
    }
}

impl Keyboard {
    fn new() -> Self {
        Self(Inner::default())
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
            self.display.0[x][y] = *col;
        }
    }

    fn joypad_pressed(&mut self, key: GBKey) -> bool {
        match key {
            GBKey::Up => self.kbd.0.up,
            GBKey::Down => self.kbd.0.down,
            GBKey::Left => self.kbd.0.left,
            GBKey::Right => self.kbd.0.right,
            GBKey::A => self.kbd.0.a,
            GBKey::B => self.kbd.0.b,
            GBKey::Start => self.kbd.0.start,
            GBKey::Select => self.kbd.0.select,
        }
    }

    fn sound_play(&mut self, _stream: Box<dyn Stream>) {}

    fn clock(&mut self) -> u64 {
        #[cfg(target_arch = "wasm32")]
        {
            Date::now() as u64 * 1000
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let epoch = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Couldn't get epoch");
            epoch.as_micros() as u64
        }
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
