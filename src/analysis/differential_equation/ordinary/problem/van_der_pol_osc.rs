use crate::algebra::abstr::Real;
use crate::algebra::linear::{Vector, Matrix};
use crate::analysis::differential_equation::ordinary::ImplicitODE;

/// Van der Pol oscillator
/// ```math
/// x_{1}^{'}(t) = x_{2}(t) \\
/// x_{2}^{'}(t) = \epsilon((1 - x_{1}(t)^{2})x_{2}(t) - x_{1}(t)) \\
/// ```
///
/// ```math
/// x_{1}(0) = 1 \\
/// x_{2}(0) = 0 \\
/// \epsilon = 0.1 \\
/// ```
pub struct VanDerPolOsc<T>
{
    epsilon: T,

    time_span: (T, T),
    init_cond: Vector<T>,
}

impl<T> Default for VanDerPolOsc<T> 
    where T: Real
{
    fn default() -> VanDerPolOsc<T>
    {
        VanDerPolOsc { epsilon: T::from_f64(0.1),
                       time_span: (T::from_f64(0.0), T::from_f64(30.0)),
                       init_cond: vector![T::from_f64(1.0); T::from_f64(0.0)] }
    }
}
/// Implicit ordinary differential equation
///
/// $0 = f(t, x(t), x^{'}(t), \dots, x^{n}(t))$
impl<T> ImplicitODE<T> for VanDerPolOsc<T> where T: Real
{
    fn func(&self, _t: &T, x: &Vector<T>) -> Vector<T>
    {
        let x_1 = x[0];
        let x_2 = x[1];
        vector![x_2; self.epsilon * (T::one() - (x_1 * x_1)) * x_2 - x_1]
    }

    fn jacobian(&self, _t: &T, x: &Vector<T>) -> Matrix<T>
    {
        let x_1 = x[0];
        let x_2 = x[1];
        matrix![T::zero(), T::one(); -T::from_f64(2.0) * self.epsilon * x_1 * x_2  - T::one(), (T::one() - x_1 *
		x_1) * self.epsilon]
    }

    fn time_span(&self) -> (T, T)
    {
        self.time_span
    }

    fn init_cond(&self) -> Vector<T>
    {
        self.init_cond.clone()
    }
}
