use crate::{
    algebra::{
        abstr::{Field, Scalar},
        linear::{matrix::{Transpose, QRDec}, Matrix},
    },
    elementary::Power,
};
use crate::algebra::abstr::AbsDiffEq;

impl<T> Matrix<T>
    where T: Field + Scalar + Power + AbsDiffEq
{
    /// QR Decomposition with Givens rotations
    ///
    /// A = QR \
    /// Q is an orthogonal matrix \
    /// R is an upper triangular matrix \
    ///
    /// # Panics
    ///
    /// if A is not a square matrix
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, -2.0, 3.0, -7.0]);
    ///
    /// let (q, r): (Matrix<f64>, Matrix<f64>) = a.dec_qr().unwrap().qr();
    /// ```
    pub fn dec_qr(&self) -> Result<QRDec<T>, ()>
    {
        let (m, n) = self.dim();
        assert!(m >= n);

        let mut q: Matrix<T> = Matrix::one(self.m);
        let mut r: Matrix<T> = self.clone();

        for j in 0..self.n
        {
            for i in (j + 1..self.m).rev()
            {
                let a_jj: T = r[[j, j]];
                let a_ij: T = r[[i, j]];

                let p: T = (a_jj * a_jj + a_ij * a_ij).sqrt();

                if p.abs_diff_ne(&T::zero(), T::default_epsilon()) && a_jj.abs_diff_ne(&T::zero(), T::default_epsilon()) && a_ij.abs_diff_ne(&T::zero(), T::default_epsilon())
                {
                    let c: T = a_jj / p;
                    let s: T = -a_ij / p;
                    let g_ij: Matrix<T> = Matrix::givens(r.m, i, j, c, s);

                    r = &g_ij * &r;
                    q = &g_ij * &q;
                }
            }
        }
        q = q.transpose();
        Ok(QRDec::new(q, r))
    }
}
