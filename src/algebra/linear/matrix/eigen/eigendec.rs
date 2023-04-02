use crate::algebra::linear::{Matrix, Vector};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::clone::Clone;

/// Result of a Eigen decomposition
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct EigenDec<T>
{
    value: Vector<T>,
    _vector: Matrix<T>,
}

impl<T> EigenDec<T>
{
    pub(super) fn new(value: Vector<T>, vector: Matrix<T>) -> EigenDec<T>
    {
        EigenDec { value, _vector: vector }
    }

    pub fn value(self) -> Vector<T>
    {
        self.value
    }

    pub fn vector(self) -> Matrix<T>
    {
        unimplemented!();
        // return self.vector;
    }

    pub fn pair(self) -> (Vector<T>, Matrix<T>)
    {
        unimplemented!();
        // return (self.value, self.vector);
    }
}
