use crate::algebra::{
    abstr::{Field, Scalar},
    linear::{matrix::LUDec, Matrix},
};

use crate::algebra::abstr::Zero;

impl<T> Matrix<T> where T: Field + Scalar
{
    /// Decomposes the matrix into a upper and a lower matrix
    ///
    /// PA = LU
    ///
    /// # Arguments
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, -2.0, 3.0, -7.0]);
    ///
    /// let (l, u, p): (Matrix<f64>, Matrix<f64>, Matrix<f64>) = a.dec_lu().unwrap().lup();
    /// ```
    pub fn dec_lu(&self) -> Result<LUDec<T>, ()>
    {
        let (m, n): (usize, usize) = self.dim();
        assert_eq!(m, n);

        let (m, n): (usize, usize) = self.dim();
        let m_i32: i32 = m as i32;
        let n_i32: i32 = n as i32;

        let dim_min: i32 = m_i32.min(n_i32);
        let mut ipiv: Vec<i32> = vec![Zero::zero(); dim_min as usize];

        let mut info: i32 = 0;

        let mut self_data = self.clone().data;

        T::xgetrf(m_i32,
                  n_i32,
                  self_data.as_mut_slice(),
                  m_i32,
                  ipiv.as_mut_slice(),
                  &mut info);

        if info != 0
        {
            return Err(());
        }

        let mat: Matrix<T> = Matrix::new(m, n, self_data);
        let l: Matrix<T> = Matrix::l(mat.clone());
        let u: Matrix<T> = Matrix::u(mat.clone());
        let p: Matrix<T> = Matrix::p(ipiv);

        Ok(LUDec::new(l, u, p))
    }

    fn l(mut mat: Matrix<T>) -> Self
    {
        let (m, n): (usize, usize) = mat.dim();

        //fill upper triangle with zero
        for i in 0..m
        {
            for k in i..n
            {
                mat[[i, k]] = T::zero();
            }
        }

        //set diagonal to 1
        for i in 0..m
        {
            mat[[i, i]] = T::one();
        }

        mat
    }

    fn u(mut mat: Matrix<T>) -> Self
    {
        let (m, _n): (usize, usize) = mat.dim();

        //fill lower triangle with zero
        for i in 0..m
        {
            for k in 0..i
            {
                mat[[i, k]] = T::zero();
            }
        }

        mat
    }

    /// transforms a sequence of permutations to a permutation matrix
    fn p(per: Vec<i32>) -> Self
    {
        let length = per.len();

        let mut perm: Vec<usize> = vec![0; length];

        for i in 0..length
        {
            perm[i] = i;
        }

        for i in 0..length - 1
        {
            let temp = perm[(per[i] - 1) as usize];
            perm[(per[i] - 1) as usize] = perm[i];
            perm[i] = temp;
        }

        let mut p: Matrix<T> = Matrix::zero(length, length);

        for i in 0..length
        {
            let k = perm[i];
            p[[i, k]] = T::one();
        }

        p
    }
}
