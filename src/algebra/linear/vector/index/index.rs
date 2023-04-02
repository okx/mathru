use std::ops::{Index, IndexMut};
use crate::algebra::linear::Vector;

impl<T> Index<usize> for Vector<T>
{
    type Output = T;

    /// Returns the component
    ///
    /// # Panics
    ///
    /// if index is out of bounds
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let mut a: Vector<f64> = Vector::new_row(vec![1.0, 0.0, 3.0, -2.0]);
    ///
    /// assert_eq!(-2.0, a[3])
    /// ```
    fn index(&self, index: usize) -> &Self::Output
    {
        let (m, n): (usize, usize) = self.data.dim();

        if m == 1
        {
            return &self.data[[0, index]]
        }

        if n == 1
        {
            return &self.data[[index, 0]]
        }
        panic!("")
    }
}

impl<'a, T> IndexMut<usize> for Vector<T>
{
    /// Returns the component
    ///
    /// # Panics
    ///
    /// if index is out of bounds
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let mut a: Vector<f64> = Vector::new_row(vec![1.0, 0.0, 3.0, -2.0]);
    ///
    /// assert_eq!(-2.0, a[3])
    /// ```
    fn index_mut(&mut self, index: usize) -> &mut Self::Output
    {
        let (m, n): (usize, usize) = self.data.dim();

        if m == 1
        {
            return &mut self.data[[0, index]]
        }

        if n == 1
        {
            return &mut self.data[[index, 0]]
        }
        panic!("")
    }
}