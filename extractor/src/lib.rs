pub mod extractors;
mod memory_reader;

pub use memory_reader::MemoryReader;

pub trait Extractor {
    type Output;
    fn extract<R: MemoryReader>(reader: &R) -> Self::Output;
}
