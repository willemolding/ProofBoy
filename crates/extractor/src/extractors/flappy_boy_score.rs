//! Extracts the current score in FlappyBoy
//!
//! C0F1 - Score
//!

use crate::metadata::{Attribute, Metadata};
use alloc::string::ToString;
use alloc::format;
use alloc::vec;

pub struct FlappyBoyScoreExtractor;

impl crate::Extractor for FlappyBoyScoreExtractor {
    type Output = Metadata;
    type Error = &'static str;

    fn extract<R: crate::MemoryReader>(reader: &R) -> Result<Self::Output, Self::Error> {
        let score = reader.get8(0xC0F1);

        Ok(Metadata {
            name: format!("FlappyBoy score: {}", score).to_string(),
            description: format!("FlappyBoy score: {} - produced by ProofBoy", score).to_string(),
            image: "https://img.itch.zone/aW1hZ2UvNDIwMTE4LzIwOTI2MjkucG5n/794x1000/SBZwlo.png".to_string(),
            attributes: vec![
                Attribute {
                    trait_type: "Score".to_string(),
                    value: score as u32,
                    display_type: None,
                }
            ],
        })
    }
}
