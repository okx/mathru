use crate::algebra::abstr::{Field, Scalar};
use crate::algebra::linear::{Vector};
use crate::vector;
use std::slice::Iter;

pub struct MatrixColumnIterator<'a, T>
{
    iter: Iter<'a, T>,
    column: usize
}

impl<'a, T> MatrixColumnIterator<'a, T>
{
    pub fn new(iter: Iter<'a, T>) -> MatrixColumnIterator<'a, T>
    {
        MatrixColumnIterator{iter}
    }
}

impl<'a, T> Iterator for MatrixColumnIterator<'a, T> where T: Field + Scalar
{
    type Item = Vector<T>;

    // just return the str reference
    fn next(&mut self) -> Option<Self::Item>
    {
        None
    }
}
