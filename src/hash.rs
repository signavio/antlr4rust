pub use murmur3::murmur3_32::MurmurHasher as DefaultHasher;
use std::hash::BuildHasher;

#[doc(hidden)]
#[derive(Debug)]
pub struct DefaultHasherBuilder {}

impl BuildHasher for DefaultHasherBuilder {
    type Hasher = DefaultHasher;

    fn build_hasher(&self) -> Self::Hasher {
        DefaultHasher::default()
    }
}
