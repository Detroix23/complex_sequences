//! # Complex sequences.
//! src/support/defaults.rs

use complex;

/// 2 degree polynomial with `c` coefficient
#[allow(dead_code)]
pub fn polynomial2_c(z: complex::Algebraic, c: complex::Algebraic) -> complex::Algebraic {
	z * z + c
}

/// 3 degree polynomial with `c` coefficient.
#[allow(dead_code)]
pub fn polynomial3_c(z: complex::Algebraic, c: complex::Algebraic) -> complex::Algebraic {
	z * z + c
}

