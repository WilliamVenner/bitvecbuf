test!(write_bit_lsb, write_bit_msb, result, &[1u8], &[128u8], {
	use crate::BitVecWriter;

	let mut bitbuf = BitVecWriter::<Endian>::default();
	bitbuf.write_bit(true);
	assert_eq!(bitbuf.into_bytes(), result);
});

test!(write_uint_lsb, write_uint_msb, result, &[69, 0], &[4, 80], {
	use crate::BitVecWriter;

	let mut bitbuf = BitVecWriter::<Endian>::default();
	bitbuf.write_uint(69u16, 12);
	assert_eq!(bitbuf.into_bytes(), result);
});

test!(write_int_lsb, write_int_msb, result, &[187, 255], &[255, 187], {
	use crate::BitVecWriter;

	let mut bitbuf = BitVecWriter::<Endian>::default();
	bitbuf.write_int(-69_i16, 16);
	assert_eq!(bitbuf.into_bytes(), result);
});

test!(write_float_lsb, write_float_msb, result, &[72, 97, 139, 66], &[66, 139, 97, 72], {
	use crate::BitVecWriter;

	let mut bitbuf = BitVecWriter::<Endian>::default();
	bitbuf.write_float(69.69_f32);
	assert_eq!(bitbuf.into_bytes(), result);
});

test!(write_double_lsb, write_double_msb, result, &[0, 0, 0, 0, 0, 64, 81, 64], &[64, 81, 64, 0, 0, 0, 0, 0], {
	use crate::BitVecWriter;

	let mut bitbuf = BitVecWriter::<Endian>::default();
	bitbuf.write_float(69_f64);
	assert_eq!(bitbuf.into_bytes(), result);
});

test!(write_str_lsb, write_str_msb, {
	use crate::BitVecWriter;

	let mut bitbuf = BitVecWriter::<Endian>::default();
	bitbuf.write_string("Hello, world!");
	bitbuf.write_string_nul("Hello, world!");
	assert_eq!(bitbuf.into_bytes(), b"Hello, world!Hello, world!\0");
});

test!(write_bytes_lsb, write_bytes_msb, {
	use crate::BitVecWriter;

	let mut bitbuf = BitVecWriter::<Endian>::default();
	bitbuf.write_bytes(b"Hello, world!");
	assert_eq!(bitbuf.into_bytes(), b"Hello, world!");
});
