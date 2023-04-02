use crate::algebra::{
    abstr::{Field, Scalar, AbsDiffEq},
    linear::{matrix::lu::LUDec, matrix::Inverse, Matrix},
};

impl<T> Inverse<T> for Matrix<T> where T: Field + Scalar + AbsDiffEq
{
    /// Inverse Matrix
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::{matrix::*, Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b_inv: Matrix<f64> = a.inv().unwrap();
    /// ```
    fn inv(&self) -> Result<Matrix<T>, ()>
    {
        self.inv_r()
    }
}

impl<T> Matrix<T> where T: Field + Scalar + AbsDiffEq
{
    pub fn inv_r(&self) -> Result<Matrix<T>, ()>
    {
        let lu_dec: LUDec<T> = self.dec_lu()?;
        lu_dec.inv()
    }
}
