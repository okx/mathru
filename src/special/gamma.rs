//! Provides [gamma](https://en.wikipedia.org/wiki/Beta_function) related functions

use crate::algebra::abstr::Real;
use crate::elementary::Power;
use std::f64;
use crate::algebra::abstr::cast::FromPrimitive;
use crate::algebra::abstr::cast::ToPrimitive;

/// Provides [gamma](https://en.wikipedia.org/wiki/Beta_function) related functions
pub trait Gamma
{
    /// Gamma function
    ///
    /// ```math
    /// \Gamma(z) = \int_0^\infty t^{z-1} {\mathrm e}^{-t} \mathrm dt
    /// ```
    ///
    /// Fore more information:
    /// <https://en.wikipedia.org/wiki/Gamma_function>
    ///
    /// # Arguments
    ///
    /// * `self` > 0.0
    ///
    /// # Panics
    ///
    /// *`self` <= 0.0
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::special::gamma::Gamma;
    ///
    /// let z: f64 = 0.3_f64;
    /// let gamma: f64 = z.gamma();
    /// ```
    fn gamma(self) -> Self;

    /// Log-gamma function
    ///
    /// ln&Gamma;(z)
    //
    /// Fore more information:
    /// <https://en.wikipedia.org/wiki/Gamma_function#The_log-gamma_function>
    ///
    /// # Arguments
    ///
    /// * `self`
    ///
    /// ```
    /// use mathru::special::gamma::Gamma;
    ///
    /// let x: f64 = 0.3_f64;
    /// let ln_gamma: f64 = x.ln_gamma();
    /// ```
    fn ln_gamma(self) -> Self;

    /// Digamma function
    ///
    /// ```math
    /// \psi(x)=\frac{d}{dx}\ln\big(\Gamma(x)\big)=\frac{\Gamma'(x)}{\Gamma(x)}
    /// ```
    ///
    /// Fore more information:
    /// <https://en.wikipedia.org/wiki/Digamma_function>
    ///
    /// # Arguments
    ///
    /// * `self`
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::special::gamma::Gamma;
    ///
    /// let x: f64 = 0.3_f64;
    /// let digamma: f64 = x.digamma();
    /// ```
    fn digamma(self) -> Self;

    /// Upper incomplete gamma function
    ///
    /// ```math
    /// \Gamma(a,x) = \int_x^{\infty} t^{a-1}\,\mathrm{e}^{-t}\,{\rm d}t
    /// ```
    ///
    /// Fore more information:
    /// <https://en.wikipedia.org/wiki/Incomplete_gamma_function#Upper_incomplete_Gamma_function>
    ///
    /// # Arguments
    ///
    /// * `self`
    /// * `x`
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::special::gamma::Gamma;
    ///
    /// let a: f64 = 0.5_f64;
    /// let x: f64 = 0.3_f64;
    ///
    /// let gamma_u: f64 = a.gamma_u(x);
    /// ```
    fn gamma_u(self, x: Self) -> Self;

    /// Upper incomplete regularized gamma function
    ///
    /// ```math
    /// Q(a,x)=\frac{\Gamma(a,x)}{\Gamma(a)}
    /// ```
    ///
    /// Fore more information:
    /// <https://en.wikipedia.org/wiki/Incomplete_gamma_function#Regularized_Gamma_functions_and_Poisson_random_variables>
    ///
    /// # Arguments
    ///
    /// * `self`
    /// * `x`
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::special::gamma::Gamma;
    ///
    /// let a: f64 = 0.5_f64;
    /// let x: f64 = 0.3_f64;
    /// let gamma_ur: f64 = a.gamma_ur(x);
    /// ```
    fn gamma_ur(self, x: Self) -> Self;

    /// Lower incomplete gamma function
    ///
    /// ```math
    /// \gamma(a,x) = \int_0^x t^{a-1}\,\mathrm{e}^{-t}\,{\rm d}t
    /// ```
    ///
    /// Fore more information:
    /// <https://en.wikipedia.org/wiki/Incomplete_gamma_function#Regularized_Gamma_functions_and_Poisson_random_variables>
    ///
    /// # Arguments
    ///
    /// * `self`
    /// * `x`
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::special::gamma::Gamma;
    ///
    /// let a: f64 = 0.5_f64;
    /// let x: f64 = 0.3_f64;
    /// let gamma_l: f64 = a.gamma_l(x);
    /// ```
    fn gamma_l(self, x: Self) -> Self;

    /// Lower regularized incomplete gamma function
    ///
    /// ```math
    /// P(a,x)=\frac{\gamma(a,x)}{\Gamma(a)}=1-Q(a,x)
    /// ```
    ///
    /// Fore more information:
    /// <https://en.wikipedia.org/wiki/Incomplete_gamma_function#Regularized_Gamma_functions_and_Poisson_random_variables>
    ///
    /// # Arguments
    ///
    /// * `self`
    /// * `x`
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::special::gamma::Gamma;
    ///
    /// let a: f64 = 0.5_f64;
    /// let x: f64 = 0.3_f64;
    /// let gamma_lr: f64 = a.gamma_lr(x);
    /// ```
    fn gamma_lr(self, x: Self) -> Self;

    /// Inverse of the upper incomplete regularized gamma function
    ///
    /// ```math
    /// Q^{-1}(q,x)
    /// ```
    ///
    /// Fore more information:
    /// <https://en.wikipedia.org/wiki/Incomplete_gamma_function#Regularized_Gamma_functions_and_Poisson_random_variables>
    ///
    /// # Arguments
    ///
    /// * `q`
    /// * `x`
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::special::gamma;
    ///
    /// let a: f64 = 0.5_f64;
    /// let x: f64 = 0.3_f64;
    /// let q = gamma::gamma_ur(a, x);
    /// let x_s: f64 = gamma::gamma_ur_inv(a, q);
    /// ```
    fn gamma_ur_inv(self, p: Self) -> Self;

    /// Inverse of the lower incomplete regularized gamma function
    ///
    /// ```math
    /// P^{-1}(a,p)
    /// ```
    ///
    /// Fore more information:
    /// <https://en.wikipedia.org/wiki/Incomplete_gamma_function#Regularized_Gamma_functions_and_Poisson_random_variables>
    ///
    /// # Arguments
    ///
    /// * `a`
    /// * `x`
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::special::gamma;
    ///
    /// let a: f64 = 0.5_f64;
    /// let x: f64 = 0.3_f64;
    /// let p = gamma::gamma_lr(a, x);
    /// let x_s: f64 = gamma::gamma_lr_inv(a, p);
    /// ```
    fn gamma_lr_inv(self, p: Self) -> Self;
}

macro_rules! impl_gamma
{
    ($t: ty, $PI: expr, $E: expr) =>
    {
        impl Gamma for $t
        {
            fn gamma(self) -> Self
            {
                if self == 0.0
                {
                panic!("The gamma function is undefined for z == 0.0")
                }

                if self < 0.5
                {
                    return $PI / (($PI * self).sin() * (1.0 - self).gamma()); //reflection formula
                }

                let t: $t = self + 6.5;
                let x: $t = 0.999_999_999_999_809_9 + 676.5203681218851 / self
                    - 1259.1392167224028 / (self + 1.0)
                    + 771.323_428_777_653_1 / (self + 2.0)
                    - 176.615_029_162_140_6 / (self + 3.0)
                    + 12.507343278686905 / (self + 4.0)
                    - 0.13857109526572012 / (self + 5.0)
                    + 9.984_369_578_019_572e-6 / (self + 6.0)
                    + 1.5056327351493116e-7 / (self + 7.0);

                return 2.0.sqrt() * $PI.sqrt()
                    * t.pow((self - 0.5))
                    * (-t).exp()
                    * x;
            }

            fn ln_gamma(self) -> Self
            {
                // Auxiliary variable when evaluating the `gamma_ln` function
                let gamma_r: Self = 10.900511;

                // Polynomial coefficients for approximating the `gamma_ln` function
                let gamma_dk: &[$t] = &[2.485_740_891_387_535_5e-5,
                           1.051_423_785_817_219_7,
                           -3.456_870_972_220_1625,
                           4.512_277_094_668_948,
                           -2.982_852_253_235_766_4,
                           1.056_397_115_771_267,
                           -1.954_287_731_916_458_7e-1,
                           1.709_705_434_044_412e-2,
                           -5.719_261_174_043_057e-4,
                           4.633_994_733_599_057e-6,
                           -2.719_949_084_886_077_2e-9];

                let x: Self = self;

                if x < 0.5
                {
                    let s = gamma_dk.iter()
                        .enumerate()
                        .skip(1)
                        .fold(gamma_dk[0], |s, t| s + *t.1 / ((t.0 as u64) as $t - x));

                    $PI.ln()
                    - ($PI * x).sin().ln()
                    - s.ln()
                    - (2.0 * ($E / $PI).pow(0.5)).ln()
                    - (0.5 - x) * ((0.5 - x + gamma_r) / $E).ln()
                }
                else
                {
                    let s = gamma_dk.iter()
                        .enumerate()
                        .skip(1)
                        .fold(gamma_dk[0], |s, t| {
                            s + *t.1 / (x + (t.0 as u64) as $t - 1.0)
                        });

                    s.ln()
                    + (2.0 * ($E / $PI).pow(0.5)).ln()
                    + (x - 0.5) * ((x - 0.5 + gamma_r) / $E).ln()
                }
            }

            //
            fn digamma(self) -> Self
            {
                let c: Self = 8.5;
                let mut value: Self = 0.0;
                let mut x2: Self = self;
                //The comparison only compares the real part ot the number
                while x2 < c
                {
                    value -= 1.0 / x2;
                    x2 += 1.0;
                }
                /*
                  Use Stirling's (actually de Moivre's) expansion
                */
                let mut r: Self = 1.0 / x2;
                value = value + x2.ln() - 0.5 * r;

                r = r * r;

                value -= r
                          * (1.0 / 12.0
                             - r
                               * (1.0 / 120.0
                                  - r
                                    * (1.0 / 252.0
                                       - r
                                         * (1.0 / 240.0
                                            - r
                                              * (1.0 / 132.0
                                                 - r
                                                   * (691.0 / 32760.0
                                                      - r * (1.0 / 12.0)))))));

                return value;
            }

            fn gamma_u(self, x: Self) -> Self
            {
                return self.gamma_ur(x) * self.gamma();
            }

            fn gamma_ur(self, x: Self) -> Self
            {
                return 1.0 - self.gamma_lr(x);
            }

            fn gamma_l(self, x: Self) -> Self
            {
                return self.gamma_lr(x)  * self.gamma();
            }

            fn gamma_lr(self, x: Self) -> Self
            {

                if x <= 0.0
                {
                    panic!("Lower regularized gamma function is not defined for `x` <= 0.0");
                }

                let eps: Self = 0.000000000000001;
                let big: Self = 4503599627370496.0;
                let big_inv: Self = 2.220_446_049_250_313e-16;

                if self == 0.0
                {
                    return 1.0;
                }

                if x == 0.0
                {
                    return 0.0;
                }

                let ax: Self = self * x.ln() - x - self.ln_gamma();

                if ax < -709.782_712_893_384
                {
                    if self < x
                    {
                        return 1.0;
                    }
                    return 0.0;
                }

                if x <= 1.0 || x <= self
                {
                    let mut r2: Self = self;
                    let mut c2: Self = 1.0;
                    let mut ans2: Self = 1.0;
                    loop
                    {
                        r2 += 1.0;
                        c2 *= x / r2;
                        ans2 += c2;

                        if c2 / ans2 <= eps
                        {
                            break;
                        }
                    }
                    return ax.exp() * ans2 / self;
                }

                let mut y: Self = 1.0 - self;
                let mut z: Self = x + y + 1.0;
                let mut c: Self = 0.0;

                let mut p3: Self = 1.0;
                let mut q3: Self = x;
                let mut p2: Self = x + 1.0;
                let mut q2: Self = z * x;
                let mut ans: Self = p2 / q2;

                loop
                {
                    y += 1.0;
                    z += 2.0;
                    c += 1.0;
                    let yc: Self = y * c;

                    let p = p2 * z - p3 * yc;
                    let q = q2 * z - q3 * yc;

                    p3 = p2;
                    p2 = p;
                    q3 = q2;
                    q2 = q;

                    if p.abs() > big
                    {
                        p3 *= big_inv;
                        p2 *= big_inv;
                        q3 *= big_inv;
                        q2 *= big_inv;
                    }

                    if q != 0.0
                    {
                        let nextans = p / q;
                        let error = ((ans - nextans) / nextans).abs();
                        ans = nextans;

                        if error <= eps
                        {
                            break;
                        }
                    }
                }

                1.0 - ax.exp() * ans
            }

            /// Computation of the Incomplete Gamma Function Ratios and their Inverse
            /// ARMIDO R. DIDONATO and ALFRED H. MORRIS, JR. U.S. Naval Surface Weapons Center
            /// ACM Transactions on Mathematical Software, Vol. 12, No. 4, December 1986
            fn gamma_ur_inv(self, q: Self) -> Self
            {
                let a: Self = self;
                let c: Self = 0.577_215_664_901_532_9;
                let eps: Self = 1.0e-10;
                let tau: Self = 1.0e-5;

                if a == 1.0
                {
                    return - q.ln();
                }

                let b: Self = q * gamma(a);
                let p: Self = 1.0 - q;
                let x_0: Self;

                if 0.6 < b || ( 0.45 <= b && a >= 0.3)
                {
                    let u: Self = if b * q > 1.0e-8
                    {
                        (p * gamma(a + 1.0)).pow(1.0 / a)
                    }
                    else
                    {
                        (-q / a - c).exp()
                    };

                    x_0 = u / (1.0 - u / (a + 1.0))
                }
                else
                {
                    if a < 0.3 && (0.35..=0.6).contains(&b)
                    {
                        let t: Self = (-c - b).exp();
                        let u: Self = t * t.exp();
                        x_0 = t * u.exp()
                    }
                    else
                    {
                        if (0.15..0.35).contains(&b) || (0.35 <= a && (0.15..0.45).contains(&b))
                        {
                            let y: Self = -b.ln();
                            let v: Self = y - (1.0 - a) * y.ln();
                            x_0 = y - (1.0 - a) * v.ln() - (1.0 + (1.0 - a) / (1.0 + v)).ln();
                        }
                        else
                        {
                            if 0.01 < b && b < 0.15
                            {
                                let y: Self = -b.ln();
                                let v: Self = y - (1.0 - a) * y.ln();
                                x_0 = y - (1.0 - a) * v.ln() - ((v * v + 2.0 * (3.0 - a) * v + (2.0 - a) * (3.0 - a)) / (v * v + (5.0 - a) * v + 2.0)).ln()
                            }
                            else
                            {
                                let y: Self = -b.ln();
                                let c_1: Self = (a - 1.0) * y.ln();

                                let c_2: Self = (a - 1.0) * (1.0 + c_1);

                                let c_1_2: Self = c_1 * c_1;
                                let c_3: Self = (a - 1.0) * (-c_1_2 / 2.0 + (a - 2.0) * c_1 + (3.0 * a - 5.0) / 2.0);

                                let a_2: Self= a * a;

                                let c_1_3: Self = c_1_2 * c_1;
                                let c_4: Self = (a - 1.0) * (c_1_3 / 3.0 - (3.0 * a - 5.0) / 2.0 * c_1_2 + (a_2 + -6.0 * a + 7.0) * c_1 + (11.0 * a_2 - 46.0 * a + 47.0) / 6.0);

                                let c_1_4: Self = c_1_3 * c_1;

                                let a_3: Self = a_2 * a;

                                let c_5: Self = (a - 1.0) * (-c_1_4 / 4.0 + (11.0 * a - 17.0) / 6.0 * c_1_3 + (-3.0 * a_2 + 13.0 * a - 13.0) * c_1_2 + (2.0 * a_3 - 25.0 * a_2 + 72.0 * a - 61.0) / 2.0 * c_1 + (25.0 * a_3 - 195.0 * a_2 + 477.0 * a - 379.0) / 12.0);

                                let y_2: Self = y * y;
                                let y_3: Self = y_2 * y;
                                let y_4: Self = y_3 * y;

                                x_0 = y + c_1 + c_2 / y + c_3 / y_2 + c_4 / y_3 + c_5 / y_4;
                            }
                        }
                    }
                }

                let mut x_n: Self = x_0;

                for _i in 0..20
                {
                    let r_x: Self = Self::from_f64(r(a.to_f64(), x_n.to_f64()));
                    let w_n: Self = (a - 1.0 - x_n) / 2.0;

                    let t_n: Self = if p <= 0.5
                    {
                        (gamma_lr(a, x_n) - p) / r_x
                    }
                    else
                    {
                        (q - gamma_ur(a, x_n)) / r_x
                    };

                    let x_n_1: Self = if t_n.abs() <= 0.1 && (w_n * t_n).abs() <= 0.1
                    {
                        // Schröder method
                        let k: Self =  w_n * t_n * t_n;
                        let h_n: Self = t_n + k;
                        let x_n_1: Self = x_n * (1.0 - h_n);

                        if w_n.abs() >= 1.0 && k.abs() <= eps
                        {
                            return x_n_1;
                        }

                        x_n_1
                    }
                    else
                    {
                        // Newton-Raphson
                        let h_n: Self = t_n;
                        let x_n_1: Self = x_n * (1.0 - h_n);

                        if h_n.abs() < eps
                        {
                            return x_n_1;
                        }

                        if h_n.abs() <= tau
                        {
                            if p <= 0.5
                            {
                                if (gamma_lr(a, x_n) - p).abs() < tau * p
                                {
                                    return x_n_1;
                                }
                            }
                            else
                            {
                                if (q - gamma_ur(a, x_n)).abs() <= tau * q
                                {
                                    return x_n_1;
                                }
                            }
                        }

                        x_n_1
                    };

                    x_n = x_n_1;

                }

                return x_n;

                fn r(a: f64, x: f64) -> f64
                {
                    if a <= 0.0
                    {
                        panic!();
                    }
                    if x < 0.0
                    {
                        panic!();
                    }

                    x.powf(a) * (-x).exp() / gamma(a)
                }
            }

            fn gamma_lr_inv(self, q: Self) -> Self
            {
                1.0 - self.gamma_ur_inv(q)
            }
        }
    };
}

impl_gamma!(f64, std::f64::consts::PI, std::f64::consts::E);
impl_gamma!(f32, std::f32::consts::PI, std::f32::consts::E);


/// Gamma function
///
/// ```math
/// \Gamma(z) = \int_0^\infty t^{z-1} {\mathrm e}^{-t} \mathrm dt
/// ```
///
/// Fore more information:
/// <https://en.wikipedia.org/wiki/Gamma_function>
///
/// # Arguments
///
/// * `z`
///
/// # Panics
///
/// *`z` == 0.0
///
/// # Example
///
/// ```
/// use mathru::special::gamma;
///
/// let z: f64 = 0.3_f64;
/// let gamma: f64 = gamma::gamma(z);
/// ```
/// The following approximation is implemented
/// <https://en.wikipedia.org/wiki/Lanczos_approximation>
pub fn gamma<T>(z: T) -> T
    where T: Real + Gamma
{
    z.gamma()
}

/// Log-gamma function
///
/// ln&Gamma;(z)
///
/// Fore more information:
/// <https://en.wikipedia.org/wiki/Gamma_function#The_log-gamma_function>
///
/// # Arguments
///
/// * `z`
///
/// # Example
///
/// ```
/// use mathru::special::gamma;
///
/// let x: f64 = 0.3_f64;
/// let ln_gamma: f64 = gamma::ln_gamma(x);
/// ```
pub fn ln_gamma<T>(x: T) -> T
    where T: Gamma
{
    x.gamma()
}

/// Digamma function
///
/// ```math
/// \psi(x)=\frac{d}{dx}\ln\big(\Gamma(x)\big)=\frac{\Gamma'(x)}{\Gamma(x)}
/// ```
///
/// Fore more information:
/// <https://en.wikipedia.org/wiki/Digamma_function>
///
/// # Arguments
///
/// * `x`
///
/// # Example
///
/// ```
/// use mathru::special::gamma;
///
/// let x: f64 = 0.3_f64;
/// let digamma: f64 = gamma::digamma(x);
/// ```
pub fn digamma<T>(x: T) -> T
    where T: Gamma
{
    x.digamma()
}

/// Upper incomplete gamma function
///
/// ```math
/// \Gamma(a,x) = \int_x^{\infty} t^{a-1}\,\mathrm{e}^{-t}\,{\rm d}t
/// ```
///
/// Fore more information:
/// <https://en.wikipedia.org/wiki/Incomplete_gamma_function#Upper_incomplete_Gamma_function>
///
/// # Arguments
///
/// * `a`
/// * `x`
///
/// # Example
///
/// ```
/// use mathru::special::gamma;
///
/// let a: f64 = 0.5_f64;
/// let x: f64 = 0.3_f64;
///
/// let gamma_u: f64 = gamma::gamma_u(a, x);
/// ```
pub fn gamma_u<T>(a: T, x: T) -> T
    where T: Real + Gamma
{
    a.gamma_u(x)
}

/// Upper incomplete regularized gamma function
///
/// ```math
/// Q(a,x)=\frac{\Gamma(a,x)}{\Gamma(a)}
/// ```
///
/// Fore more information:
/// <https://en.wikipedia.org/wiki/Incomplete_gamma_function#Regularized_Gamma_functions_and_Poisson_random_variables>
///
/// # Arguments
///
/// * `a`
/// * `x`
///
/// # Example
///
/// ```
/// use mathru::special::gamma;
///
/// let a: f64 = 0.5_f64;
/// let x: f64 = 0.3_f64;
/// let gamma_u: f64 = gamma::gamma_ur(a, x);
/// ```
pub fn gamma_ur<T>(a: T, x: T) -> T
    where T: Real + Gamma
{
    a.gamma_ur(x)
}

/// Inverse of the upper incomplete regularized gamma function
///
/// ```math
/// Q^{-1}(a,q)
/// ```
///
/// Fore more information:
/// <https://en.wikipedia.org/wiki/Incomplete_gamma_function#Regularized_Gamma_functions_and_Poisson_random_variables>
///
/// # Arguments
///
/// * `a`
/// * `q`
///
/// # Example
///
/// ```
/// use mathru::special::gamma;
///
/// let a: f64 = 0.5_f64;
/// let x: f64 = 0.3_f64;
/// let q = gamma::gamma_ur(a, x);
/// let x_s: f64 = gamma::gamma_ur_inv(a, q);
/// ```
pub fn gamma_ur_inv<T>(a: T, q: T) -> T
    where T: Real + Gamma
{
    a.gamma_ur_inv(q)
}

/// Lower incomplete gamma function
///
/// ```math
/// \gamma(a,x) = \int_0^x t^{a-1}\,\mathrm{e}^{-t}\,{\rm d}t
/// ```
///
/// Fore more information:
/// <https://en.wikipedia.org/wiki/Incomplete_gamma_function#Regularized_Gamma_functions_and_Poisson_random_variables>
///
/// # Arguments
///
/// * `a`
/// * `x`
///
/// # Example
///
/// ```
/// use mathru::special::gamma;
///
/// let a: f64 = 0.5_f64;
/// let x: f64 = 0.3_f64;
/// let gamma_l: f64 = gamma::gamma_l(a, x);
/// ```
pub fn gamma_l<T>(a: T, x: T) -> T
    where T: Real + Gamma
{
    a.gamma_l(x)
}

/// Lower regularized incomplete gamma function
///
/// ```math
/// P(a,x)=\frac{\gamma(a,x)}{\Gamma(a)}=1-Q(a,x)
/// ```
///
/// Fore more information:
/// <https://en.wikipedia.org/wiki/Incomplete_gamma_function#Regularized_Gamma_functions_and_Poisson_random_variables>
///
/// <https://people.sc.fsu.edu/~jburkardt/c_src/asa239/asa239.c>
/// # Arguments
///
/// * `a`
/// * `x`
///
/// # Example
///
/// ```
/// use mathru::special::gamma;
///
/// let a: f64 = 0.5_f64;
/// let x: f64 = 0.3_f64;
/// let gamma_lr: f64 = gamma::gamma_lr(a, x);
/// ```
pub fn gamma_lr<T>(a: T, x: T) -> T
    where T: Real + Gamma
{
    a.gamma_lr(x)
}



/// Inverse of the lower incomplete regularized gamma function
///
/// ```math
/// P^{-1}(a,p)
/// ```
///
/// Fore more information:
/// <https://en.wikipedia.org/wiki/Incomplete_gamma_function#Regularized_Gamma_functions_and_Poisson_random_variables>
///
/// # Arguments
///
/// * `a`
/// * `p`
///
/// # Example
///
/// ```
/// use mathru::special::gamma;
///
/// let a: f64 = 0.5_f64;
/// let x: f64 = 0.3_f64;
/// let p = gamma::gamma_lr(a, x);
/// let x_s: f64 = gamma::gamma_lr_inv(a, p);
/// ```
pub fn gamma_lr_inv<T>(a: T, p: T) -> T
    where T: Real + Gamma
{
    a.gamma_lr_inv(p)
}
