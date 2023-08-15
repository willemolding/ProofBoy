use alloc::string::{String, ToString};
use alloc::vec::Vec;

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
    pub display_type: Option<String>,
}

impl Attribute {
    pub fn new_numeric(name: &str, value: u32) -> Self {
        Self {
            trait_type: name.to_string(),
            value,
            display_type: Some("number".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_serialize_metadata() {
        let metadata = Metadata {
            name: "foo".to_string(),
            description: "bar".to_string(),
            image: "baz".to_string(),
            attributes: vec![Attribute::new_numeric("foo", 42)],
        };
        let json = serde_json::to_string(&metadata).unwrap();
        assert_eq!(
            json,
            r#"{"name":"foo","description":"bar","image":"baz","attributes":[{"trait_type":"foo","value":42,"display_type":"number"}]}"#
        );
    }
}
