use std::ops::{Index, IndexMut};
use crate::algebra::linear::Matrix;

impl<T> Index<[usize; 2]> for Matrix<T>
{
    type Output = T;

    fn index(&self, index: [usize; 2]) -> &Self::Output
    {
        &self.data[index[1] * self.m + index[0]]
    }
}

impl<T> IndexMut<[usize; 2]> for Matrix<T>
{
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output
    {
        &mut self.data[index[1] * self.m + index[0]]
    }
}