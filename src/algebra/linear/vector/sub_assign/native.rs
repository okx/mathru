use crate::algebra::{
    abstr::{Field, Scalar},
    linear::{Vector},
};
use std::ops::SubAssign;

// Subtract vector from vector
impl<T> SubAssign<Vector<T>> for Vector<T>
    where T: Field + Scalar
{
    /// Subtract a vector from a vector
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let mut a: Vector<f64> = Vector::new_column(vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Vector<f64> = Vector::new_column(vec![2.0, 3.0, -5.0, 2.0]);
    /// a -= b;
    /// ```
    fn sub_assign(&mut self, rhs: Vector<T>)
    {
        self.data -= rhs.data
    }
}

// Subtract scalar from vector
impl<T> SubAssign<T> for Vector<T>
    where T: Field + Scalar
{
    /// Subtract a scalar from a vector
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let mut a: Vector<f64> = Vector::new_column(vec![1.0, 0.0, 3.0, -7.0]);
    /// a -= -4.0;
    /// ```
    fn sub_assign(&mut self, rhs: T)
    {
        self.data -= rhs
    }
}
