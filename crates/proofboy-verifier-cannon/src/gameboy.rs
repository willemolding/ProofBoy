use journal::KeyState;
use rgy::{debug::NullDebugger, Config, Key as GBKey, Stream, System};

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec;
use alloc::vec::Vec;
use core::cell::RefCell;

/// A headless gameboy emulator for speed! Does not emulator sound or GPU
pub struct Gameboy {
    pub sys: System<NullDebugger>,
    pub kbd: Keyboard,
    pub cycle_count: u64,
    pub active: bool,
}

impl Gameboy {
    pub fn step(&mut self) {
        self.sys.poll(false);
        self.cycle_count += 1;
    }

    pub fn new(rom: &[u8]) -> Self {
        let kbd = Keyboard::new();
        let cfg = Config::new().native_speed(true);
        let hw = Hardware::new(kbd.clone());
        let sys = System::new(cfg, rom, vec![0u8; 0x10000], hw, NullDebugger);
        Self {
            sys,
            kbd,
            cycle_count: 0,
            active: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Keyboard(pub Rc<RefCell<KeyState>>);

impl Keyboard {
    fn new() -> Self {
        Self(Rc::new(RefCell::new(KeyState::default())))
    }
}

struct Hardware {
    pub kbd: Keyboard,
}

impl Hardware {
    fn new(kbd: Keyboard) -> Self {
        Self { kbd }
    }
}

impl rgy::Hardware for Hardware {
    fn vram_update(&mut self, line: usize, buffer: &[u32]) {}

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
        0
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
