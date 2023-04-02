use crate::analysis::differential_equation::ordinary::ImplicitODE;
use crate::algebra::{abstr::Real, linear::Vector};

pub trait ImplicitFixedStepSizeMethod<T>
    where T: Real
{
    fn do_step<F>(&self, prob: &F, t_n: &T, x_n: &Vector<T>, h: &T) -> Vector<T>
        where F: ImplicitODE<T>;

    fn order(&self) -> u8;
}
