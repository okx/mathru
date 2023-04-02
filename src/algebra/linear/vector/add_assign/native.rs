use crate::algebra::{
    abstr::{Field, Scalar},
    linear::{Vector},
};
use std::ops::AddAssign;

// Add vector to vector
impl<T> AddAssign<Vector<T>> for Vector<T>
    where T: Field + Scalar
{
    /// Add a vector to the vector
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let mut a: Vector<f64> = Vector::new_column(vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Vector<f64> = Vector::new_column(vec![2.0, 3.0, -5.0, 2.0]);
    /// a += b;
    /// ```
    fn add_assign(&mut self, rhs: Vector<T>)
    {
        self.data += rhs.data
    }
}


// Add scalar to vector
impl<T> AddAssign<T> for Vector<T>
    where T: Field + Scalar
{
    /// Add a scalar to the vector
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let mut a: Vector<f64> = Vector::new_column(vec![1.0, 0.0, 3.0, -7.0]);
    /// a += -4.0;
    /// ```
    fn add_assign(&mut self, rhs: T)
    {
        self.data += rhs
    }
}


