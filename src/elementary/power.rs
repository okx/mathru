use crate::algebra::abstr::{Complex, Real };
use crate::algebra::abstr::Sign;

/// Power functions
///
///<https://en.wikipedia.org/wiki/Exponentiation#Power_functions>
pub trait Power
{
    fn pow(self, exp: Self) -> Self;

    fn root(self, root: Self) -> Self;

    fn sqrt(self) -> Self;
}

macro_rules! power_impl {
    ($t:ty) => {
        impl Power for $t
        {
            fn pow(self, exp: Self) -> Self
            {
                self.powf(exp)
            }

            fn root(self, root: Self) -> Self
            {
                self.powf(1.0 / root)
            }

            fn sqrt(self) -> Self
            {
                self.powf(0.5)
            }
        }
    };
}

power_impl!(f32);
power_impl!(f64);

macro_rules! power_impl_integer {
    ($t:ty) => {
        impl Power for $t
        {
            fn pow(self, _exp: Self) -> Self
            {
                unimplemented!();
            }

            fn root(self, _root: Self) -> Self
            {
                unimplemented!();
            }

            fn sqrt(self) -> Self
            {
                unimplemented!();
            }
        }
    };
}

power_impl_integer!(u8);
power_impl_integer!(u16);
power_impl_integer!(u32);
power_impl_integer!(u64);
power_impl_integer!(u128);
power_impl_integer!(usize);
power_impl_integer!(i8);
power_impl_integer!(i16);
power_impl_integer!(i32);
power_impl_integer!(i64);
power_impl_integer!(i128);
power_impl_integer!(isize);

impl<T> Power for Complex<T>
    where T: Real
{
    /// Power
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::{elementary::Power};
    /// use mathru::algebra::abstr::Complex;
    ///
    /// let a: Complex<f64> = Complex::new(1.0_f64, 2.0_f64);
    /// let b: Complex<f64> = Complex::new(-2.0_f64, -1.0_f64);
    /// let refer: Complex<f64> = Complex::new(-0.6006033457684014, -0.07399065302898929);
    ///
    /// assert_eq!(refer, a.pow(b));
    /// ```
    fn pow(self, exp: Self) -> Self
    {
        let r: T = self.abs().re;
        let phi: T = self.arg().re;
        let k: T = r.pow(exp.re) * (-exp.im * phi).exp();
        let theta: T = r.ln() * exp.im + exp.re * phi;
        let re: T = k * theta.cos();
        let im: T = k * theta.sin();

        Complex::new(re, im)
    }

    fn root(self, _root: Self) -> Self
    {
        unimplemented!();
    }

    fn sqrt(self) -> Self
    {
        let arg = self.arg().re * T::from_f64(0.5);
        let abs = self.abs().re.sqrt();

        Complex::new(abs * (arg.cos()), abs * (arg.sin()))
    }
}