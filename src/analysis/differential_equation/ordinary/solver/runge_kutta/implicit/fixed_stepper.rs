/// Fixed step size Stepper
use crate::algebra::{abstr::Real, linear::Vector};
use crate::analysis::differential_equation::ordinary::ImplicitODE;
use crate::analysis::differential_equation::ordinary::solver::runge_kutta::ImplicitFixedStepSizeMethod;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug)]
pub struct ImplicitFixedStepper<T>
{
    /// Step size
    step_size: T,
}

impl<T> ImplicitFixedStepper<T>
    where T: Real
{
    /// Creates an instance with the given step size
    ///
    /// # Arguments
    ///
    /// * 'step_size'
    ///
    /// # Panics
    ///
    /// if 'step_size' <= 0.0
    pub fn new(step_size: T) -> ImplicitFixedStepper<T>
    {
        if step_size <= T::zero()
        {
            panic!();
        }
        ImplicitFixedStepper { step_size }
    }

    pub fn solve<F, M>(&self, prob: &F, method: &M) -> Result<(Vec<T>, Vec<Vector<T>>), ()>
        where F: ImplicitODE<T>,
              M: ImplicitFixedStepSizeMethod<T>
    {
        let t_span = prob.time_span();
        let init = prob.init_cond();
        let t_start = t_span.0;
        let t_stop = t_span.1;

        if t_start > t_stop
        {
            panic!()
        }

        let mut x_n: Vector<T> = init;

        let mut t_n: T = t_start;

        let limit = ((t_stop - t_start) / self.step_size).ceil() + T::one();

        let steps: usize = limit.to_u64() as usize;

        let mut t_vec: Vec<T> = Vec::with_capacity(steps);
        let mut res_vec: Vec<Vector<T>> = Vec::with_capacity(steps);

        for _i in 0..steps
        {
            let h: T = self.step_size.min(t_stop - t_n);

            t_vec.push(t_n);
            res_vec.push(x_n.clone());

            x_n = method.do_step(prob, &t_n, &x_n, &h);

            t_n += h;
        }

        Ok((t_vec, res_vec))
    }

    pub fn get_step_size(&self) -> &T
    {
        &self.step_size
    }

    pub fn set_step_size(&mut self, step_size: T)
    {
        self.step_size = step_size;
    }
}
