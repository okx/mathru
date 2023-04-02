use crate::algebra::{
    abstr::{Field, Scalar},
    linear::{Matrix, Vector},
};
use std::ops::Mul;

/// Multiplies matrix by vector.
impl<'a, 'b, T> Mul<&'b Vector<T>> for &'a Matrix<T>
    where T: Field + Scalar
{
    type Output = Vector<T>;

    fn mul(self, v: &'b Vector<T>) -> Vector<T>
    {
        let (_self_m, self_n): (usize, usize) = self.dim();
        let (v_m, _v_n): (usize, usize) = v.dim();

        if self_n != v_m
        {
            panic!("Matrix and Vector dimension do not match");
        }

        let mut prod_data = Vec::with_capacity(self.m);

        for i in 0..self.m
        {
            let mut row_column_product: T = T::zero();
            for k in 0..self.n
            {
                row_column_product += self.data[k * self.m + i] * v[k];
            }
            prod_data.push(row_column_product);
        }

        Vector::new_column(prod_data)
    }
}


// Multiplies matrix by vector.
impl<T> Mul<Vector<T>> for Matrix<T> where T: Field + Scalar
{
    type Output = Vector<T>;

    fn mul(self, v: Vector<T>) -> Vector<T>
    {
        (&self) * (&v)
    }
}

impl<T> Mul<Matrix<T>> for Matrix<T>
    where T: Field + Scalar
{
    type Output = Matrix<T>;

    /// Multiplies two matrices
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let res_ref: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, -18.0, 49.0]);
    /// assert_eq!(res_ref, a * b);
    /// ```
    fn mul(self, rhs: Self) -> Self::Output
    {
        (&self).mul(&rhs)
    }
}

impl<'a, 'b, T> Mul<&'b Matrix<T>> for &'a Matrix<T>
    where T: Field + Scalar
{
    type Output = Matrix<T>;

    /// Multiplies two matrices
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let res_ref: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, -18.0, 49.0]);
    /// assert_eq!(res_ref, &a * &b);
    /// ```
    fn mul(self, rhs: &'b Matrix<T>) -> Self::Output
    {
        let (self_rows, self_cols) = self.dim();
        let (rhs_rows, rhs_cols) = rhs.dim();

        assert_eq!(self_cols, rhs_rows);

        let m = self_rows as i32;
        let n = rhs_cols as i32;
        let k = self_cols as i32;
        let mut c: Matrix<T> = Matrix::zero(m as usize, n as usize);

        T::xgemm('N' as u8,
                 'N' as u8,
                 m,
                 n,
                 k,
                 T::one(),
                 &self.data[..],
                 m,
                 &rhs.data[..],
                 k,
                 T::zero(),
                 &mut c.data[..],
                 m);

        return c;
    }
}

impl<'a, 'b, T> Mul<&'b Matrix<T>> for &'a mut Matrix<T>
    where T: Field + Scalar
{
    type Output = &'a mut Matrix<T>;

    /// Multiplies two matrices
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let mut a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let res_ref: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, -18.0, 49.0]);
    /// assert_eq!(res_ref, *(&mut a * &b));
    /// ```
    fn mul(self, rhs: &'b Matrix<T>) -> Self::Output
    {
        let (self_rows, self_cols) = self.dim();
        let (rhs_rows, rhs_cols) = rhs.dim();

        assert_eq!(self_cols, rhs_rows);

        let m = self_rows as i32;
        let n = rhs_cols as i32;
        let k = self_cols as i32;

        T::xgemm('N' as u8,
                 'N' as u8,
                 m,
                 n,
                 k,
                 T::one(),
                 &self.data.clone()[..],
                 m,
                 &rhs.data[..],
                 k,
                 T::zero(),
                 &mut self.data[..],
                 m);

        self
    }
}

impl<'a, 'b, T> Matrix<T>
    where T: Field + Scalar
{
    fn mul_scalar(mut self, s: &'b T) -> Matrix<T>
    {
        let (rows, cols): (usize, usize) = self.dim();
        //
        let m: i32 = rows as i32;
        let n: i32 = cols as i32;

        T::xscal(m * n,  *s,&mut self.data[..], 1);
        return self;
    }
}

//Multiplies matrix by scalar
impl<T> Mul<T> for Matrix<T>
    where T: Field + Scalar
{
    type Output = Matrix<T>;

    /// Multiplies a matrix with a scalar
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let f: f64 = 7.0;
    /// let res_ref: Matrix<f64> = Matrix::new(2, 2, vec![7.0, 0.0, 21.0, -49.0]);
    ///
    /// assert_eq!(res_ref, a * f);
    /// ```
    fn mul(self, s: T) -> Matrix<T>
    {
        self.mul_scalar(&s)
    }
}

// Multiplies matrix by scalar
impl<'a, 'b, T> Mul<&'b T> for &'a Matrix<T>
    where T: Field + Scalar
{
    type Output = Matrix<T>;

    /// Multiplies a matrix with a scalar
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let res_ref: Matrix<f64> = Matrix::new(2, 2, vec![4.0, 0.0, 12.0, -28.0]);
    ///
    /// assert_eq!(res_ref, &a * &4.0);
    /// ```
    fn mul(self, m: &'b T) -> Matrix<T>
    {
        return self.clone().mul_scalar(m);
    }
}

// Multiplies matrix by scalar
impl<'a, 'b, T> Mul<&'b T> for &'a mut Matrix<T>
    where T: Field + Scalar
{
    type Output = &'a mut Matrix<T>;

    /// Multiplies a matrix with a scalar
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let mut a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let res_ref: Matrix<f64> = Matrix::new(2, 2, vec![4.0, 0.0, 12.0, -28.0]);
    ///
    /// assert_eq!(res_ref, *(&mut a * &4.0));
    /// ```
    fn mul(self, m: &'b T) -> Self::Output
    {
        let _ = self.data.iter_mut().for_each(&|a: &mut T| *a *= *m );
        self
    }
}
