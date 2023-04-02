use crate::algebra::{
    abstr::{Field, Scalar},
    linear::Matrix,
};

use super::Transpose;


impl<T> Matrix<T>
{
    pub fn gcd(mut m: usize, mut n: usize) -> usize
    {
        while m != 0
        {
            let old_m: usize = m;
            m = n % m;
            n = old_m;
        }
        n
    }

    fn gather(&self, i: usize, j: usize, b: usize) -> usize
    {
        (i + (j / b)) % self.m
    }

    fn gather_sa(&self, i: usize, j: usize, a: usize) -> usize
    {
        (j + i * self.n - i / a) % self.m
    }

    fn scatter_da(&self, i: usize, j: usize, b: usize) -> usize
    {
        (((i + j / b) % self.m) + j * self.m) % self.n
    }
}

impl<T> Transpose for Matrix<T>
    where T: Field + Scalar
{
    type Output = Matrix<T>;

    /// Transpose a matrix without allocating memory for the transposed matrix
    ///
    /// catanzaro.name/papers/PPoPP-2014.pdf
    ///
    /// # Example
    ///
    /// ```
    /// #[macro_use]
    /// # extern crate mathru;
    /// # fn main()
    /// # {
    /// use mathru::algebra::linear::Matrix;
    /// use mathru::algebra::linear::matrix::Transpose;
    ///
    /// let mut uut: Matrix<f64> = Matrix::new(4, 2, vec![1.0, 0.0, 3.0, 0.0, 1.0, -7.0, 0.5, 0.25]);
    /// uut = uut.transpose();
    ///
    /// let refer: Matrix<f64> = Matrix::new(2, 4, vec![1.0, 1.0, 0.0, -7.0, 3.0, 0.5, 0.0, 0.25]);
    /// assert_relative_eq!(refer, uut);
    /// # }
    /// ```
    fn transpose(mut self) -> Matrix<T>
    {
        let gcdiv: usize = Matrix::<T>::gcd(self.m, self.n);

        std::mem::swap(&mut self.m, &mut self.n);

        let a: usize = self.m / gcdiv;
        let b: usize = self.n / gcdiv;

        let bigger: usize = self.m.max(self.n);

        let mut temp: Vec<T> = Vec::with_capacity(bigger);
        //Bad
        unsafe { temp.set_len(bigger) };

        if gcdiv > 1
        {
            for j in 0..self.n
            {
                for i in 0..self.m
                {
                    temp[i] = self[[self.gather(i, j, b), j]]
                }

                for i in 0..self.m
                {
                    self[[i, j]] = temp[i];
                }
            }
        }

        for i in 0..self.m
        {
            for j in 0..self.n
            {
                temp[self.scatter_da(i, j, b)] = self[[i, j]];
            }

            for j in 0..self.n
            {
                self[[i, j]] = temp[j];
            }
        }

        for j in 0..self.n
        {
            for i in 0..self.m
            {
                temp[i] = self[[self.gather_sa(i, j, a), j]];
            }

            for i in 0..self.m
            {
                self[[i, j]] = temp[i];
            }
        }

        self
    }
}