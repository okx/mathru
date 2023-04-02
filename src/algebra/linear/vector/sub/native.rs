use crate::algebra::{
    abstr::{Field, Scalar},
    linear::{Vector},
};
use std::ops::Sub;

//c = a - b , a,b,c E T^m
impl<T> Sub<Vector<T>> for Vector<T>
    where T: Field + Scalar
{
    type Output = Vector<T>;

    /// Subtracts two vectors
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let a: Vector<f64> = Vector::new_column(vec![1.0, 2.0, 3.0, 4.0]);
    /// let b: Vector<f64> = Vector::new_column(vec![3.0, -4.0, 5.0, 4.0]);
    /// let res_ref: Vector<f64> = Vector::new_column(vec![-2.0, 6.0, -2.0, 0.0]);
    ///
    /// assert_eq!(res_ref, a - b)
    /// ```
    fn sub(self, rhs: Vector<T>) -> Self::Output
    {
        &self - &rhs
    }
}

impl<'a, 'b, T> Sub<&'b Vector<T>> for &'a Vector<T>
    where T: Field + Scalar
{
    type Output = Vector<T>;

    /// Subtracts two vectors
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let a: Vector<f64> = Vector::new_column(vec![1.0, 2.0, 3.0, 4.0]);
    /// let b: Vector<f64> = Vector::new_column(vec![3.0, -4.0, 5.0, 4.0]);
    /// let res_ref: Vector<f64> = Vector::new_column(vec![-2.0, 6.0, -2.0, 0.0]);
    ///
    /// assert_eq!(res_ref, &a - &b)
    /// ```
    fn sub(self, rhs: &'b Vector<T>) -> Self::Output
    {
        Vector { data: (&self.data).sub(&rhs.data) }
    }
}

impl<'a, 'b, T> Sub<&'b Vector<T>> for &'a mut Vector<T>
    where T: Field + Scalar
{
    type Output = &'a mut Vector<T>;

    /// Subtracts two vectors
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let mut a: Vector<f64> = Vector::new_column(vec![1.0, 2.0, 3.0, 4.0]);
    /// let b: Vector<f64> = Vector::new_column(vec![3.0, -4.0, 5.0, 4.0]);
    /// let res_ref: Vector<f64> = Vector::new_column(vec![-2.0, 6.0, -2.0, 0.0]);
    /// &mut a - &b;
    /// assert_eq!(res_ref, a)
    /// ```
    fn sub(self, rhs: &'b Vector<T>) -> Self::Output
    {
        let _ = &mut self.data - &rhs.data;
        self
    }
}

impl<T> Sub<T> for Vector<T>
    where T: Field + Scalar
{
    type Output = Vector<T>;

    /// Subtracts a scalar value from all vector elements
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let a: Vector<f64> = Vector::new_column(vec![1.0, 2.0, 3.0, 4.0]);
    /// let res_ref: Vector<f64> = Vector::new_column(vec![6.0, 7.0, 8.0, 9.0]);
    ///
    /// assert_eq!(res_ref, a - -5.0)
    /// ```
    fn sub(mut self, rhs: T) -> Self::Output
    {
        self.data = (&self.data).sub(&rhs);
        self
    }
}

impl<'a, T> Sub<&T> for &'a Vector<T>
    where T: Field + Scalar
{
    type Output = Vector<T>;

    /// Subtract a scalar from vector elements
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let a: Vector<f64> = Vector::new_column(vec![1.0, 2.0, 3.0, 4.0]);
    /// let res_ref: Vector<f64> = Vector::new_column(vec![-4.0, -3.0, -2.0, -1.0]);
    ///
    /// assert_eq!(res_ref, a - 5.0)
    /// ```
    fn sub(self, rhs: &T) -> Self::Output
    {
        Vector { data: (&self.data).sub(rhs) }
    }
}

impl<'a, T> Sub<&T> for &'a mut Vector<T>
    where T: Field + Scalar
{
    type Output = &'a mut Vector<T>;

    /// Subtract a scalar from vector elements
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let mut a: Vector<f64> = Vector::new_column(vec![1.0, 2.0, 3.0, 4.0]);
    /// let res_ref: Vector<f64> = Vector::new_column(vec![-4.0, -3.0, -2.0, -1.0]);
    ///
    /// assert_eq!(res_ref, *(&mut a - &5.0))
    /// ```
    fn sub(self, rhs: &T) -> Self::Output
    {
        let _ = &mut self.data - rhs;
        self
    }
}

