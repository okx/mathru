use crate::algebra::linear::{Vector, Matrix};
use super::Substitute;
use crate::algebra::abstr::{Field, Scalar};

impl<T> Substitute<Vector<T>> for Matrix<T> where T: Field + Scalar
{
    fn substitute_forward(&self, b: Vector<T>) -> Result<Vector<T>, ()>
    {
        let (b_m, b_n): (usize, usize) = b.dim();
        let mut b_data = b.convert_to_vec();
        T::xtrsm('L',
                 'L',
                 'N',
                 'N',
                 b_m as i32,
                 b_n as i32,
                 T::one(),
                 self.data.as_slice(),
                 self.m as i32,
                 b_data.as_mut_slice(),
                 b_m as i32);

        Ok(Vector::new_column(b_data))
    }

    fn substitute_backward(&self, b: Vector<T>) -> Result<Vector<T>, ()>
    {
        let (b_m, b_n): (usize, usize) = b.dim();
        let mut b_data = b.convert_to_vec();
        T::xtrsm('L',
                 'U',
                 'N',
                 'N',
                 b_m as i32,
                 b_n as i32,
                 T::one(),
                 self.data.as_slice(),
                 self.m as i32,
                 b_data.as_mut_slice(),
                 b_m as i32);

        Ok(Vector::new_column(b_data))
    }
}

impl<T> Substitute<Matrix<T>> for Matrix<T> where T: Field + Scalar
{
    fn substitute_forward(&self, b: Matrix<T>) -> Result<Matrix<T>, ()>
    {
        let mut c: Matrix<T> = b;
        T::xtrsm('L',
                 'L',
                 'N',
                 'N',
                 c.m as i32,
                 c.n as i32,
                 T::one(),
                 self.data.as_slice(),
                 self.m as i32,
                 c.data.as_mut_slice(),
                 c.m as i32);

        Ok(c)
    }

    fn substitute_backward(&self, b: Matrix<T>) -> Result<Matrix<T>, ()>
    {
        let mut c: Matrix<T> = b;
        T::xtrsm('L',
                 'U',
                 'N',
                 'N',
                 c.m as i32,
                 c.n as i32,
                 T::one(),
                 self.data.as_slice(),
                 self.m as i32,
                 c.data.as_mut_slice(),
                 c.m as i32);

        Ok(c)
    }
}