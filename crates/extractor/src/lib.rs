#![no_std]

extern crate alloc;

pub mod extractors;
mod memory_reader;
pub mod metadata;

pub use memory_reader::MemoryReader;
pub use metadata::{Metadata, Attribute};

pub trait Extractor {
    type Output;
    fn extract<R: MemoryReader>(reader: &R) -> Self::Output;
}
