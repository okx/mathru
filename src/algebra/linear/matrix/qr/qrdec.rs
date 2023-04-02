use crate::algebra::linear::Matrix;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::clone::Clone;

/// QR decomposition
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct QRDec<T>
{
    q: Matrix<T>,
    r: Matrix<T>,
}

impl<T> QRDec<T>
{
    pub(super) fn new(q: Matrix<T>, r: Matrix<T>) -> QRDec<T>
    {
        QRDec { q, r }
    }

    /// Return the q matrix of the QR decomposition
    ///
    /// # Arguments
    ///
    /// * `self`
    pub fn q(self) -> Matrix<T>
    {
        self.q
    }

    /// Return the r matrix of the qr decomposition
    ///
    /// # Re
    pub fn r(self) -> Matrix<T>
    {
        self.r
    }

    pub fn qr(self) -> (Matrix<T>, Matrix<T>)
    {
        (self.q, self.r)
    }
}
