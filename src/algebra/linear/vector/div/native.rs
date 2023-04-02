use crate::algebra::{
    abstr::{Field, Scalar},
    linear::{Vector},
};
use std::ops::Div;

impl<T> Div<T> for Vector<T>
    where T: Field + Scalar
{
    type Output = Vector<T>;

    /// Divides the vector elements with scalar values
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let a: Vector<f64> = Vector::new_column(vec![-5.0, -10.0, -15.0, -20.0]);
    /// let res_ref: Vector<f64> = Vector::new_column(vec![1.0, 2.0, 3.0, 4.0]);
    ///
    /// assert_eq!(res_ref, a / -5.0)
    /// ```
    fn div(self, rhs: T) -> Self::Output
    {
        Vector { data: &self.data / (&rhs) }
    }
}

impl<'a, T> Div<&T> for &'a Vector<T>
    where T: Field + Scalar
{
    type Output = Vector<T>;

    /// Divides the elements of a vector with the scalar value
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let a: Vector<f64> = Vector::new_column(vec![5.0, 10.0, 15.0, 20.0]);
    /// let res_ref: Vector<f64> = Vector::new_column(vec![1.0, 2.0, 3.0, 4.0]);
    ///
    /// assert_eq!(res_ref, a / 5.0)
    /// ```
    fn div(self, rhs: &T) -> Self::Output
    {
        Vector { data: (&self.data).div(rhs) }
    }
}
