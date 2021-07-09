#![cfg_attr(all(feature = "nightly", test), feature(test))]
#[cfg(all(feature = "nightly", test))]
extern crate test;

#[cfg(test)]
mod tests;

mod read;
mod write;

pub use read::BitVecReader;
pub use write::BitVecWriter;

pub trait BitCount {
	const BIT_COUNT: usize;
}
macro_rules! impl_bit_count {
	( $ty:ty, $n:literal ) => {
		impl BitCount for $ty {
			const BIT_COUNT: usize = $n;
		}
	};
}
impl_bit_count!(u8, 8);
impl_bit_count!(u16, 16);
impl_bit_count!(u32, 32);
impl_bit_count!(u64, 64);
impl_bit_count!(u128, 128);
impl_bit_count!(i8, 8);
impl_bit_count!(i16, 16);
impl_bit_count!(i32, 32);
impl_bit_count!(i64, 64);
impl_bit_count!(i128, 128);
impl_bit_count!(f32, 32);
impl_bit_count!(f64, 64);

#[cfg(target_pointer_width = "128")]
impl_bit_count!(isize, 128);
#[cfg(target_pointer_width = "128")]
impl_bit_count!(usize, 128);

#[cfg(target_pointer_width = "64")]
impl_bit_count!(isize, 64);
#[cfg(target_pointer_width = "64")]
impl_bit_count!(usize, 64);

#[cfg(target_pointer_width = "32")]
impl_bit_count!(isize, 32);
#[cfg(target_pointer_width = "32")]
impl_bit_count!(usize, 32);
