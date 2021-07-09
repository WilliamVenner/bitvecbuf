use std::{io::Read, ops::Range, string::FromUtf8Error};

use bitvec::{
	field::BitField,
	macros::internal::funty::{IsFloat, IsNumber, IsSigned, IsUnsigned},
	mem::BitMemory,
	order::{BitOrder, Lsb0, Msb0},
	slice::BitSlice,
	vec::BitVec,
};

use crate::BitCount;

#[derive(Debug, Clone)]
pub struct BitVecReader<O: BitOrder> {
	cursor: usize,
	bitvec: BitVec<O, u8>,
}
impl<O: BitOrder> BitVecReader<O>
where
	BitSlice<O, u8>: BitField + LoadBits<O>,
{
	#[inline]
	pub fn new(bitvec: BitVec<O, u8>) -> BitVecReader<O> {
		BitVecReader { cursor: 0, bitvec }
	}

	#[inline]
	pub fn from_bytes(bytes: Vec<u8>) -> BitVecReader<O> {
		BitVecReader {
			cursor: 0,
			bitvec: BitVec::from_vec(bytes),
		}
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
	pub fn bits_len(&self) -> usize {
		self.bitvec.len()
	}

	/// Returns the number of bytes in the bitvec, rounded up to the byte.
	#[inline]
	pub fn bytes_len(&self) -> usize {
		((self.bits_len() as f32) / 8.).ceil() as usize
	}

	#[inline]
	pub fn bits_left(&self) -> usize {
		self.bitvec.len() - self.cursor
	}

	#[inline]
	pub fn bytes_left(&self) -> usize {
		((self.bits_left() as f32) / 8.).ceil() as usize
	}

	#[inline]
	pub fn bits_read(&self) -> usize {
		self.cursor
	}

	#[inline]
	pub fn bytes_read(&self) -> usize {
		((self.bits_read() as f32) / 8.).ceil() as usize
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
	fn check_range(&self, max: usize) -> Option<Range<usize>> {
		let max = self.cursor + max;
		if self.bitvec.len() < max {
			None
		} else {
			Some(self.cursor..max)
		}
	}

	#[inline]
	fn read_bits(&self, max: usize) -> Option<&BitSlice<O, u8>>
	where
		BitSlice<O, u8>: BitField + LoadBits<O>,
	{
		Some(&self.bitvec[self.check_range(max)?])
	}

	#[inline]
	pub fn read_bit(&mut self) -> Option<bool> {
		let bit = self.bitvec.get(self.cursor).map(|bit| *bit);
		self.advance(1);
		bit
	}

	#[inline]
	pub fn read_byte(&mut self) -> Option<u8> {
		let byte = self.read_bits(8)?;
		let byte = Some(byte.load_bits());
		self.advance(8);
		byte
	}

	pub fn read_bytes(&mut self, len: usize) -> Option<Vec<u8>> {
		let len_bits = len * 8;
		let range = self.check_range(len_bits)?;

		let mut bytes = Vec::with_capacity(len);
		for i in range.step_by(8) {
			let byte: u8 = self.bitvec[i..i+8].load_bits();
			bytes.push(byte);
		}

		self.advance(len_bits);

		Some(bytes)
	}

	#[inline]
	pub fn read_uint<N>(&mut self, bits: usize) -> Option<N>
	where
		N: BitMemory + IsNumber + IsUnsigned,
	{
		let uint = self.read_bits(bits)?;
		let uint: Option<N> = Some(uint.load_bits());
		self.advance(bits);
		uint
	}

	#[inline]
	pub fn read_int<N>(&mut self, bits: usize) -> Option<N>
	where
		N: FromBitMemory + IsNumber + IsSigned,
	{
		let int = self.read_bits(bits)?;
		let int: Option<N> = Some(int.load_bits())
			.map(|int: <N as FromBitMemory>::Unsigned| N::from_bitmemory(int, bits));
		self.advance(bits);
		int
	}

	#[inline]
	pub fn read_float<N>(&mut self) -> Option<N>
	where
		N: FromBitMemory + IsNumber + IsFloat,
	{
		let float = self.read_bits(N::BIT_COUNT)?;
		let float: Option<N> = Some(float.load_bits())
			.map(|float: <N as FromBitMemory>::Unsigned| N::from_bitmemory(float, N::BIT_COUNT));
		self.advance(N::BIT_COUNT);
		float
	}

	#[inline]
	pub unsafe fn read_string_unchecked(&mut self, bytes: usize) -> Option<String> {
		self.read_bytes(bytes)
			.map(|bytes| String::from_utf8_unchecked(bytes))
	}

	#[inline]
	pub fn read_string(&mut self, bytes: usize) -> Option<Result<String, FromUtf8Error>> {
		self.read_bytes(bytes).map(|bytes| String::from_utf8(bytes))
	}

	#[inline]
	pub fn read_string_lossy(&mut self, bytes: usize) -> Option<String> {
		self.read_bytes(bytes)
			.map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
	}

	pub fn read_nul_string(&mut self) -> Option<Result<String, FromUtf8Error>> {
		let mut string = Vec::new();
		while self.cursor < self.bitvec.len() {
			let byte: u8 = self.bitvec[self.cursor..self.cursor + 8].load_bits();
			self.advance(8);
			if byte == 0 {
				break;
			} else {
				string.push(byte);
			}
		}
		Some(String::from_utf8(string))
	}

	pub unsafe fn read_nul_string_unchecked(&mut self) -> Option<String> {
		let mut string = String::new();
		while self.cursor < self.bitvec.len() {
			let byte: u8 = self.bitvec[self.cursor..self.cursor + 8].load_bits();
			self.advance(8);
			if byte == 0 {
				break;
			} else {
				string.push(byte as char);
			}
		}
		Some(string)
	}

	pub fn read_nul_string_lossy(&mut self) -> Option<String> {
		let mut string = Vec::new();
		while self.cursor < self.bitvec.len() {
			let byte: u8 = self.bitvec[self.cursor..self.cursor + 8].load_bits();
			self.advance(8);
			if byte == 0 {
				break;
			} else {
				string.push(byte);
			}
		}
		Some(String::from_utf8_lossy(&string).into_owned())
	}
}

impl<O: BitOrder> From<Vec<u8>> for BitVecReader<O>
where
	BitSlice<O, u8>: BitField + LoadBits<O>,
{
	fn from(bytes: Vec<u8>) -> Self {
		BitVecReader::from_bytes(bytes)
	}
}

pub trait LoadBits<O: BitOrder>
where
	BitSlice<O, u8>: BitField,
{
	fn load_bits<M: BitMemory>(&self) -> M;
}
impl<O: BitOrder> LoadBits<O> for BitSlice<Lsb0, u8>
where
	BitSlice<O, u8>: BitField,
{
	fn load_bits<M: BitMemory>(&self) -> M {
		self.load_le()
	}
}
impl<O: BitOrder> LoadBits<O> for BitSlice<Msb0, u8>
where
	BitSlice<O, u8>: BitField,
{
	fn load_bits<M: BitMemory>(&self) -> M {
		self.load_be()
	}
}

pub trait FromBitMemory: BitCount {
	type Unsigned: IsUnsigned + IsNumber + BitMemory;
	fn from_bitmemory(u: Self::Unsigned, bits: usize) -> Self;
}
macro_rules! impl_into_bitmemory {
	( $from:ty, $to:ty ) => {
		impl FromBitMemory for $from {
			type Unsigned = $to;
			fn from_bitmemory(u: Self::Unsigned, bits: usize) -> Self {
				if bits == Self::BIT_COUNT {
					unsafe { std::mem::transmute(u) }
				} else {
					let remainder = Self::BIT_COUNT - bits;
					let mask = (<$to>::pow(2, remainder as _) - 1) as $to << (Self::BIT_COUNT - remainder);
					unsafe { std::mem::transmute(u + mask) }
				}
			}
		}
	};
}
macro_rules! impl_into_bitmemory_float {
	( $from:ty ) => {
		impl FromBitMemory for $from {
			type Unsigned = <$from as IsFloat>::Raw;
			fn from_bitmemory(u: Self::Unsigned, _bits: usize) -> Self {
				unsafe { std::mem::transmute(u) }
			}
		}
	};
}
impl_into_bitmemory!(i8, u8);
impl_into_bitmemory!(i16, u16);
impl_into_bitmemory!(i32, u32);
impl_into_bitmemory!(i64, u64);
impl_into_bitmemory!(isize, usize);
impl_into_bitmemory_float!(f32);
impl_into_bitmemory_float!(f64);
