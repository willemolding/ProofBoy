//! Extracts the pokemon listed first in the party
//! Retrieves all its stats and moves
//!
//! Info obtained from here https://datacrystal.romhacking.net/wiki/Pok%C3%A9mon_Red/Blue:RAM_map#Saved_data_.28SRAM.29
//!
//! D16B - Pokémon (Again)
//! D16C-D16D - Current HP
//! D16E - 'Level' (not the actual level, see the notes article)
//! D16F - Status (Poisoned, Paralyzed, etc.)
//! D170 - Type 1
//! D171 - Type 2
//! D172 - Catch rate/Held item (When traded to Generation II)
//! D173 - Move 1
//! D174 - Move 2
//! D175 - Move 3
//! D176 - Move 4
//! D177-D178 - Trainer ID
//! D179-D17B - Experience
//! D17C-D17D - HP EV
//! D17E-D17F - Attack EV
//! D180-D181 - Defense EV
//! D182-D183 - Speed EV
//! D184-D185 - Special EV
//! D186 - Attack/Defense IV
//! D187 - Speed/Special IV
//! D188 - PP Move 1
//! D189 - PP Move 2
//! D18A - PP Move 3
//! D18B - PP Move 4
//! D18C - Level (actual level)
//! D18D-D18E - Max HP
//! D18F-D190 - Attack
//! D191-D192 - Defense
//! D193-D194 - Speed
//! D195-D196 - Special
//!

struct PartyLeaderExtractor;

struct Pokemon {
    id: u8,
    level: u8,
}

impl crate::Extractor for PartyLeaderExtractor {
    type Output = Pokemon;

    fn extract<R: crate::MemoryReader>(reader: &R) -> Self::Output {
        Pokemon {
            id: reader.get8(0xD16B),
            level: reader.get8(0xD18C),
        }
    }
}
