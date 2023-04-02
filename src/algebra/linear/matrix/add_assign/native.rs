use crate::algebra::{
    abstr::{Field, Scalar},
    linear::Matrix,
};
use std::ops::AddAssign;

// Add scalar to matrix
impl<T> AddAssign<Matrix<T>> for Matrix<T>
    where T: Field + Scalar
{
    /// Add a scalar to the matrix
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let mut a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = Matrix::new(2, 2, vec![2.0, 3.0, -5.0, 2.0]);
    /// a += b;
    /// ```
    fn add_assign(&mut self, rhs: Matrix<T>)
    {
        self.data.iter_mut().zip(rhs.data.iter()).for_each(|(a, b)|
            *a += *b
        )
    }
}

// Add scalar to matrix
impl<T> AddAssign<T> for Matrix<T>
    where T: Field + Scalar
{
    /// Add a scalar to the matrix
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let mut a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// a += -4.0;
    /// ```
    fn add_assign(&mut self, rhs: T)
    {
        self.data.iter_mut().for_each(|a: &mut T|
            *a += rhs
        );
    }
}
