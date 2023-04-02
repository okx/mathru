//! Often used ODEs
use mathru::{algebra::linear::Vector, analysis::differential_equation::ordinary::ExplicitODE};
use std::{default::Default, f64};

/// Define ODE
/// $x^{'}(t) = 2x(t) = f(t, x)$
/// $x(t) = C e^{at}$
/// $x^'(t) = a x(t) $ 
/// $x(t_{s}) = C e^{at_s} => C = \frac{y(t_s)}{e^{at_s}}$
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
                       init_cond: vector![0.5; 2.0] }
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
/// $x^{'}(t) = x^2 + 1 = f(t, x) $
/// $x(t) = tan(c+t)$
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
/// $x^{'}(t)xy = x^2 = f(t, x) $
/// $x(t) = 1/(c-t)$
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
