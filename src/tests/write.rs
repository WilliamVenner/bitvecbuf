test!(
	test_write_bit_lsb,
	test_write_bit_msb,
	bench_write_bit_lsb,
	bench_write_bit_msb,
	result,
	&[1u8],
	&[128u8],
	{
		let mut bitbuf = BitVecWriter::<Endian>::default();
		bitbuf.write_bit(true);
		assert_eq!(bitbuf.into_bytes(), result);
	}
);

test!(
	test_write_uint_lsb,
	test_write_uint_msb,
	bench_write_uint_lsb,
	bench_write_uint_msb,
	result,
	&[69, 0],
	&[4, 80],
	{
		let mut bitbuf = BitVecWriter::<Endian>::default();
		bitbuf.write_uint(69u16, 12);
		assert_eq!(bitbuf.into_bytes(), result);
	}
);

test!(
	test_write_int_lsb,
	test_write_int_msb,
	bench_write_int_lsb,
	bench_write_int_msb,
	result,
	&[187, 255],
	&[255, 187],
	{
		let mut bitbuf = BitVecWriter::<Endian>::default();
		bitbuf.write_int(-69_i16, 16);
		assert_eq!(bitbuf.into_bytes(), result);
	}
);

test!(
	test_write_float_lsb,
	test_write_float_msb,
	bench_write_float_lsb,
	bench_write_float_msb,
	result,
	&[72, 97, 139, 66],
	&[66, 139, 97, 72],
	{
		let mut bitbuf = BitVecWriter::<Endian>::default();
		bitbuf.write_float(69.69_f32);
		assert_eq!(bitbuf.into_bytes(), result);
	}
);

test!(
	test_write_double_lsb,
	test_write_double_msb,
	bench_write_double_lsb,
	bench_write_double_msb,
	result,
	&[0, 0, 0, 0, 0, 64, 81, 64],
	&[64, 81, 64, 0, 0, 0, 0, 0],
	{
		let mut bitbuf = BitVecWriter::<Endian>::default();
		bitbuf.write_float(69_f64);
		assert_eq!(bitbuf.into_bytes(), result);
	}
);

test!(
	test_write_str_lsb,
	test_write_str_msb,
	bench_write_str_lsb,
	bench_write_str_msb,
	{
		let mut bitbuf = BitVecWriter::<Endian>::default();
		bitbuf.write_string("Hello, world!");
		bitbuf.write_string_nul("Hello, world!");
		assert_eq!(bitbuf.into_bytes(), b"Hello, world!Hello, world!\0");
	}
);

test!(
	test_write_bytes_lsb,
	test_write_bytes_msb,
	bench_write_bytes_lsb,
	bench_write_bytes_msb,
	{
		let mut bitbuf = BitVecWriter::<Endian>::default();
		bitbuf.write_bytes(b"Hello, world!");
		assert_eq!(bitbuf.into_bytes(), b"Hello, world!");
	}
);
