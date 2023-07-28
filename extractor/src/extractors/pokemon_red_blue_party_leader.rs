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

pub struct PartyLeaderExtractor;
use alloc::string::ToString;
use alloc::vec;
use crate::metadata::{Metadata, Attribute};

use super::id_to_pokedex::ID_TO_POKEDEX;

impl crate::Extractor for PartyLeaderExtractor {
    type Output = Metadata;

    fn extract<R: crate::MemoryReader>(reader: &R) -> Self::Output {
        let id = reader.get8(0xD16B);
        let level = reader.get8(0xD18C);
        let max_hp: u16 = reader.get16(0xD18D);
        let attack: u16 = reader.get16(0xD18F);
        let defense: u16 = reader.get16(0xD191);
        let speed: u16 = reader.get16(0xD193);
        let special: u16 = reader.get16(0xD195);

        let (pokedex_num, name) = ID_TO_POKEDEX[id as usize];

        Metadata {
            name: name.to_string(),
            description: "A Pokemon NFT produced by ProofBoy".to_string(),
            image: alloc::format!("https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/{}.png", pokedex_num).to_string(),
            attributes: vec![
                Attribute::new_numeric("Level", level as u32),
                Attribute::new_numeric("Max HP", max_hp as u32),
                Attribute::new_numeric("Attack", attack as u32),
                Attribute::new_numeric("Defense", defense as u32),
                Attribute::new_numeric("Speed", speed as u32),
                Attribute::new_numeric("Special", special as u32),
            ]
        }
    }
}
