pub struct Journal(Vec<JournalEntry>);

pub struct JournalEntry {
    joypad_diff: u8, // the joypad state is equal to the previous state XORed with this. The state remains so until the next entry.
    cycles_delta: usize, // number of CPU cycles since the previous entry
}

impl Journal {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, joypad_diff: u8, cycles_delta: usize) {
        self.0.push(JournalEntry {
            joypad_diff,
            cycles_delta,
        });
    }

    pub fn iter(&self) -> impl Iterator<Item = &JournalEntry> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
