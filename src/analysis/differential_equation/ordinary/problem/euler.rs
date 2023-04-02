use crate::{
    algebra::{
        abstr::Real,
        linear::{Matrix, Vector},
    },
    analysis::differential_equation::ordinary::{ExplicitODE, ImplicitODE},
};

///
/// ```math
/// y_{1}^{'}(x) = (I_{2} - I_{3})/I_{1} * y_{2}(x) * y_{3}(x)
/// y_{2}^{'}(x) = (I_{3} - I_{1})/I_{2} * y_{3}(x) * y_{1}(x)
/// y_{3}^{'}(x) = (I_{1} - I_{2})/I_{3} * y_{1}(x) * y_{2}(x) + f(x)
///
/// f =
/// ```
pub struct Euler<T>
{
    i1: T,
    i2: T,
    i3: T,

    time_span: (T, T),
    init_cond: Vector<T>,
}

impl<T> Default for Euler<T> where T: Real
{
    fn default() -> Euler<T>
    {
        Euler { i1: T::from_f64(0.5),
                i2: T::from_f64(2.0),
                i3: T::from_f64(3.0),
                time_span: (T::from_f64(0.0), T::from_f64(20.0)),
                init_cond: vector![T::from_f64(1.0); T::from_f64(0.0); T::from_f64(0.9)] }
    }
}

impl<T> ExplicitODE<T> for Euler<T> 
    where T: Real
{
    fn func(&self, x: &T, y: &Vector<T>) -> Vector<T>
    {
        let y_1s: T = ((self.i2 - self.i3) / self.i1) * (y[1] * y[2]);
        let y_2s: T = ((self.i3 - self.i1) / self.i2) * (y[2] * y[0]);

        let f: T;
        if *x >= T::from_f64(3.0) * T::pi() && *x <= T::from_f64(4.0) * T::pi()
        {
            f = T::from_f64(0.25) * x.sin() * x.sin();
        }
        else
        {
            f = T::zero();
        }
        let y_3s: T = ((self.i1 - self.i2) / self.i3) * (y[0] * y[1]) + f;

        vector![y_1s; y_2s; y_3s]
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

impl<T> ImplicitODE<T> for Euler<T> 
    where T: Real
{
    fn func(&self, x: &T, y: &Vector<T>) -> Vector<T>
    {
        let a: T = (self.i2 - self.i3) / self.i1;
        let b: T = (self.i3 - self.i1) / self.i2;
        let c: T = (self.i1 - self.i2) / self.i3;

        let y_1s: T = a * (y[1] * y[2]);
        let y_2s: T = b * (y[2] * y[0]);

        let f: T;
        if *x >= T::from_f64(3.0) * T::pi() && *x <= T::from_f64(4.0) * T::pi()
        {
            f = T::from_f64(0.25) * x.sin() * x.sin();
        }
        else
        {
            f = T::zero();
        }
        let y_3s: T = c * (y[0] * y[1]) + f;
        vector![y_1s; y_2s; y_3s]
    }

    fn jacobian(&self, _x: &T, y: &Vector<T>) -> Matrix<T>
    {
        let a: T = (self.i2 - self.i3) / self.i1;
        let b: T = (self.i3 - self.i1) / self.i2;
        let c: T = (self.i1 - self.i2) / self.i3;

        matrix![T::zero(), a * y[2], a * y[1];
                        b * y[2], T::zero(), b * y[0];
                        c * y[1], c * y[0], T::zero()]
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