use bitvec::{macros::internal::funty::{IsFloat, IsNumber, IsSigned, IsUnsigned}, order::{BitOrder, Lsb0, Msb0}, vec::BitVec, view::{AsBits, BitView}};

use crate::BitCount;

#[derive(Debug, Clone, Default)]
pub struct BitVecWriter<O: BitOrder> {
	cursor: usize,
	bitvec: BitVec<O, u8>
}
impl<O: BitOrder> BitVecWriter<O> {
	#[inline]
	pub fn new(bitvec: BitVec<O, u8>) -> BitVecWriter<O> {
		BitVecWriter {
			cursor: 0,
			bitvec
		}
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
	pub fn rewind(&mut self, n: usize) {
		self.cursor -= n;
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

	pub fn write_float<N>(&mut self, float: N)
	where
		N: IsNumber + IsFloat + IntoBitView
	{
		let float = float.into_bitview();
		let bits = float.view_bits();
		self.bitvec.extend_from_bitslice::<O, <<N as IntoBitView>::Unsigned as BitView>::Store>(bits);
		self.advance(bits.len());
	}

	pub fn write_bytes<B: AsRef<[u8]>>(&mut self, bytes: B) {
		let bytes = bytes.as_ref();
		let bits = bytes.view_bits();
		self.bitvec.extend_from_bitslice::<O, u8>(bits);
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
		N: IsNumber + IsSigned + IntoBitView + BitCount
	{
		let int = int.into_bitview();
		let bits = &int.view_bits()[(N::BIT_COUNT - bits)..];
		self.bitvec.extend_from_bitslice::<Msb0, <<N as IntoBitView>::Unsigned as BitView>::Store>(bits);
		self.advance(bits.len());
	}

	pub fn write_uint<N>(&mut self, uint: N, bits: usize)
	where
		N: BitView + IsNumber + IsUnsigned + BitCount
	{
		let bits = &uint.view_bits()[(N::BIT_COUNT - bits)..];
		self.bitvec.extend_from_bitslice::<Msb0, N::Store>(bits);
		self.advance(bits.len());
	}
}
impl BitVecWriter<Lsb0> {
	pub fn write_int<N>(&mut self, int: N, bits: usize)
	where
		N: IsNumber + IsSigned + IntoBitView
	{
		let int = int.into_bitview();
		let bits = &int.view_bits()[..bits];
		self.bitvec.extend_from_bitslice::<Lsb0, <<N as IntoBitView>::Unsigned as BitView>::Store>(bits);
		self.advance(bits.len());
	}

	pub fn write_uint<N>(&mut self, uint: N, bits: usize)
	where
		N: BitView + IsNumber + IsUnsigned
	{
		let bits = &uint.view_bits()[..bits];
		self.bitvec.extend_from_bitslice::<Lsb0, N::Store>(bits);
		self.advance(bits.len());
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
impl_into_bitview!(i64, u64);
impl_into_bitview!(isize, usize);
impl_into_bitview_float!(f32);
impl_into_bitview_float!(f64);