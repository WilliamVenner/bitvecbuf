use bitvec::{macros::internal::funty::{IsFloat, IsNumber, IsSigned, IsUnsigned}, order::{BitOrder, Lsb0, Msb0}, slice::BitSlice, store::BitStore, vec::BitVec, view::{AsBits, BitView}};

use crate::BitCount;

#[derive(Debug, Clone, Default)]
pub struct BitVecWriter<O: BitOrder> {
	cursor: usize,
	pub bitvec: BitVec<O, u8>,
}
impl<O: BitOrder> BitVecWriter<O> {
	#[inline]
	pub fn new(bitvec: BitVec<O, u8>) -> BitVecWriter<O> {
		BitVecWriter { cursor: 0, bitvec }
	}

	#[inline]
	pub fn from_bytes(bytes: Vec<u8>) -> BitVecWriter<O> {
		bytes.into()
	}

	#[inline]
	pub fn from_byte_slice<B: AsRef<[u8]>>(bytes: B) -> BitVecWriter<O> {
		bytes.as_ref().into()
	}

	#[inline]
	pub fn into_bytes(self) -> Vec<u8> {
		self.bitvec.into_vec()
	}

	#[inline]
	pub fn into_bitvec(self) -> BitVec<O, u8> {
		self.bitvec
	}

	#[inline]
	pub fn bits_written(&self) -> usize {
		self.cursor
	}

	/// Returns the number of bytes written to the bitvec, rounded up to the byte.
	#[inline]
	pub fn bytes_written(&self) -> usize {
		((self.bits_written() as f32) / 8.).ceil() as usize
	}

	#[inline]
	pub fn advance(&mut self, n: usize) {
		self.cursor += n;
	}

	#[inline]
	pub fn cursor(&self) -> usize {
		self.cursor
	}

	/// Sets the bit at the given index. Will panic if the index is out of bounds.
	#[inline]
	pub fn set_bit(&mut self, index: usize, bit: bool) {
		self.bitvec.set(index, bit);
	}

	/// Sets the bits at the given index. Will panic if the index is out of bounds.
	#[inline]
	pub fn set_bits<T: BitStore>(&mut self, index: usize, bits: &BitSlice<O, T>) {
		for (i, bit) in bits.iter().enumerate() {
			self.bitvec.set(index + i, *bit);
		}
	}

	#[inline]
	pub fn write_bit(&mut self, bit: bool) {
		self.bitvec.push(bit);
		self.advance(1);
	}

	#[inline]
	pub fn write_byte(&mut self, byte: u8) {
		let bits = byte.view_bits();
		self.bitvec.extend_from_bitslice::<O, u8>(bits);
		self.advance(bits.len());
	}

	pub fn write_bytes<B: AsRef<[u8]>>(&mut self, bytes: B) {
		let bytes = bytes.as_ref();
		let bits = bytes.view_bits();
		self.bitvec.extend_from_bitslice::<O, u8>(bits);
		self.advance(bits.len());
	}

	pub fn write_float(&mut self, float: f32) {
		let float = float.into_bitview();
		let bits = float.view_bits();
		self.bitvec
			.extend_from_bitslice::<O, <<f32 as IsFloat>::Raw as BitView>::Store>(bits);
		self.advance(bits.len());
	}

	#[cfg(target_pointer_width = "64")]
	pub fn write_double(&mut self, double: f64) {
		let double = double.into_bitview();
		let bits = double.view_bits();
		self.bitvec
			.extend_from_bitslice::<O, <<f64 as IsFloat>::Raw as BitView>::Store>(bits);
		self.advance(bits.len());
	}

	pub fn write_string<S: AsRef<str>>(&mut self, str: S) {
		let str = str.as_ref();
		let bits = str.as_bits();
		self.bitvec.extend_from_bitslice::<O, u8>(bits);
		self.advance(bits.len());
	}

	pub fn write_string_nul<S: AsRef<str>>(&mut self, str: S) {
		self.write_string(str);
		self.write_byte(0);
	}
}

impl BitVecWriter<Msb0> {
	pub fn write_int<N>(&mut self, int: N, bits: usize)
	where
		N: IsNumber + IsSigned + IntoBitView + BitCount,
	{
		let int = int.into_bitview();
		let bits = &int.view_bits()[(N::BIT_COUNT - bits)..];
		self.bitvec
			.extend_from_bitslice::<Msb0, <<N as IntoBitView>::Unsigned as BitView>::Store>(bits);
		self.advance(bits.len());
	}

	pub fn write_uint<N>(&mut self, uint: N, bits: usize)
	where
		N: BitView + IsNumber + IsUnsigned + BitCount,
	{
		let bits = &uint.view_bits()[(N::BIT_COUNT - bits)..];
		self.bitvec.extend_from_bitslice::<Msb0, N::Store>(bits);
		self.advance(bits.len());
	}

	#[cfg(target_pointer_width = "32")]
	pub fn write_double(&mut self, double: f64) {
		// Split the f64 into two u32s
		const MASK_64_32: u64 = 0x00000000FFFFFFFF;
		let double: u64 = unsafe { std::mem::transmute(double) };
		let double1: u32 = ((double >> 32) & MASK_64_32) as u32;
		let double2: u32 = (double & MASK_64_32) as u32;
		self.write_uint(double1, 32);
		self.write_uint(double2, 32);
	}
}
impl BitVecWriter<Lsb0> {
	pub fn write_int<N>(&mut self, int: N, bits: usize)
	where
		N: IsNumber + IsSigned + IntoBitView,
	{
		let int = int.into_bitview();
		let bits = &int.view_bits()[..bits];
		self.bitvec
			.extend_from_bitslice::<Lsb0, <<N as IntoBitView>::Unsigned as BitView>::Store>(bits);
		self.advance(bits.len());
	}

	pub fn write_uint<N>(&mut self, uint: N, bits: usize)
	where
		N: BitView + IsNumber + IsUnsigned,
	{
		let bits = &uint.view_bits()[..bits];
		self.bitvec.extend_from_bitslice::<Lsb0, N::Store>(bits);
		self.advance(bits.len());
	}

	#[cfg(target_pointer_width = "32")]
	pub fn write_double(&mut self, double: f64) {
		// Split the f64 into two u32s
		const MASK_64_32: u64 = 0x00000000FFFFFFFF;
		let double: u64 = unsafe { std::mem::transmute(double) };
		let double1: u32 = ((double >> 32) & MASK_64_32) as u32;
		let double2: u32 = (double & MASK_64_32) as u32;
		self.write_uint(double2, 32); // SWAPPED ORDER
		self.write_uint(double1, 32);
	}
}

impl<O: BitOrder> Into<Vec<u8>> for BitVecWriter<O> {
	fn into(self) -> Vec<u8> {
		self.into_bytes()
	}
}
impl<O: BitOrder> From<Vec<u8>> for BitVecWriter<O> {
	fn from(bytes: Vec<u8>) -> Self {
		BitVecWriter::from_bytes(bytes)
	}
}
impl<O: BitOrder> From<&[u8]> for BitVecWriter<O> {
	fn from(bytes: &[u8]) -> Self {
		BitVecWriter::from_byte_slice(bytes)
	}
}

pub trait IntoBitView {
	type Unsigned: BitView + IsNumber + IsUnsigned;
	fn into_bitview(self) -> Self::Unsigned;
}
macro_rules! impl_into_bitview {
	( $from:ty, $to:ty ) => {
		impl IntoBitView for $from {
			type Unsigned = $to;
			fn into_bitview(self) -> Self::Unsigned {
				unsafe { std::mem::transmute(self) }
			}
		}
	};
}
macro_rules! impl_into_bitview_float {
	( $from:ty ) => {
		impl IntoBitView for $from {
			type Unsigned = <$from as IsFloat>::Raw;
			fn into_bitview(self) -> Self::Unsigned {
				unsafe { std::mem::transmute(self) }
			}
		}
	};
}
impl_into_bitview!(i8, u8);
impl_into_bitview!(i16, u16);
impl_into_bitview!(i32, u32);
impl_into_bitview!(isize, usize);
impl_into_bitview_float!(f32);

#[cfg(target_pointer_width = "64")]
impl_into_bitview!(i64, u64);
#[cfg(target_pointer_width = "64")]
impl_into_bitview_float!(f64);
