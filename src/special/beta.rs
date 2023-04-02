//! Provides the [beta](https://en.wikipedia.org/wiki/Beta_function) and related functions
use crate::{algebra::abstr::Real, special::gamma::Gamma};
use crate::algebra::abstr::cast::ToPrimitive;

/// Provides [beta, also called the Euler integral of the first kind](https://en.wikipedia.org/wiki/Beta_function) related functions
pub trait Beta
{
    /// Beta function
    ///
    /// ```math
    /// \Beta(x,y) = \int_0^1 t^{x-1}(1-t)^{y-1}\,dt
    /// ```
    ///
    /// Fore more information:
    /// <https://en.wikipedia.org/wiki/Beta_function>
    ///
    /// # Arguments
    ///
    /// * `self` > 0.0
    /// * `y` > 0.0
    ///
    /// # Panics
    ///
    /// Panics if the parameter conditions are not fulfilled.
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::special::beta::Beta;
    ///
    /// let x: f64 = 0.3_f64;
    /// let y: f64 = 0.6_f64;
    ///
    /// let beta: f64 = x.beta(y);
    /// ```
    fn beta(self, y: Self) -> Self;

    /// Incomplete beta function
    ///
    /// ```math
    /// \Beta(x;\,a,b) = \int_0^x t^{a-1}\,(1-t)^{b-1}\,dt
    /// ```
    ///
    /// Fore more information:
    /// <https://en.wikipedia.org/wiki/Beta_function>
    ///
    /// # Arguments
    ///
    /// * `self`
    /// * `a` > 0.0
    /// * `b` > 0.0
    ///
    /// # Panics
    ///
    /// Panics if the parameter conditions are not fulfilled.
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::special::beta::Beta;
    ///
    /// let x: f64 = 0.3_f64;
    /// let a: f64 = 0.6_f64;
    /// let b: f64 = 0.7_f64;
    ///
    /// let beta: f64 = x.beta_inc(a, b);
    /// ```
    fn beta_inc(self, a: Self, b: Self) -> Self;

    /// Incomplete regularized beta function
    ///
    /// ```math
    /// I_x(a,b) = \frac{\Beta(x;\,a,b)}{\Beta(a,b)}
    /// ```
    ///
    /// Fore more information:
    /// <https://en.wikipedia.org/wiki/Beta_function#Incomplete_beta_function>
    ///
    /// # Arguments
    ///
    /// * 0.0 < `self` < 1.0,
    /// * `a` > 0.0
    /// * `b` > 0.0
    ///
    /// # Panics
    ///
    /// Panics if the parameter conditions are not fulfilled.
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::special::beta::Beta;
    ///
    /// let a: f64 = 0.3_f64;
    /// let b: f64 = 0.6_f64;
    /// let x: f64 = 0.2_f64;
    ///
    /// let beta: f64 = x.beta_inc_reg(a, b);
    /// ```
    fn beta_inc_reg(self, a: Self, b: Self) -> Self;
}

macro_rules! impl_beta
{
    ($t:ident) =>
    {
        impl Beta for $t
        {
            fn beta(self, y: Self) -> Self
            {
                self.gamma() * y.gamma() / (self + y).gamma()
            }

            fn beta_inc(self, a: Self, b: Self) -> Self
            {
                a.beta(b) * self.beta_inc_reg(a, b)
            }

            /// The code from the following C code was ported to Rust
            /// <http://people.sc.fsu.edu/~jburkardt/c_src/asa109/asa109.c>
            fn beta_inc_reg(self, a: Self, b: Self) -> Self
            {
                let acu: Self = 0.1E-14;

                /*
                Check the input arguments.
                */
                if a <= 0.0 || b <= 0.0
                {
                    panic!();
                }

                if !(0.0..=1.0).contains(&self)
                {
                    panic!();
                }

                /*
                Special cases.
                */
                if self == 0.0 || self == 1.0
                {
                    return self;
                }

                /*
                Change tail if necessary and determine S.
                */
                let mut psq: Self = a + b;
                let mut cx: Self = 1.0 - self;
                let xx: Self;
                let pp: Self;
                let qq: Self;
                let indx: u32;

                if a < psq * self
                {
                    xx = cx;
                    cx = self;
                    pp = b;
                    qq = a;
                    indx = 1;
                }
                else
                {
                    xx = self;
                    pp = a;
                    qq = b;
                    indx = 0;
                }

                let mut term: Self = 1.0;
                let mut ai: Self = 1.0;
                let mut value: Self = 1.0;

                let mut ns: i32 = (qq + cx * psq).to_i32();

                /*
                  Use the Soper reduction formula.
                */
                let mut rx: Self = xx / cx;
                let mut temp: Self = qq - ai;
                if ns == 0
                {
                    rx = xx;
                }

                loop
                {
                    term = term * temp * rx / (pp + ai);
                    value += term;
                    temp = term.abs();

                    if temp <= acu && temp <= acu * value
                    {
                        value = value * (pp * xx.ln() + (qq - 1.0) * cx.ln() - (a.beta(b)).ln()).exp() / pp;

                        if indx != 0
                        {
                            value = 1.0 - value;
                        }
                        break;
                    }

                    ai += 1.0;
                    ns -= 1;

                    if 0 <= ns
                    {
                        temp = qq - ai;
                        if ns == 0
                        {
                            rx = xx;
                        }
                    }
                    else
                    {
                        temp = psq;
                        psq += 1.0;
                    }
                }

                value
            }
        }
    };
}

impl_beta!(f32);
impl_beta!(f64);

/// Beta function
///
/// ```math
/// \Beta(x,y) = \int_0^1 t^{x-1}(1-t)^{y-1}\,dt
/// ```
///
/// Fore more information:
/// <a href="https://en.wikipedia.org/wiki/Beta_function">Wikipedia Beta function</a>
///
/// # Arguments
///
/// * `x` > 0.0
/// * `y` > 0.0
///
/// # Panics
///
/// Panics if the parameter conditions are not fulfilled.
///
/// # Example
///
/// ```
/// use mathru::special::beta;
///
/// let x: f64 = 0.3_f64;
/// let y: f64 = 0.6_f64;
///
/// let beta: f64 = beta::beta(x, y);
/// ```
pub fn beta<T>(x: T, y: T) -> T
    where T: Real + Beta
{
    x.beta(y)
}

/// Incomplete beta function
///
/// ```math
/// \Beta(x;\,a,b) = \int_0^x t^{a-1}\,(1-t)^{b-1}\,dt
/// ```
///
/// Fore more information:
/// <a href="https://en.wikipedia.org/wiki/Beta_function">Wikipedia Beta function</a>
///
/// # Arguments
///
/// * `x`
/// * `a` > 0.0
/// * `b` > 0.0
///
/// # Panics
///
/// Panics if the parameter conditions are not fulfilled.
///
/// # Example
///
/// ```
/// use mathru::special::beta;
///
/// let x: f64 = 0.3_f64;
/// let a: f64 = 0.6_f64;
/// let b: f64 = 0.7_f64;
///
/// let beta: f64 = beta::beta_inc(x, a, b);
/// ```
pub fn beta_inc<T>(x: T, a: T, b: T) -> T
    where T: Real + Beta
{
    x.beta_inc(a, b)
}


/// Incomplete regularized beta function
///
/// ```math
/// I_x(a,b) = \frac{\Beta(x;\,a,b)}{\Beta(a,b)}
/// ```
///
/// Fore more information:
/// <a href="https://en.wikipedia.org/wiki/Beta_function#Incomplete_beta_function">Wikipedia Beta function</a>
///
/// # Arguments
///
/// * 0.0 < `x` < 1.0,
/// * `a` > 0.0
/// * `b` > 0.0
///
/// # Panics
///
/// Panics if the parameter conditions are not fulfilled.
///
/// # Example
///
/// ```
/// use mathru::special::beta;
///
/// let a: f64 = 0.3_f64;
/// let b: f64 = 0.6_f64;
/// let x: f64 = 0.2_f64;
///
/// let beta: f64 = beta::beta_inc_reg(x, a, b);
/// ```
pub fn beta_inc_reg<T>(x: T, a: T, b: T) -> T
    where T: Real + Beta
{
    x.beta_inc_reg(a, b)
}
