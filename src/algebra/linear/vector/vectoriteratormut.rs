use crate::algebra::{
    abstr::{Field, Scalar},
    linear::matrix::MatrixIteratorMut,
};

pub struct VectorIteratorMut<'a, T>
{
    iter: MatrixIteratorMut<'a, T>,
}

impl<'a, T> VectorIteratorMut<'a, T>
{
    pub fn new(iter: MatrixIteratorMut<'a, T>) -> VectorIteratorMut<'a, T>
    {
       VectorIteratorMut{ iter }
    }
}

impl<'a, T> Iterator for VectorIteratorMut<'a, T> where T: Field + Scalar
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item>
    {
        self.iter.next()
    }
}
