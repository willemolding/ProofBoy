mod key_state;

pub use key_state::KeyState;

#[derive(Default, Debug, Clone)]
pub struct Journal {
    pub cycle: u64,
    pub pending: u8, // button press to be added. We just don't know how long it was pressed for yet
    pub last_add: u64, // value of cycle when an entry
    pub entries: Vec<JournalEntry>,
}

#[derive(Clone, Default, Debug)]
pub struct JournalEntry {
    pub joypad: u8, // the joypad state
    pub delta: u32, // number of CPU cycles this joypad state should be active for
}

impl Journal {
    /// To be called every clock cycle with the joypad as it was when that cycle was executed. This struct will determine if it should be stored
    pub fn tick(&mut self, joypad: u8) {
        if joypad != self.pending {
            let new_entry = JournalEntry {
                joypad: self.pending,
                delta: (self.cycle - self.last_add) as u32,
            };
            log::info!("Journal added: {:?}", new_entry);
            self.entries.push(new_entry);
            self.pending = joypad;
            self.last_add = self.cycle;
        }
        self.cycle+=1;
    }

    // add a terminating value to the journal
    // this just writes whatever is in pending to the journal
    pub fn close(&mut self) {
        let new_entry = JournalEntry {
            joypad: self.pending,
            delta: (self.cycle - self.last_add) as u32,
        };
        log::info!("Journal added: {:?}", new_entry);
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
        bytes.chunks_exact(5).for_each(|w| {
            let mut delta = [0u8; 4];
            delta.copy_from_slice(&w[0..4]);
            entries.push(JournalEntry {
                delta: u32::from_be_bytes(delta),
                joypad: w[4],
            });
        });
        Self {
            entries,
            ..Default::default()
        }
    }

    pub fn into_iter(self) -> impl Iterator<Item = u8> {
        self.entries.into_iter()
            .flat_map(|entry| {
                std::iter::repeat(entry.joypad).take(entry.delta as usize)
            })
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::vec;

    #[test]
    fn test_iterator_on_empty() {
        let journal = Journal::default();
        let mut iter = journal.into_iter();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iterator_single_entry() {
        let journal = Journal {
            entries: vec![JournalEntry {
                joypad: 0x1,
                delta: 1,
            }],
            ..Journal::default()
        };
        let mut iter = journal.into_iter();
        assert_eq!(iter.next(), Some(0x1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iterator_single_held() {
        let journal = Journal {
            entries: vec![
            JournalEntry {
                joypad: 0x1,
                delta: 3,
            },
            JournalEntry {
                joypad: 0x2,
                delta: 2,
            }
            ],
            ..Journal::default()
        };
        assert_eq!(journal.clone().into_iter().collect::<Vec<u8>>().len(), 5);

        let mut iter = journal.into_iter();
        for _ in 0..3 {
            assert_eq!(iter.next(), Some(0x1));
        }
        for _ in 0..2 {
            assert_eq!(iter.next(), Some(0x2));
        }
        assert_eq!(iter.next(), None);
    }

}
