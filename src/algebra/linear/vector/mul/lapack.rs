use crate::algebra::{
    abstr::{Field, Scalar},
    linear::{Matrix, Vector},
};
use std::ops::Mul;

impl<T> Mul<Matrix<T>> for Vector<T>
    where T: Field + Scalar
{
    type Output = Vector<T>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output
    {
        &self * &rhs
    }
}

impl<'a, 'b, T> Mul<&'b Matrix<T>> for &'a Vector<T>
    where T: Field + Scalar
{
    type Output = Vector<T>;

    fn mul(self, rhs: &'b Matrix<T>) -> Self::Output
    {
        let (rhs_m, _): (usize, usize) = rhs.dim();
        let (_m, n): (usize, usize) = self.dim();

        if n != rhs_m
        {
            panic!("Vector and matrix dimensions do not match");
        }
        Vector::new_row( (&self.data * rhs).convert_to_vec())
    }
}

impl<T> Mul<T> for Vector<T>
    where T: Field + Scalar
{
    type Output = Vector<T>;

    /// Multiplies the vector elements with a scalar value
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let a: Vector<f64> = Vector::new_column(vec![1.0, 2.0, 3.0, 4.0]);
    /// let res_ref: Vector<f64> = Vector::new_column(vec![-5.0, -10.0, -15.0, -20.0]);
    ///
    /// assert_eq!(res_ref, a * -5.0)
    /// ```
    fn mul(self, rhs: T) -> Self::Output
    {
        Vector { data: &self.data * (&rhs) }
    }
}

impl<'a, T> Mul<&T> for &'a Vector<T>
    where T: Field + Scalar
{
    type Output = Vector<T>;

    /// Multiplies the vector elements with the scalar value
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let a: Vector<f64> = Vector::new_column(vec![1.0, 2.0, 3.0, 4.0]);
    /// let res_ref: Vector<f64> = Vector::new_column(vec![5.0, 10.0, 15.0, 20.0]);
    ///
    /// assert_eq!(res_ref, a * 5.0)
    /// ```
    fn mul(self, rhs: &T) -> Self::Output
    {
        Vector { data: &self.data * rhs }
    }
}

impl<'a, 'b, T> Mul<&'b T> for &'a mut Vector<T>
    where T: Field + Scalar
{
    type Output = &'a mut Vector<T>;

    /// Multiplies the vector elements with the scalar value
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let mut a: Vector<f64> = Vector::new_column(vec![1.0, 2.0, 3.0, 4.0]);
    /// let res_ref: Vector<f64> = Vector::new_column(vec![5.0, 10.0, 15.0, 20.0]);
    ///
    /// assert_eq!(res_ref, *(&mut a * &5.0))
    /// ```
    fn mul(self, rhs: &'b T) -> Self::Output
    {
        let _ = &mut self.data * rhs;
        self
    }
}
