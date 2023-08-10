#![no_std]

extern crate alloc;

pub mod extractors;
mod memory_reader;
pub mod metadata;

pub use memory_reader::MemoryReader;
pub use metadata::{Attribute, Metadata};

pub trait Extractor {
    type Output;
    type Error;
    fn extract<R: MemoryReader>(reader: &R) -> Result<Self::Output, Self::Error>;
}
