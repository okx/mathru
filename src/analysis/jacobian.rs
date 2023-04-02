use crate::algebra::{
    abstr::Real,
    linear::{Matrix, Vector},
};

pub trait Jacobian<T>
    where T: Real
{
    fn jacobian(&self, input: &Vector<T>) -> Matrix<T>;
}
