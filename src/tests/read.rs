test!(read_bit_lsb, read_bit_msb, {
	use crate::{BitVecReader, BitVecWriter};

	let mut bitbuf = BitVecWriter::<Endian>::default();
	bitbuf.write_bit(true);
	bitbuf.write_bit(false);
	bitbuf.write_bit(true);

	let mut bitbuf = BitVecReader::<Endian>::new(bitbuf.into_bitvec());
	assert_eq!(bitbuf.read_bit(), Some(true));
	assert_eq!(bitbuf.read_bit(), Some(false));
	assert_eq!(bitbuf.read_bit(), Some(true));
	assert_eq!(bitbuf.read_bit(), None);
});

test!(read_byte_lsb, read_byte_msb, {
	use crate::{BitVecReader, BitVecWriter};

	let mut bitbuf = BitVecWriter::<Endian>::default();
	bitbuf.write_byte(69);
	bitbuf.write_byte(50);

	let mut bitbuf = BitVecReader::<Endian>::new(bitbuf.into_bitvec());
	assert_eq!(bitbuf.read_byte(), Some(69));
	assert_eq!(bitbuf.read_byte(), Some(50));
	assert_eq!(bitbuf.read_byte(), None);
});

test!(read_uint_lsb, read_uint_msb, {
	use crate::{BitVecReader, BitVecWriter};

	let mut bitbuf = BitVecWriter::<Endian>::default();
	bitbuf.write_uint(69_u16, 12);
	bitbuf.write_uint(12_u16, 15);
	bitbuf.write_uint(u16::MAX, 16);

	let mut bitbuf = BitVecReader::<Endian>::new(bitbuf.into_bitvec());
	assert_eq!(bitbuf.read_uint(12), Some(69_u16));
	assert_eq!(bitbuf.read_uint(15), Some(12_u16));
	assert_eq!(bitbuf.read_uint(16), Some(u16::MAX));
	assert_eq!(bitbuf.read_uint(16), None::<u16>);
});

test!(read_int_lsb, read_int_msb, {
	use crate::{BitVecReader, BitVecWriter};

	let mut bitbuf = BitVecWriter::<Endian>::default();
	bitbuf.write_int(-69_i16, 14);
	bitbuf.write_int(-12_i16, 15);
	bitbuf.write_int(i16::MIN, 16);

	let mut bitbuf = BitVecReader::<Endian>::new(bitbuf.into_bitvec());
	assert_eq!(bitbuf.read_int(14), Some(-69_i16));
	assert_eq!(bitbuf.read_int(15), Some(-12_i16));
	assert_eq!(bitbuf.read_int(16), Some(i16::MIN));
	assert_eq!(bitbuf.read_int(16), None::<i16>);
});

test!(read_float_lsb, read_float_msb, {
	use crate::{BitVecReader, BitVecWriter};

	let mut bitbuf = BitVecWriter::<Endian>::default();
	bitbuf.write_float(69.69_f32);
	bitbuf.write_float(0.01_f32);
	bitbuf.write_float(f32::MAX);
	bitbuf.write_float(f32::MIN);

	let mut bitbuf = BitVecReader::<Endian>::new(bitbuf.into_bitvec());
	assert_eq!(bitbuf.read_float(), Some(69.69_f32));
	assert_eq!(bitbuf.read_float(), Some(0.01_f32));
	assert_eq!(bitbuf.read_float(), Some(f32::MAX));
	assert_eq!(bitbuf.read_float(), Some(f32::MIN));
	assert_eq!(bitbuf.read_float(), None::<f32>);
});

test!(read_double_lsb, read_double_msb, {
	use crate::{BitVecReader, BitVecWriter};

	let mut bitbuf = BitVecWriter::<Endian>::default();
	bitbuf.write_float(69.69_f64);
	bitbuf.write_float(0.01_f64);
	bitbuf.write_float(f64::MAX);
	bitbuf.write_float(f64::MIN);

	let mut bitbuf = BitVecReader::<Endian>::new(bitbuf.into_bitvec());
	assert_eq!(bitbuf.read_float(), Some(69.69_f64));
	assert_eq!(bitbuf.read_float(), Some(0.01_f64));
	assert_eq!(bitbuf.read_float(), Some(f64::MAX));
	assert_eq!(bitbuf.read_float(), Some(f64::MIN));
	assert_eq!(bitbuf.read_float(), None::<f64>);
});

test!(read_string_lsb, read_string_msb, {
	use crate::{BitVecReader, BitVecWriter};

	let mut bitbuf = BitVecWriter::<Endian>::default();
	bitbuf.write_string("Hello, world!");
	bitbuf.write_string("Hello, world!");
	bitbuf.write_string("Hello, world!");

	bitbuf.write_string_nul("Hello, world!");
	bitbuf.write_string_nul("Hello, world!");
	bitbuf.write_string_nul("Hello, world!");

	let mut bitbuf = BitVecReader::<Endian>::new(bitbuf.into_bitvec());
	assert_eq!(bitbuf.read_string("Hello, world!".len()), Some(Ok("Hello, world!".to_string())));
	assert_eq!(unsafe { bitbuf.read_string_unchecked("Hello, world!".len()) }, Some("Hello, world!".to_string()));
	assert_eq!(bitbuf.read_string_lossy("Hello, world!".len()), Some("Hello, world!".to_string()));

	assert_eq!(bitbuf.read_nul_string(), Some(Ok("Hello, world!".to_string())));
	assert_eq!(unsafe { bitbuf.read_nul_string_unchecked() }, Some("Hello, world!".to_string()));
	assert_eq!(bitbuf.read_nul_string_lossy(), Some("Hello, world!".to_string()));
});

test!(test_read_bytes_lsb, test_read_bytes_msb, {
	let mut bitbuf = BitVecWriter::<Endian>::default();
	bitbuf.write_bytes(b"Hello, world!");

	let mut bitbuf = BitVecReader::<Endian>::new(bitbuf.into_bitvec());
	assert_eq!(bitbuf.read_bytes(b"Hello, world!".len()), Some(b"Hello, world!".to_vec()));
});