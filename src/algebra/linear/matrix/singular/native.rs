use crate::algebra::linear::{Matrix, Vector};
use crate::algebra::abstr::{Field, Scalar};
use crate::elementary::Power;
use crate::algebra::linear::matrix::Transpose;

impl<T> Matrix<T>
    where T: Field + Scalar + Power
{
    /// Computes the singular value decomposition
    ///
    /// M = U * S * V*
    ///
    /// # Return
    ///
    /// (u, s, v)
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let a: Matrix<f64> = Matrix::new(4,
    ///                                  4,
    ///                                  vec![4.0, 1.0, -2.0, 2.0, 1.0, 2.0, 0.0, -2.0, 0.0, 3.0,
    ///                                       -2.0, 2.0, 2.0, 1.0, -2.0, -1.0]);
    ///
    /// let (u, s, v): (Matrix<f64>, Matrix<f64>, Matrix<f64>) = a.dec_sv();
    /// ```
    pub fn dec_sv(&self) -> (Self, Self, Self)
    {
        let (mut u, mut b, mut v): (Matrix<T>, Matrix<T>, Matrix<T>) = self.householder_bidiag();

        let (_m, n): (usize, usize) = b.dim();
        let max_iterations: usize = 500 * n * n;

        let mut u_k: Matrix<T> = Matrix::one(n);

        for _k in 0..max_iterations
        {
            let (u_ks, b_k, v_k): (Matrix<T>, Matrix<T>, Matrix<T>) = Matrix::msweep(u_k, b, v);
            u_k = u_ks;
            b = b_k;
            v = v_k;
        }

        u *= u_k.transpose();

        let (b_m, _b_n): (usize, usize) = b.dim();

        // check that all singular values are positive
        for l in 0..b_m
        {
            if b[[l, l]] < T::zero()
            {
                b[[l, l]] = -b[[l, l]];
                let mut column_l: Vector<T> = u.get_column(l);
                column_l = &column_l * &-T::one();
                u.set_column(&column_l, l);
            }
        }

        // null all values beneath the diagonal
        for l in 0..b_m
        {
            for k in 0..b_m
            {
                if k != l
                {
                    b[[k, l]] = T::zero();
                }
            }
        }

        // sort singular values in descending order
        (u, b, v)
    }

    fn msweep(mut u: Matrix<T>,
              mut b: Matrix<T>,
              mut v: Matrix<T>)
              -> (Matrix<T>, Matrix<T>, Matrix<T>)
    {
        let (_m, n): (usize, usize) = b.dim();

        for k in 0..n - 1
        {
            let mut q: Matrix<T> = Matrix::one(n);

            // Construct matrix Q and multiply on the right by Q'.
            // Q annihilates both B(k-1,k+1) and B(k,k+1)
            // but makes B(k+1,k) non-zero.
            let (c_r, s_r, _r_r): (T, T, T) = Matrix::rot(b[[k, k]], b[[k, k + 1]]);

            q[[k, k]] = c_r;
            q[[k, k + 1]] = s_r;
            q[[k + 1, k]] = -s_r;
            q[[k + 1, k + 1]] = c_r;

            let q_t: Matrix<T> = q.clone().transpose();
            b = &b * &q_t;
            v = &v * &q_t;


            // Construct matrix Q and multiply on the left by Q.
            // Q annihilates B(k+1,k) but makes B(k,k+1) and
            // B(k,k+2) non-zero.
            let (c_l, s_l, _r_l): (T, T, T) = Matrix::rot(b[[k, k]], b[[k + 1, k]]);

            q[[k, k]] = c_l;
            q[[k, k + 1]] = s_l;
            q[[k + 1, k]] = -s_l;
            q[[k + 1, k + 1]] = c_l;

            b = &q * &b;
            u = &q * &u;
        }

        (u, b, v)
    }

    pub fn rot(f: T, g: T) -> (T, T, T)
    {
        if f == T::zero()
        {
            (T::zero(), T::one(), g)
        }
        else
        {
            let expo: T = T::from_f64(2.0);
            let sqrt: T = T::from_f64(0.5);
            if f.abs() > g.abs()
            {
                let t: T = g / f;
                let t1: T = (T::one() + t.pow(expo)).pow(sqrt);

                (T::one() / t1, t / t1, f * t1)
            }
            else
            {
                let t: T = f / g;
                let t1: T = (T::one() + t.pow(expo)).pow(sqrt);

                (t / t1, T::one() / t1, g * t1)
            }
        }
    }

    ///
    /// self is an m times n matrix with m >= n
    /// A = UBV^{T}
    /// U \in T^{m \times n}
    /// B \in T^{n \times n}
    /// V \in T^{n \times n}
    pub fn householder_bidiag(&self) -> (Self, Self, Self)
    {
        let (m, n): (usize, usize) = self.dim();
        if m < n
        {
            panic!("Read the API");
        }

        let mut u: Matrix<T> = Matrix::one(m);
        let mut v: Matrix<T> = Matrix::one(n);
        let mut a_i: Matrix<T> = self.clone();

        for i in 0..n - 1
        {
            // eliminate non-zeros below the diagonal
            // Keep the product U*B unchanged
            let u_x: Vector<T> = a_i.clone().get_column(i);
            let u_slice: Vector<T> = u_x.get_slice(i, m - 1);

            let u_i: Matrix<T> = Matrix::householder(&u_slice, 0);

            let a_i_slice = &u_i * &a_i.clone().get_slice(i, m - 1, i, n - 1);
            a_i = a_i.set_slice(&a_i_slice, i, i);
            let mut u_mi: Matrix<T> = Matrix::one(m);
            u_mi = u_mi.set_slice(&u_i, i, i);

            u = &u * &u_mi;

            //eliminate non-zeros to the right of the
            //superdiagonal by working with the transpose
            // Keep the product B*V' unchanged
            //B_T = B';
            if i < (n - 1)
            {
                let v_x: Vector<T> = a_i.get_row(i);
                let v_x_trans: Vector<T> = v_x.transpose();
                let v_x_trans_slice: Vector<T> = v_x_trans.get_slice(i + 1, n - 1);

                let v_i: Matrix<T> = Matrix::householder(&v_x_trans_slice, 0);

                let mut v_ni: Matrix<T> = Matrix::one(n);
                v_ni = v_ni.set_slice(&v_i, i + 1, i + 1);
                //let a_i_slice = &a_i.clone().get_slice(i+1, m - 1, i+1, n - 1) * &v_i;
                //a_i = a_i.set_slice(&a_i_slice, i+1, i+1);
                a_i = &a_i * &v_ni;

                v = &v * &v_ni;
            }
        }

        //Null all elements beneath the diagonal, and superdiagonal
        for i in 0..m
        {
            for k in 0..n
            {
                if k != i && k != (i + 1)
                {
                    a_i[[i, k]] = T::zero();
                }
            }
        }
        (u, a_i, v)
    }
}