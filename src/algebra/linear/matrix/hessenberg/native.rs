use crate::{
    algebra::{
        abstr::{Field, Scalar},
        linear::{matrix::{Transpose, HessenbergDec}, Matrix, Vector},
    },
    elementary::Power,
};

impl<T> Matrix<T> where T: Field + Scalar + Power
{
    /// Decomposes self in to the M
    ///
    /// q * h * q^T = self
    ///
    /// # Arguments
    ///
    /// # Return
    ///
    /// (q, h)
    ///
    /// # Panics
    ///
    /// if M is not a square matrix
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let a: Matrix<f64> = Matrix::new(3, 3, vec![1.0, 5.0, 3.0, 1.0, 0.0, -7.0, 3.0, 8.0, 9.0]);
    /// let (q, h): (Matrix<f64>, Matrix<f64>) = a.dec_hessenberg().qh();
    /// ```
    pub fn dec_hessenberg(&self) -> HessenbergDec<T>
    {
        let (m, n): (usize, usize) = self.dim();
        assert_eq!(m, n, "Unable to compute the hessenberg decomposition of a non-square matrix");
        assert_ne!(m, 0, "Unable to compute the hessenberg decomposition of an empty matrix.");

        let (m, _n): (usize, usize) = self.dim();

        let mut q: Matrix<T> = Matrix::one(m);
        let mut h: Matrix<T> = self.clone();

        for k in 1..m-1
        {
            let v: Vector<T> = h.get_column(k - 1);

            let househ: Matrix<T> = Matrix::householder(&v, k);
            h = &househ.clone().transpose() * &h;
            q = &househ * &q;
            h = &h.clone() * &househ;
        }

        HessenbergDec::new(q, h)
    }
}
