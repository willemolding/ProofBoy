#[derive(Clone, Default, Debug)]
pub struct KeyState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub a: bool,
    pub b: bool,
    pub start: bool,
    pub select: bool,
}

impl KeyState {
    pub fn as_byte(&self) -> u8 {
        let mut byte = 0;
        if self.right {
            byte |= 1 << 0;
        }
        if self.left {
            byte |= 1 << 1;
        }
        if self.up {
            byte |= 1 << 2;
        }
        if self.down {
            byte |= 1 << 3;
        }
        if self.a {
            byte |= 1 << 4;
        }
        if self.b {
            byte |= 1 << 5;
        }
        if self.select {
            byte |= 1 << 6;
        }
        if self.start {
            byte |= 1 << 7;
        }
        byte
    }

    pub fn from_byte(b: u8) -> Self {
        Self {
            right: b & (1 << 0) != 0,
            left: b & (1 << 1) != 0,
            up: b & (1 << 2) != 0,
            down: b & (1 << 3) != 0,
            a: b & (1 << 4) != 0,
            b: b & (1 << 5) != 0,
            select: b & (1 << 6) != 0,
            start: b & (1 << 7) != 0,
        }
    }
}
