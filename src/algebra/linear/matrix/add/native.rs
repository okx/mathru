use crate::algebra::{
    abstr::{Field, Scalar},
    linear::Matrix,
};
use std::ops::Add;

impl<T> Add<Self> for Matrix<T>
    where T: Field + Scalar
{
    type Output = Matrix<T>;

    /// Adds two matrices
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    ///
    /// let c: Matrix<f64> = a + b;
    /// ```
    fn add(mut self, rhs: Self) -> Self::Output
    {
        let _ = (&mut self).add(&rhs);
        self
    }
}

///Adds two matrices
impl<'a, 'b, T> Add<&'b Matrix<T>> for &'a Matrix<T>
    where T: Field + Scalar
{
    type Output = Matrix<T>;

    /// Adds two matrices
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    ///
    /// let c: Matrix<f64> = &b + &a;
    /// ```
    fn add(self, rhs: &'b Matrix<T>) -> Self::Output
    {
        // assert_eq!(self.dim(), rhs.dim());
        let (m, n) = self.dim();
        Matrix { m,
                 n,
                 data: self.data
                           .iter()
                           .zip(rhs.data.iter())
                           .map(|(x, y)| *x + *y)
                           .collect::<Vec<T>>() }
    }
}

///Adds two matrices
impl<'a, 'b, T> Add<&'b Matrix<T>> for &'a mut Matrix<T>
    where T: Field + Scalar
{
    type Output = &'a mut Matrix<T>;

    /// Adds two matrices
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let mut a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    ///
    /// let _ = &mut a + &b;
    /// ```
    fn add(self, rhs: &'b Matrix<T>) -> Self::Output
    {
        // assert_eq!(self.dim(), rhs.dim());
        self.data.iter_mut().zip(rhs.data.iter()).for_each(|(x, y)| *x += *y);
        self
    }
}

/// Add scalar to matrix
impl<T> Add<T> for Matrix<T>
    where T: Field + Scalar
{
    type Output = Matrix<T>;

    /// Add a scalar to the matrix
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = a + -4.0;
    /// ```
    fn add(mut self, rhs: T) -> Self::Output
    {
        let _ = (&mut self).add(&rhs);
        self
    }
}

/// Add scalar to matrix
impl<'a, 'b, T> Add<&'a T> for &'a Matrix<T>
    where T: Field + Scalar
{
    type Output = Matrix<T>;

    /// Add a scalar to the matrix
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// &a + &-4.0;
    /// ```
    fn add(self, rhs: &'a T) -> Self::Output
    {
        let mut a: Matrix<T> = self.clone();
        let _ = &mut a + rhs;
        a
    }
}

/// Add scalar to matrix
impl<'a, 'b, T> Add<&'a T> for &'a mut Matrix<T>
    where T: Field + Scalar
{
    type Output = &'a mut Matrix<T>;

    /// Add a scalar to the matrix
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let mut a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// &mut a + &-4.0;
    /// ```
    fn add(self, rhs: &'a T) -> Self::Output
    {
        let _ = self.data.iter_mut().for_each(&|x: &mut T| *x += *rhs);
        self
    }
}
