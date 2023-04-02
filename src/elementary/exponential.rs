use std::{f32, f64};
use crate::algebra::abstr::{Complex, Real, Sign};

/// Exponential function and its inverse
///
///<https://en.wikipedia.org/wiki/Exponential_function>
pub trait Exponential
{
    /// Euler's number
    fn e() -> Self;

    ///Exponential function
    fn exp(self) -> Self;

    /// Natural logarithm function
    fn ln(self) -> Self;
}

macro_rules! exponential_impl {
    ($t:ty, $e: expr) => {
        impl Exponential for $t
        {
            ///
            fn e() -> Self
            {
                $e
            }

            ///Exponential function
            fn exp(self) -> Self
            {
                self.exp()
            }

            ///Logarithm function
            fn ln(self) -> Self
            {
                self.ln()
            }
        }
    };
}

exponential_impl!(f32, f32::consts::E);
exponential_impl!(f64, f64::consts::E);

impl<T> Exponential for Complex<T>
    where T: Real
{
    /// Returns the euler number represented as a complex number
    fn e() -> Self
    {
        Complex { re: T::e(), im: T::zero() }
    }

    ///Exponential function
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::{elementary::Exponential};
    /// use mathru::algebra::abstr::Complex;
    ///
    /// let z: Complex<f64> = Complex::new(1.0, 2.0);
    /// let a: Complex<f64> = z.exp();
    /// ```
    fn exp(self) -> Self
    {
        let k: T = self.re.exp();
        Complex { re: k * self.im.cos(), im: k * self.im.sin() }
    }

    ///Logiarithm function
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::{elementary::Exponential};
    /// use mathru::algebra::abstr::Complex;
    ///
    /// let a: Complex<f64> = Complex::new(1.0_f64, 2.0_f64);
    /// let refer: Complex<f64> = Complex::new(5.0_f64.powf(0.5_f64).ln(), 2.0_f64.atan());
    ///
    /// assert_eq!(refer, a.ln());
    /// ```
    fn ln(self) -> Self
    {
        Complex { re: self.abs().re.ln(), im: self.arg().re }
    }
}