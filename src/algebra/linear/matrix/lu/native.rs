use crate::algebra::{
    abstr::{Field, Scalar},
    linear::{matrix::LUDec, Matrix},
};

impl<T> Matrix<T> where
    T: Field + Scalar
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

        let mut l: Matrix<T> = Matrix::one(self.m);
        let mut u: Matrix<T> = Matrix::one(self.n);
        let mut p: Matrix<T> = Matrix::one(self.m);

        let mut a: Matrix<T> = self.clone();

        for i in 0..a.m
        {
            //pivoting
            let mut max: T = T::zero();
            let mut i_max: usize = i;

            for l in i ..a.m
            {
                let p_cand: T =  a[[l, i]].abs();
                if p_cand > max
                {
                    max = p_cand;
                    i_max = l;
                }
            }

            if i != i_max
            {
                a.swap_rows(i, i_max);
                p.swap_rows(i, i_max);
            }

            for j in (i + 1)..a.n
            {
                let f: T = if a[[i, i]].clone() != T::zero()
                {
                    a[[j, i]] / a[[i, i]]
                }
                else
                {
                    a[[j, i]]
                };

                for k in (i + 1)..a.n
                {
                    a[[j, k]] = a[[j, k]] - f * a[[i, k]];
                }
                a[[j, i]] = f;
            }
        }

        for i in 1..a.n
        {
            for j in 0..i
            {
                l.data[j * a.m + i] = a.data[j * a.m + i];
            }
        }

        for i in 0..a.n
        {
            for k in i..a.n
            {
                u.data[k * a.m + i] = a.data[k * a.m + i];
            }
        }

        Ok(LUDec::new(l, u, p))
    }
}
