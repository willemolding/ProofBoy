mod key_state;

pub use key_state::KeyState;
use std::collections::VecDeque;

#[derive(Default, Debug)]
pub struct Journal {
    counter: u64,
    entries: Vec<JournalEntry>,
}

#[derive(Clone, Default, Debug)]
pub struct JournalEntry {
    joypad: u8, // the joypad state is equal to the previous state XORed with this. The state remains so until the next entry.
    delta: u32, // number of CPU cycles since the previous entry
}

impl Journal {
    /// To be called every clock cycle. This struct will determine if it should be stored
    pub fn tick(&mut self, cycles: u64, joypad: u8) {
        let last_entry = self.entries.last().cloned().unwrap_or_default();
        if joypad != last_entry.joypad {
            log::info!("Journal added: {}: {}", cycles - self.counter, joypad);
            self.entries.push(JournalEntry {
                joypad,
                delta: (cycles - self.counter) as u32,
            });
            self.counter = cycles;
        }
    }

    pub fn to_bytes(self) -> Vec<u8> {
        self.entries
            .into_iter()
            .flat_map(|e| {
                let mut v = Vec::new();
                v.extend_from_slice(&e.delta.to_be_bytes());
                v.push(e.joypad);
                v
            })
            .collect()
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut entries = Vec::new();
        bytes.windows(5).for_each(|w| {
            let mut delta = [0u8; 4];
            delta.copy_from_slice(&w[0..4]);
            entries.push(JournalEntry {
                delta: u32::from_be_bytes(delta),
                joypad: w[4],
            });
        });
        Self {
            entries,
            counter: 0,
        }
    }
}

impl IntoIterator for Journal {
    type Item = u8;
    type IntoIter = JournalIterator;

    fn into_iter(self) -> Self::IntoIter {
        JournalIterator {
            entries: self.entries.into(),
            last_joypad: 0,
            last_cycles: 0,
            cycles: 0,
        }
    }
}

/// An iterator over every CPU cycle that efficiently (ish) returns what the joypad state should be at each cycle
pub struct JournalIterator {
    entries: VecDeque<JournalEntry>,
    last_joypad: u8,
    last_cycles: u64,
    cycles: u64,
}

impl Iterator for JournalIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cycles
            >= self.last_cycles + self.entries.front().map(|e| e.delta as u64).unwrap_or(0)
        {
            let last = self.entries.pop_front().unwrap();
            self.last_joypad = last.joypad;
            self.last_cycles += last.delta as u64;
        }
        self.cycles += 1;
        Some(self.last_joypad)
    }
}
