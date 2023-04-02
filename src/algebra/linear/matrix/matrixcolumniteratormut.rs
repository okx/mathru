use crate::algebra::abstr::{Field, Scalar};
use crate::algebra::linear::Vector;
use std::slice::IterMut;

pub struct MatrixColumnIteratorMut<'a, T>
{
    iter: IterMut<'a, T>,
}

impl<'a, T> MatrixColumnIteratorMut<'a, T>
{
    pub fn new(iter: IterMut<'a, T>) -> MatrixColumnIteratorMut<'a, T>
    {
        return MatrixColumnIteratorMut{iter}
    }
}

impl<'a, T> Iterator for MatrixColumnIteratorMut<'a, T>
    where T: Field + Scalar
{
    type Item = &'a mut Vector<T>;

    fn next(&mut self) -> Option<Self::Item>
    {
        return Some(&mut vector![T::zero()]);
    }
}
