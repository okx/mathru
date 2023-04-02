use crate::algebra::linear::{Vector, Matrix};
use super::Substitute;
use crate::algebra::abstr::{Field, Scalar, AbsDiffEq};

impl<T> Substitute<Vector<T>> for Matrix<T> where T: Field + Scalar + AbsDiffEq
{
    fn substitute_forward(&self, a: Vector<T>) -> Result<Vector<T>, ()>
    {
        let mut b: Vector<T> = a;
        for k in 0..self.n
        {

            if self[[k, k]].abs_diff_eq(&T::zero(), T::default_epsilon())
            {
                // free variable
                // if b.get(k).abs_diff_eq(&T::zero(), T::default_epsilon())
                // {
                //     *b.get_mut(k) = T::one();
                // }
                // else
                // {
                //     return Err(())
                // }
            }
            else
            {
                for l in 0..k
                {
                    b[k] = b[k] - self[[k, l]] * b[l];
                }
                b[k] /= self[[k, k]];
            }
        }

        Ok(b)
    }

    fn substitute_backward(&self, c: Vector<T>) -> Result<Vector<T>, ()>
    {
        let mut b: Vector<T> = c;

        for k in (0..self.n).rev()
        {
            if self[[k, k]].abs_diff_eq(&T::zero(), T::default_epsilon())
            {
                // free variable
                // if b.get(k).abs_diff_eq(&T::zero(), T::default_epsilon())
                // {
                //     *b.get_mut(k) = T::one();
                // }
                // else
                // {
                //     return Err(());
                // }
            }
            else
            {
                for l in (k + 1..self.n).rev()
                {
                    b[k] = b[k] - self[[k, l]] * b[l];
                }
                b[k] /= self[[k, k]];
            }
        }

        Ok(b)
    }
}

impl<T> Substitute<Matrix<T>> for Matrix<T> where T: Field + Scalar + AbsDiffEq
{
    fn substitute_forward(&self, a: Matrix<T>) -> Result<Matrix<T>, ()>
    {
        let mut b: Matrix<T> = a;
        let min: usize = std::cmp::min( self.m, self.n);
        for k in 0..min
        {
            if self[[k, k]].abs_diff_eq(&T::zero(), T::default_epsilon())
            {
                //free variable
                // if b.get(k).abs_diff_eq(&T::zero(), T::default_epsilon())
                // {
                //     *b.get_mut(k) = T::one();
                // }
                // else
                // {
                //     return Err(());
                // }
            }
            else
            {
                for l in 0..k
                {
                    b.set_row( & (b.get_row(k) - (b.get_row(l) * self[[k, l]])), k);
                }
                b.set_row( & (b.get_row(k) / self[[k, k]]), k);
            }

        }

        Ok(b)
    }

    fn substitute_backward(&self, a: Matrix<T>) -> Result<Matrix<T>, ()>
    {
        let mut b: Matrix<T> = a;
        let min = std::cmp::min(self.m, self.n);

        for k in (0..min).rev()
        {

            if self[[k, k]].abs_diff_eq(&T::zero(), T::default_epsilon())
            {
                //free variable
                // if b.get(k).abs_diff_eq(&T::zero(), T::default_epsilon())
                // {
                //     for m in 0..min {
                //         *b.get_mut(k, m) = T::one();
                //     }
                // }
                // else
                // {
                //     return Err(())
                // }
            }
            else
            {
                for l in (k + 1..min).rev()
                {
                    b.set_row(&(b.get_row(k) - (b.get_row(l) * self[[k, l]])), k);
                }

                b.set_row(&(b.get_row(k) / self[[k, k]]), k);
            }
        }

        Ok(b)
    }
}