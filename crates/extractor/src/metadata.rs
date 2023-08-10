use alloc::vec::Vec;
use alloc::string::{String, ToString};

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Metadata {
    pub name: String,
    pub description: String,
    pub image: String,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Attribute {
    pub trait_type: String,
    pub value: u32,
    pub display_type: Option<String>
}

impl Attribute {
    pub fn new_numeric(name: &str, value: u32) -> Self {
        Self {
            trait_type: name.to_string(),
            value,
            display_type: Some("number".to_string(),)
        }
    }
}
