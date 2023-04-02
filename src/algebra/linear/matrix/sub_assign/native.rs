use crate::algebra::{
    abstr::{Field, Scalar},
    linear::Matrix,
};
use std::ops::SubAssign;


// Subtract matrix from a matrix
impl<T> SubAssign<Matrix<T>> for Matrix<T>
    where T: Field + Scalar
{
    /// Subtract matrix from a matrix
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let mut a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = Matrix::new(2, 2, vec![2.0, 3.0, -5.0, 2.0]);
    /// a -= b;
    /// ```
    fn sub_assign(&mut self, rhs: Matrix<T>)
    {
        self.data.iter_mut().zip(rhs.data.iter()).for_each(|(a, b)|
            *a -= *b
        )
    }
}

// Subtract scalar from matrix
impl<T> SubAssign<T> for Matrix<T>
    where T: Field + Scalar
{
    /// Subtract a scalar from matrix
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let mut a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// a += -4.0;
    /// ```
    fn sub_assign(&mut self, rhs: T)
    {
        self.data.iter_mut().for_each(|a: &mut T| *a -= rhs);
    }
}
