#![cfg_attr(all(feature = "nightly", test), feature(test))]
#[cfg(all(feature = "nightly", test))]
extern crate test;

#[cfg(test)]
mod tests;

mod write;
mod read;

pub use write::BitVecWriter;
pub use read::BitVecReader;