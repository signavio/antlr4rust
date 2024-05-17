// pub use murmur3::murmur3_32::MurmurHasher as DefaultHasher;
use std::hash::{BuildHasher, Hasher};

#[doc(hidden)]
#[derive(Debug)]
pub struct DefaultHasher {
    buf: Vec<u8>,
    index: usize,
}

impl Default for DefaultHasher {
    fn default() -> Self {
        Self {
            buf: Vec::new(),
            index: 0,
        }
    }
}

impl Hasher for DefaultHasher {
    fn finish(&self) -> u64 { fastmurmur3::hash(&self.buf) as _ }

    fn write(&mut self, bytes: &[u8]) {
        self.buf.extend(bytes);
        self.index += bytes.len();
    }
}

#[doc(hidden)]
#[derive(Debug)]
pub struct DefaultHasherBuilder {}

impl BuildHasher for DefaultHasherBuilder {
    type Hasher = DefaultHasher;

    fn build_hasher(&self) -> Self::Hasher { DefaultHasher::default() }
}
