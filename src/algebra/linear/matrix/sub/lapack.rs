use crate::algebra::{
    abstr::{Field, Scalar},
    linear::Matrix,
};
use std::ops::Sub;

impl<T> Sub for Matrix<T>
    where T: Field + Scalar
{
    type Output = Matrix<T>;

    /// Subtracts two matrices
    ///
    /// A = (a_{ij}) \in T^{m \times n}
    /// B = (b_{ij}) \in T^{m \times n}
    /// A - B = ( a_{ij} - b_{ij} )
    ///
    /// # Arguments
    ///
    /// rhs:
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    ///
    /// let c: Matrix<f64> = a - b;
    /// ```
    fn sub(self, rhs: Self) -> Self::Output
    {
        assert_eq!(self.dim(), rhs.dim());

        let (m, n): (usize, usize) = rhs.dim();

        let mut c: Matrix<T> = self;

        T::xaxpy((m * n) as i32, -T::one(), &rhs.data[..], 1, &mut c.data[..], 1);

        c
    }
}

impl<'a, 'b, T> Sub<&'b Matrix<T>> for &'a Matrix<T>
    where T: Field + Scalar
{
    type Output = Matrix<T>;

    fn sub(self, rhs: &'b Matrix<T>) -> Self::Output
    {
        assert_eq!(self.dim(), rhs.dim());

        let (m, n): (usize, usize) = rhs.dim();

        let mut c: Matrix<T> = self.clone();

        T::xaxpy((m * n) as i32, -T::one(), &rhs.data[..], 1, &mut c.data[..], 1);

        c
    }
}

impl<'a, 'b, T> Sub<&'b Matrix<T>> for &'a mut Matrix<T>
    where T: Field + Scalar
{
    type Output = &'a mut Matrix<T>;

    fn sub(self, rhs: &'b Matrix<T>) -> Self::Output
    {
        assert_eq!(self.dim(), rhs.dim());

        let (m, n): (usize, usize) = rhs.dim();

        T::xaxpy((m * n) as i32, -T::one(), &rhs.data[..], 1, &mut self.data[..], 1);

        self
    }
}



impl<T> Sub<T> for Matrix<T> where T: Field + Scalar
{
    type Output = Matrix<T>;

    /// Subtracts a scalar from all matrix elements
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = a - -4.0;
    /// ```
    fn sub(self, rhs: T) -> Self::Output
    {
        self.apply_mut(&|x: &T| -> T { *x - rhs})
    }
}

///
/// Subtracts scalar from all matrix elements
impl<'a, 'b, T> Sub<&'b T> for &'a Matrix<T>
    where T: Field + Scalar
{
    type Output = Matrix<T>;

    /// Subtracts a scalar value from all matrix elements
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = &a - &-4.0;
    /// ```
    fn sub(self, rhs: &T) -> Self::Output
    {
        self.apply(&|x: &T| -> T { x.clone() - rhs.clone() })
    }
}

///
/// Subtracts scalar from all matrix elements
impl<'a, 'b, T> Sub<&'b T> for &'a mut Matrix<T>
    where T: Field + Scalar
{
    type Output = &'a mut Matrix<T>;

    /// Subtracts a scalar value from all matrix elements
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = &a - &-4.0;
    /// ```
    fn sub(self, rhs: &T) -> Self::Output
    {
        self.data.iter_mut().for_each(|x: &mut T| *x -= *rhs);
        self
    }
}