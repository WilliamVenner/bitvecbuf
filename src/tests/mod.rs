macro_rules! test {
	($lsb:ident, $msb:ident, $test:block) => {
		#[test]
		fn $lsb() {
			#[allow(unused)]
			use crate::{BitVecReader, BitVecWriter};
			type Endian = bitvec::prelude::Lsb0;
			$test
		}

		#[test]
		fn $msb() {
			#[allow(unused)]
			use crate::{BitVecReader, BitVecWriter};
			type Endian = bitvec::prelude::Msb0;
			$test
		}
	};

	($lsb:ident, $msb:ident, $result_var_name:ident, $lsb_result:expr, $msb_result:expr, $test:block) => {
		#[test]
		fn $lsb() {
			#[allow(unused)]
			use crate::{BitVecReader, BitVecWriter};
			type Endian = bitvec::prelude::Lsb0;
			let $result_var_name = $lsb_result;
			$test
		}

		#[test]
		fn $msb() {
			#[allow(unused)]
			use crate::{BitVecReader, BitVecWriter};
			type Endian = bitvec::prelude::Msb0;
			let $result_var_name = $msb_result;
			$test
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