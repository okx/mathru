use crate::algebra::{
    abstr::{Field, Scalar},
    linear::{Vector},
};
use std::ops::Add;

impl<T> Add<Self> for Vector<T>
    where T: Field + Scalar
{
    type Output = Vector<T>;

    /// Adds two vectors
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let a: Vector<f64> = Vector::new_column(vec![1.0, 2.0, 3.0, 4.0]);
    /// let b: Vector<f64> = Vector::new_column(vec![3.0, -4.0, 5.0, 4.0]);
    /// let res_ref: Vector<f64> = Vector::new_column(vec![4.0, -2.0, 8.0, 8.0]);
    ///
    /// assert_eq!(res_ref, a + b)
    /// ```
    fn add(self, rhs: Self) -> Self::Output
    {
        &self + &rhs
    }
}

//c = a + b, a,b,c E T^m
impl<'a, 'b, T> Add<&'b Vector<T>> for &'a Vector<T>
    where T: Field + Scalar
{
    type Output = Vector<T>;

    /// Adds two vectors
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let a: Vector<f64> = Vector::new_column(vec![1.0, 2.0, 3.0, 4.0]);
    /// let b: Vector<f64> = Vector::new_column(vec![3.0, -4.0, 5.0, 4.0]);
    /// let res_ref: Vector<f64> = Vector::new_column(vec![4.0, -2.0, 8.0, 8.0]);
    ///
    /// assert_eq!(res_ref, &a + &b)
    /// ```
    fn add(self, rhs: &'b Vector<T>) -> Self::Output
    {
        Vector { data: (&self.data).add(&rhs.data) }
    }
}

// c = a + b, a,b,c E T^m
impl<'a, 'b, T> Add<&'b Vector<T>> for &'a mut Vector<T>
    where T: Field + Scalar
{
    type Output = &'a mut Vector<T>;

    /// Adds two vectors
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let mut a: Vector<f64> = Vector::new_column(vec![1.0, 2.0, 3.0, 4.0]);
    /// let b: Vector<f64> = Vector::new_column(vec![3.0, -4.0, 5.0, 4.0]);
    /// let res_ref: Vector<f64> = Vector::new_column(vec![4.0, -2.0, 8.0, 8.0]);
    ///
    /// assert_eq!(res_ref, *(&mut a + &b))
    /// ```
    fn add(self, rhs: &'b Vector<T>) -> Self::Output
    {
        let _ = &mut self.data + &rhs.data;
        self
    }
}

impl<T> Add<T> for Vector<T>
    where T: Field + Scalar
{
    type Output = Vector<T>;

    /// Adds a scalar to the vector
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let a: Vector<f64> = Vector::new_column(vec![1.0, 2.0, 3.0, 4.0]);
    /// let res_ref: Vector<f64> = Vector::new_column(vec![-4.0, -3.0, -2.0, -1.0]);
    ///
    /// assert_eq!(res_ref, a + -5.0)
    /// ```
    fn add(mut self, rhs: T) -> Self::Output
    {
        let _ = &mut self.data + &rhs;
        self
    }
}

impl<'a, 'b, T> Add<&'b T> for &'a Vector<T>
    where T: Field + Scalar
{
    type Output = Vector<T>;

    /// Adds a scalar to the vector
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let mut a: Vector<f64> = Vector::new_column(vec![1.0, 2.0, 3.0, 4.0]);
    /// let res_ref: Vector<f64> = Vector::new_column(vec![-4.0, -3.0, -2.0, -1.0]);
    ///
    /// assert_eq!(res_ref, &a + &-5.0)
    /// ```
    fn add(self, rhs: &'b T) -> Self::Output
    {
        let mut res: Vector<T> = self.clone();
        let _ = &mut res + rhs;
        res
    }
}

impl<'a, 'b, T> Add<&'b T> for &'a mut Vector<T>
    where T: Field + Scalar
{
    type Output = Self;

    /// Adds a scalar to the vector
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let mut a: Vector<f64> = Vector::new_column(vec![1.0, 2.0, 3.0, 4.0]);
    /// let res_ref: Vector<f64> = Vector::new_column(vec![-4.0, -3.0, -2.0, -1.0]);
    ///
    /// assert_eq!(res_ref, *(&mut a + &-5.0))
    /// ```
    fn add(self, rhs: &'b T) -> Self::Output
    {
        let _ = &mut self.data + rhs;
        self
    }
}
