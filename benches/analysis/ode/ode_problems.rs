//! Often used ODEs
use mathru::{
    vector, matrix,
    algebra::linear::{Matrix, Vector},
    analysis::differential_equation::ordinary::{ExplicitODE, ImplicitODE},
};
use std::default::Default;

/// Define ODE
/// $y^{'} = ay = f(x, y) $
/// $y = C a e^{at}$
/// $'y(t_{s}) = C a e^{at_s} => C = \frac{y(t_s)}{ae^{at_s}}$
pub struct ExplicitODE1
{
    time_span: (f64, f64),
    init_cond: Vector<f64>,
}

impl Default for ExplicitODE1
{
    fn default() -> ExplicitODE1
    {
        ExplicitODE1 { time_span: (0.0, 2.0),
                       init_cond: vector![0.5] }
    }
}

impl ExplicitODE<f64> for ExplicitODE1
{
    fn func(&self, _t: &f64, x: &Vector<f64>) -> Vector<f64>
    {
        return x * &2.0f64;
    }

    fn time_span(&self) -> (f64, f64)
    {
        return self.time_span;
    }

    fn init_cond(&self) -> Vector<f64>
    {
        return self.init_cond.clone();
    }
}

/// Define ODE
/// $y^{'} = y^2 + 1 = f(x, y) $
/// $y = tan(c+x)$
pub struct ExplicitODE2
{
    time_span: (f64, f64),
    init_cond: Vector<f64>,
}

impl Default for ExplicitODE2
{
    fn default() -> ExplicitODE2
    {
        ExplicitODE2 { time_span: (0.0, 1.4),
                       init_cond: vector![0.0] }
    }
}

impl ExplicitODE<f64> for ExplicitODE2
{
    fn func(&self, _t: &f64, x: &Vector<f64>) -> Vector<f64>
    {
        return x.clone().apply(&|e: &f64| -> f64 {
                            return e * e + 1.0;
                        });
    }

    fn time_span(&self) -> (f64, f64)
    {
        return self.time_span;
    }

    fn init_cond(&self) -> Vector<f64>
    {
        return self.init_cond.clone();
    }
}

/// Define ODE
/// $y^{'} = y^2 = f(x, y) $
/// $y(x) = 1/(c-x)$
pub struct ExplicitODE3
{
    time_span: (f64, f64),
    init_cond: Vector<f64>,
}

impl Default for ExplicitODE3
{
    fn default() -> ExplicitODE3
    {
        ExplicitODE3 { time_span: (0.8, 1.8),
                       init_cond: vector![5.0 / 6.0] }
    }
}

impl ExplicitODE<f64> for ExplicitODE3
{
    fn func(&self, _t: &f64, x: &Vector<f64>) -> Vector<f64>
    {
        return x.clone().apply(&|e: &f64| -> f64 { return e * e });
    }

    fn time_span(&self) -> (f64, f64)
    {
        return self.time_span;
    }

    fn init_cond(&self) -> Vector<f64>
    {
        return self.init_cond.clone();
    }
}

/// Define ODE
/// $y^{'}(t) = -4y(t) + 8 $
/// $y(0) = 1 $
///
/// $ y(t) = 2 - exp(-4t) $
pub struct ImplicitODE1
{
    time_span: (f64, f64),
    init_cond: Vector<f64>,
}

impl Default for ImplicitODE1
{
    fn default() -> ImplicitODE1
    {
        ImplicitODE1 { time_span: (0.0, 2.0),
                       init_cond: vector![1.0] }
    }
}

impl ImplicitODE<f64> for ImplicitODE1
{
    fn func(&self, _t: &f64, x: &Vector<f64>) -> Vector<f64>
    {
        let result = (x * &-4.0) + 8.0;
        return result;
    }

    fn jacobian(&self, _t: &f64, _input: &Vector<f64>) -> Matrix<f64>
    {
        let jacobian = matrix![-4.0];
        return jacobian;
    }

    fn time_span(&self) -> (f64, f64)
    {
        return self.time_span;
    }

    fn init_cond(&self) -> Vector<f64>
    {
        return self.init_cond.clone();
    }
}
