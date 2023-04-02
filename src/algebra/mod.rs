//! Algebra
//!
//! This module provides the two modules linear for linear aglebra and abstr for abstract algebra.
//! 
//! The linear algebra module supports BLAS/LAPACK if it is enable via features. See README.md for more details
//! how to enable BLAS/LAPACK support.
//! 
//! Fore more information: <br>
//! <https://en.wikipedia.org/wiki/Algebra>

#[macro_use]
pub mod linear;
pub mod abstr;
