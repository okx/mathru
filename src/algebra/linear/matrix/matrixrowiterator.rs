use crate::algebra::abstr::{Field, Scalar, Zero};
use crate::algebra::linear::{Vector};
use std::slice::Iter;

pub struct MatrixRowIterator<'a, T>
{
    iter: Iter<'a, T>,
}

impl<'a, T> MatrixRowIterator<'a, T>
    where T: Zero
{
    pub fn new(iter: Iter<'a, T>) -> MatrixRowIterator<'a, T>
    {
        return MatrixRowIterator{iter};
    }
}

impl<'a, T> Iterator for MatrixRowIterator<'a, T> where T: Field + Scalar
{
    type Item = &'a Vector<T>;

    fn next(&'a mut self) -> Option<Self::Item>
    {
        Some(&self.vec)
    }
}
