// Copyright 2014 M. Rizky Luthfianto.
// Licensed under the MIT license (http://opensource.org/licenses/MIT)
// This file may not be copied, modified, or distributed
// except according to those terms.

use nalgebra::DMat;

lazy_static! {

	// taken from https://github.com/seqan/seqan/blob/master/include%2Fseqan%2Fscore%2Fscore_matrix_data.h#L806
	static ref ARRAY: [i32;729]=[
		 2,  0, -2,  0,  0, -3,  1, -1, -1, -2, -1, -2, -1,  0,  0,  1,  0, -2,  1,  1,  0,  0, -6, -3,  0,  0, -8,
		 0,  3, -4,  3,  3, -4,  0,  1, -2, -3,  1, -3, -2,  2, -1, -1,  1, -1,  0,  0, -1, -2, -5, -3,  2, -1, -8,
		-2, -4, 12, -5, -5, -4, -3, -3, -2, -4, -5, -6, -5, -4, -3, -3, -5, -4,  0, -2, -3, -2, -8,  0, -5, -3, -8,
		 0,  3, -5,  4,  3, -6,  1,  1, -2, -3,  0, -4, -3,  2, -1, -1,  2, -1,  0,  0, -1, -2, -7, -4,  3, -1, -8,
		 0,  3, -5,  3,  4, -5,  0,  1, -2, -3,  0, -3, -2,  1, -1, -1,  2, -1,  0,  0, -1, -2, -7, -4,  3, -1, -8,
		-3, -4, -4, -6, -5,  9, -5, -2,  1,  2, -5,  2,  0, -3, -2, -5, -5, -4, -3, -3, -2, -1,  0,  7, -5, -2, -8,
		 1,  0, -3,  1,  0, -5,  5, -2, -3, -4, -2, -4, -3,  0, -1,  0, -1, -3,  1,  0, -1, -1, -7, -5,  0, -1, -8,
		-1,  1, -3,  1,  1, -2, -2,  6, -2, -2,  0, -2, -2,  2, -1,  0,  3,  2, -1, -1, -1, -2, -3,  0,  2, -1, -8,
		-1, -2, -2, -2, -2,  1, -3, -2,  5,  4, -2,  2,  2, -2, -1, -2, -2, -2, -1,  0, -1,  4, -5, -1, -2, -1, -8,
		-2, -3, -4, -3, -3,  2, -4, -2,  4,  4, -3,  4,  3, -3, -1, -3, -2, -3, -2, -1, -1,  3, -4, -1, -3, -1, -8,
		-1,  1, -5,  0,  0, -5, -2,  0, -2, -3,  5, -3,  0,  1, -1, -1,  1,  3,  0,  0, -1, -2, -3, -4,  0, -1, -8,
		-2, -3, -6, -4, -3,  2, -4, -2,  2,  4, -3,  6,  4, -3, -1, -3, -2, -3, -3, -2, -1,  2, -2, -1, -3, -1, -8,
		-1, -2, -5, -3, -2,  0, -3, -2,  2,  3,  0,  4,  6, -2, -1, -2, -1,  0, -2, -1, -1,  2, -4, -2, -2, -1, -8,
		 0,  2, -4,  2,  1, -3,  0,  2, -2, -3,  1, -3, -2,  2,  0,  0,  1,  0,  1,  0,  0, -2, -4, -2,  1,  0, -8,
		 0, -1, -3, -1, -1, -2, -1, -1, -1, -1, -1, -1, -1,  0, -1, -1, -1, -1,  0,  0, -1, -1, -4, -2, -1, -1, -8,
		 1, -1, -3, -1, -1, -5,  0,  0, -2, -3, -1, -3, -2,  0, -1,  6,  0,  0,  1,  0, -1, -1, -6, -5,  0, -1, -8,
		 0,  1, -5,  2,  2, -5, -1,  3, -2, -2,  1, -2, -1,  1, -1,  0,  4,  1, -1, -1, -1, -2, -5, -4,  3, -1, -8,
		-2, -1, -4, -1, -1, -4, -3,  2, -2, -3,  3, -3,  0,  0, -1,  0,  1,  6,  0, -1, -1, -2,  2, -4,  0, -1, -8,
		 1,  0,  0,  0,  0, -3,  1, -1, -1, -2,  0, -3, -2,  1,  0,  1, -1,  0,  2,  1,  0, -1, -2, -3,  0,  0, -8,
		 1,  0, -2,  0,  0, -3,  0, -1,  0, -1,  0, -2, -1,  0,  0,  0, -1, -1,  1,  3,  0,  0, -5, -3, -1,  0, -8,
		 0, -1, -3, -1, -1, -2, -1, -1, -1, -1, -1, -1, -1,  0, -1, -1, -1, -1,  0,  0, -1, -1, -4, -2, -1, -1, -8,
		 0, -2, -2, -2, -2, -1, -1, -2,  4,  3, -2,  2,  2, -2, -1, -1, -2, -2, -1,  0, -1,  4, -6, -2, -2, -1, -8,
		-6, -5, -8, -7, -7,  0, -7, -3, -5, -4, -3, -2, -4, -4, -4, -6, -5,  2, -2, -5, -4, -6, 17,  0, -6, -4, -8,
		-3, -3,  0, -4, -4,  7, -5,  0, -1, -1, -4, -1, -2, -2, -2, -5, -4, -4, -3, -3, -2, -2,  0, 10, -4, -2, -8,
		 0,  2, -5,  3,  3, -5,  0,  2, -2, -3,  0, -3, -2,  1, -1,  0,  3,  0,  0, -1, -1, -2, -6, -4,  3, -1, -8,
		 0, -1, -3, -1, -1, -2, -1, -1, -1, -1, -1, -1, -1,  0, -1, -1, -1, -1,  0,  0, -1, -1, -4, -2, -1, -1, -8,
		-8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8,  1,
	];

	static ref MAT: DMat<i32> = DMat::from_col_vec(27, 27, &*ARRAY);
}

#[inline]
fn lookup(number: u8) -> usize {
	if      number==b'Y' { 23 as usize }
	else if number==b'Z' { 24 as usize }
	else if number==b'X' { 25 as usize }
	else if number==b'*' { 26 as usize }
	else { (number-65) as usize }
}

pub fn pam250(a: u8, b: u8) -> i32 {
	let a = lookup(a);
	let b = lookup(b);

	MAT[(a, b)]
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_pam250() {
		let score1 = pam250(b'A',b'A');
		assert_eq!(score1, 2);
		let score2 = pam250(b'*',b'*');
		assert_eq!(score2, 1);
		let score3 = pam250(b'A',b'*');
		assert_eq!(score3, -8);
		let score4 = pam250(b'*',b'*');
		assert_eq!(score4, 1);
		let score5 = pam250(b'X',b'X');
		assert_eq!(score5, -1);
		let score6 = pam250(b'X',b'Z');
		assert_eq!(score6, -1);
	}
}
