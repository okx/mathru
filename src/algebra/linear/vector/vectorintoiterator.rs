use crate::algebra::{
    abstr::{Field, Scalar},
    linear::matrix::MatrixIntoIterator,
};
use std::iter::Iterator;

pub struct VectorIntoIterator<T>
{
    iter: MatrixIntoIterator<T>,
}

impl<T> VectorIntoIterator<T>
{
    pub fn new(iter: MatrixIntoIterator<T>) -> VectorIntoIterator<T>
    {
        VectorIntoIterator{ iter }
    }
}

impl<T> Iterator for VectorIntoIterator<T> where T: Field + Scalar
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item>
    {
        self.iter.next()
    }
}
