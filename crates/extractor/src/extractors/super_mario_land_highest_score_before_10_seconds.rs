//! Extracts the pokemon listed first in the party
//! Retrieves all its stats and moves
//!
//! Info obtained from here https://datacrystal.romhacking.net/wiki/Super_Mario_Land:RAM_map
//! 
//! Addr    Size    Description
//! ----    ----    -----------
//! 9806       2    Lives displayed (copy from 0xDA15)
//! 9820       6    Score (copy from 0xC0A0)
//! ....    ....    ....
//! 9829       1    Coins - Tens
//! 982A       1    Coins - Ones
//! ....    ....    ....
//! 982C       1    Current world
//! 982E       1    Current stage
//! ....    ....    ....
//! 9831       1    Timer - Hundreds
//! 9832       1    Timer - Tens
//! 9833       1    Timer - Ones
//! 



use alloc::string::ToString;
use alloc::vec;
use crate::metadata::{Metadata, Attribute};

pub struct SuperMarioLandHighestScoreBefore10SecondsExtractor
;
impl crate::Extractor for SuperMarioLandHighestScoreBefore10SecondsExtractor {
    type Output = Metadata;
    type Error = &'static str;

    fn extract<R: crate::MemoryReader>(reader: &R) -> Result<Self::Output, Self::Error> {
        let id = reader.get8(0xD16B);
       
        let score: u16 = reader.get16(0x9820);


        Ok(Metadata {
            name: "Mario Score".to_string(),
            description: "Super Mario World Score on 1-1 in 10 seconds (produced by ProofBoy)".to_string(),
            image: alloc::format!("https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/{}.png", pokedex_num).to_string(),
            attributes: vec![
                Attribute::new_numeric("Level", level as u32),
            ]
        })
    }
}
