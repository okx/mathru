use crate::algebra::linear::Matrix;
use crate::elementary::Power;
use crate::algebra::abstr::{Field, Scalar};

impl<T> Matrix<T>
    where T: Field + Scalar + Power
{
    /// Calculates the determinant
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, -2.0, 3.0, -7.0]);
    /// let det: f64 = a.det();
    /// assert_eq!(det, -1.0)
    /// ```
    pub fn det(&self) -> T
    {
        assert_eq!(self.m, self.n);

        if self.m == 1
        {
            return self[[0, 0]];
        }

        if self.m == 2
        {
            let a_11: T = self[[0, 0]];
            let a_12: T = self[[0, 1]];
            let a_21: T = self[[1, 0]];
            let a_22: T = self[[1, 1]];
            return a_11 * a_22 - a_12 * a_21;
        }

        let (_l, u, p) = match self.dec_lu()
        {
            Err(_e) => return T::zero(),
            Ok(dec) => dec.lup(),
        };

        let mut det: T = T::one();

        for i in 0..self.m
        {
            det *= u[[i, i]];
        }

        let mut counter: usize = 0;
        for i in 0..self.m
        {
            if p[[i, i]] != T::one()
            {
                counter += 1;
            }
        }

        let mut perm: T = T::one();
        if counter != 0
        {
            perm = (-T::one()).pow(T::from_u128(counter as u128 - 1));
        }

        perm * det
    }
}