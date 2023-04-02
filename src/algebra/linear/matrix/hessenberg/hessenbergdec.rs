use crate::algebra::linear::Matrix;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::clone::Clone;

/// Result of a Hessenberg decomposition
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct HessenbergDec<T>
{
    q: Matrix<T>,
    h: Matrix<T>,
}

impl<T> HessenbergDec<T>
{
    pub(super) fn new(q: Matrix<T>, h: Matrix<T>) -> HessenbergDec<T>
    {
        HessenbergDec { q, h }
    }

    pub fn q(self) -> Matrix<T>
    {
        self.q
    }

    pub fn h(self) -> Matrix<T>
    {
        self.h
    }

    pub fn qh(self) -> (Matrix<T>, Matrix<T>)
    {
        (self.q, self.h)
    }
}
