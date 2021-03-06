macro_rules! test {
	($test_lsb:ident, $test_msb:ident, $bench_lsb:ident, $bench_msb:ident, $code:block) => {
		#[test]
		fn $test_lsb() {
			#[allow(unused)]
			use crate::{BitVecReader, BitVecWriter};
			type Endian = bitvec::prelude::Lsb0;
			$code
		}

		#[test]
		fn $test_msb() {
			#[allow(unused)]
			use crate::{BitVecReader, BitVecWriter};
			type Endian = bitvec::prelude::Msb0;
			$code
		}

		#[cfg(all(feature = "nightly", test))]
		#[bench]
		fn $bench_lsb(b: &mut test::Bencher) {
			#[allow(unused)]
			use crate::{BitVecReader, BitVecWriter};
			type Endian = bitvec::prelude::Lsb0;
			b.iter(|| $code);
		}

		#[cfg(all(feature = "nightly", test))]
		#[bench]
		fn $bench_msb(b: &mut test::Bencher) {
			#[allow(unused)]
			use crate::{BitVecReader, BitVecWriter};
			type Endian = bitvec::prelude::Msb0;
			b.iter(|| $code);
		}
	};

	($test_lsb:ident, $test_msb:ident, $bench_lsb:ident, $bench_msb:ident, $result_var_name:ident, $lsb_result:expr, $msb_result:expr, $code:block) => {
		#[test]
		fn $test_lsb() {
			#[allow(unused)]
			use crate::{BitVecReader, BitVecWriter};
			type Endian = bitvec::prelude::Lsb0;
			let $result_var_name = $lsb_result;
			$code
		}

		#[test]
		fn $test_msb() {
			#[allow(unused)]
			use crate::{BitVecReader, BitVecWriter};
			type Endian = bitvec::prelude::Msb0;
			let $result_var_name = $msb_result;
			$code
		}

		#[cfg(all(feature = "nightly", test))]
		#[bench]
		fn $bench_lsb(b: &mut test::Bencher) {
			#[allow(unused)]
			use crate::{BitVecReader, BitVecWriter};
			type Endian = bitvec::prelude::Lsb0;
			let $result_var_name = $lsb_result;
			b.iter(|| $code);
		}

		#[cfg(all(feature = "nightly", test))]
		#[bench]
		fn $bench_msb(b: &mut test::Bencher) {
			#[allow(unused)]
			use crate::{BitVecReader, BitVecWriter};
			type Endian = bitvec::prelude::Msb0;
			let $result_var_name = $msb_result;
			b.iter(|| $code);
		}
	};
}

mod read;
mod write;

#[test]
fn test_empty_bitbuf() {
	use crate::BitVecWriter;
	use bitvec::prelude::Lsb0;

	let bitbuf = BitVecWriter::<Lsb0>::default();
	assert_eq!(bitbuf.into_bytes(), &[]);
}


#[test]
fn test_set_bits() {
	use crate::{BitVecWriter, BitVecReader};
	use bitvec::prelude::Lsb0;
	use bitvec::view::BitView;

	let mut bitbuf = BitVecWriter::<Lsb0>::default();
	bitbuf.write_uint(69_u32, 32);
	bitbuf.set_bits(0, 420_u32.view_bits());

	let mut bitbuf = BitVecReader::<Lsb0>::from_bytes(bitbuf.into_bytes());
	assert_eq!(bitbuf.read_uint(32), Some(420_u32));
}
