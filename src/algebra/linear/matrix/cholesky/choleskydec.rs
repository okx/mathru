#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::algebra::linear::Matrix;
use std::clone::Clone;


/// Result of a Cholesky decomposition
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct CholeskyDec<T>
{
    l: Matrix<T>,
}

impl<T> CholeskyDec<T>
{
    pub fn new(m: Matrix<T>) -> CholeskyDec<T>
    {
        CholeskyDec { l: m }
    }
}

impl<T> CholeskyDec<T>
{
    /// Return the l matrix
    pub fn l(self) -> Matrix<T>
    {
        self.l
    }
}
