use crate::algebra::abstr::{Complex, Real};
use crate::elementary::trigonometry::Trigonometry;
use crate::algebra::abstr::One;
use crate::elementary::power::Power;
use crate::elementary::exponential::Exponential;

/// Hyperbolic functions
///
///<https://en.wikipedia.org/wiki/Inverse_hyperbolic_functions>
pub trait Hyperbolic
{
    /// Hyperbolic sine
    fn sinh(self) -> Self;

    /// Hyperbolic cosine
    fn cosh(self) -> Self;

    /// Hyperbolic tangens
    fn tanh(self) -> Self;

    /// Hyperbolic cotangens
    fn coth(self) -> Self;

    /// Hyperbolic secant
    fn sech(self) -> Self;

    /// Hyperbolic cosecant
    fn csch(self) -> Self;

    /// Inverse hyperbolic  sine
    fn arsinh(self) -> Self;

    /// Inverse hyperbolic cosine
    fn arcosh(self) -> Self;

    /// Inverse hyperbolic tangens
    fn artanh(self) -> Self;

    /// Inverse hyperbolic cosecant
    fn arcoth(self) -> Self;

    /// Inverse hyperbolic secant
    fn arsech(self) -> Self;

    /// Inverse hyperbolic cosecant
    fn arcsch(self) -> Self;
}

macro_rules! hyperbolic_impl {
    ($t:ty) => {
        impl Hyperbolic for $t
        {
            /// Hyperbolic sine
            fn sinh(self) -> Self
            {
                self.sinh()
            }

            /// Hyperbolic cosine
            fn cosh(self) -> Self
            {
                self.cosh()
            }

            /// Hyperbolic tangens
            ///
            /// # Arguments
            ///
            /// * `self` :
            ///
            /// # Example
            ///
            /// ```
            /// use mathru::elementary::Hyperbolic;
            ///
            /// let x: f64 = 0.0_f64;
            ///
            /// let f: f64 = x.tanh();
            /// ```
            fn tanh(self) -> Self
            {
                self.tanh()
            }

            /// Hyperbolic cotangens
            ///
            /// # Arguments
            ///
            /// * `self` : != 0.0
            ///
            /// # Panic
            ///
            /// iff self == 0.0
            ///
            /// # Example
            ///
            /// ```
            /// use mathru::elementary::Hyperbolic;
            ///
            /// let x: f64 = 1.0_f64;
            ///
            /// let f: f64 = x.coth();
            /// ```
            fn coth(self) -> Self
            {
                if self == 0.0
                {
                    panic!();
                }

                self.cosh() / self.sinh()
            }

            /// Hyperbolic secant
            ///
            /// # Arguments
            ///
            /// * `self` :
            ///
            /// # Example
            ///
            /// ```
            /// use mathru::elementary::Hyperbolic;
            ///
            /// let x: f64 = 0.0_f64;
            ///
            /// let f: f64 = x.sech();
            /// ```
            fn sech(self) -> Self
            {
                1.0 / self.cosh()
            }

            /// Hyperbolic cosecant
            ///
            /// # Arguments
            ///
            /// * `self` : != 0.0
            ///
            /// # Panics
            ///
            /// if  self == 0
            ///
            /// # Example
            ///
            ///
            /// ```
            /// use mathru::elementary::Hyperbolic;
            ///
            /// let x: f64 = 1.0_f64;
            ///
            /// let f: f64 = x.csch();
            /// ```
            fn csch(self) -> Self
            {
                if self == 0.0
                {
                    panic!();
                }
                1.0 / self.sinh()
            }

            /// Hyperbolic inverse sine
            fn arsinh(self) -> Self
            {
                self.asinh()
            }

            /// Hyperbolic inverse cosine
            fn arcosh(self) -> Self
            {
                self.acosh()
            }

            /// Hyperbolic inverse tangens
            fn artanh(self) -> Self
            {
                if -1.0 >= self || self >= 1.0
                {
                    panic!();
                }

                self.atanh()
            }

            /// Hyperbolic inverse cotan
            ///
            /// # Arguments
            ///
            /// * `self`  -1.0 > self, self > 1.0
            ///
            /// # Panics
            ///
            /// if  -1.0 <= self && self <= 1.0
            ///
            /// # Example
            ///
            /// ```
            /// use mathru::{
            ///     algebra::abstr::Field,
            ///     elementary::{Exponential, Hyperbolic},
            /// };
            ///
            /// let x: f64 = 2.0_f64;
            /// let f: f64 = x.arcoth();
            /// ```
            fn arcoth(self) -> Self
            {
                if (-1.0..=1.0).contains(&self)
                {
                    panic!();
                }

                ((self + 1.0) / (self - 1.0)).ln() / 2.0
            }

            /// Hyperbolic inverse secant
            ///
            /// # Arguments
            ///
            /// * `self`  0.0 < self <= 1.0
            ///
            /// # Panics
            ///
            /// if  0.0 >= self || self > 1.0
            ///
            /// # Example
            ///
            /// ```
            /// use mathru::elementary::{Exponential, Hyperbolic};
            ///
            /// let x: f64 = 0.5_f64;
            /// let f: f64 = x.arsech();
            /// let g: f64 = (1.0 / x).arcosh();
            /// ```
            fn arsech(self) -> Self
            {
                if 0.0 >= self || self > 1.0
                {
                    panic!();
                }

                (1.0 / self).arcosh()
            }

            /// Hyperbolic inverse cosecant
            ///
            /// # Arguments
            ///
            /// * `self`  <> 0.0
            ///
            /// # Panics
            ///
            /// iff self = 0.0
            ///
            /// # Example
            ///
            /// ```
            /// use mathru::{
            ///     algebra::abstr::Field,
            ///     elementary::{Exponential, Hyperbolic},
            /// };
            ///
            /// let x: f64 = 2.0_f64;
            /// let f: f64 = x.arcsch();
            /// ```
            fn arcsch(self) -> Self
            {
                (1.0 / self).arsinh()
            }
        }
    };
}

hyperbolic_impl!(f32);
hyperbolic_impl!(f64);

impl<T> Hyperbolic for Complex<T>
    where T: Real
{
    /// Hyperbolic sine
    fn sinh(self) -> Self
    {
        Complex::new(T::zero(), -T::one()) * Complex::new(-self.im, self.re).sin()
    }

    /// Hyperbolic cosine
    fn cosh(self) -> Self
    {
        Complex::new(-self.im, self.re).cos()
    }

    /// Hyperbolic tangens
    fn tanh(self) -> Self
    {
        self.sinh() / self.cosh()
    }

    /// Hyperbolic cotangens
    fn coth(self) -> Self
    {
        self.cosh() / self.sinh()
    }

    /// Hyperbolic secant
    fn sech(self) -> Self
    {
        Complex::new(-self.im, self.re).sec()
    }

    /// Hyperbolic cosecant
    fn csch(self) -> Self
    {
        Complex::new(T::zero(), -T::one()) * Complex::new(-self.im, self.re).csc()
    }

    /// Hyperbolic inverse sine
    ///
    /// # Arguments
    ///
    /// # Panics
    fn arsinh(self) -> Self
    {
        let p: Complex<T> = Complex::new(T::one() / (T::one() + T::one()), T::zero());
        (self + (self * self + Complex::one()).pow(p)).ln()
    }

    /// Hyperbolic inverse cosine
    ///
    /// # Argument
    ///
    /// # Panics
    fn arcosh(self) -> Self
    {
        let p: Complex<T> = Complex::new(T::one() / (T::one() + T::one()), T::zero());
        (self + (self * self - Complex::one()).pow(p)).ln()
    }

    /// Inverse hyperbolic tangent
    ///
    /// # Arguments
    ///
    /// # Panics
    fn artanh(self) -> Self
    {
        let f: Complex<T> = Complex::new(T::one() / (T::one() + T::one()), T::zero());
        ((Complex::one() + self) / (Complex::one() - self)).ln() * f
    }

    /// Inverse hyperbolic cosecant
    ///
    /// # Arguments
    ///
    ///
    /// # Panics
    fn arcoth(self) -> Self
    {
        let f: Complex<T> = Complex::new(T::one() / (T::one() + T::one()), T::zero());
        ((self + Complex::one()) / (self - Complex::one())).ln() * f
    }

    /// Hyperbolic inverse secant
    fn arsech(self) -> Self
    {
        (Complex::one() / self).arcosh()
    }

    // Hyperbolic inverse cosecant
    fn arcsch(self) -> Self
    {
        (Complex::one() / self).arsinh()
    }
}

