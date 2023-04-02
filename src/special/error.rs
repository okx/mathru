//! Provides the [error](https://en.wikipedia.org/wiki/Error_function) and related functions
use crate::algebra::abstr::Real;
use crate::elementary::Trigonometry;
use crate::special::gamma;
use crate::special::gamma::Gamma;
use crate::algebra::abstr::Sign;

/// Provides [error, also called the Gauss error function](https://en.wikipedia.org/wiki/Error_function) related functions
pub trait Error
{
    /// Error function
    ///
    /// ```math
    /// \operatorname{erf}(z) = \frac{2}{\sqrt\pi}\int_0^z e^{-t^2}\,dt.
    /// ```
    ///
    /// [For more information](https://en.wikipedia.org/wiki/Error_function)
    ///
    /// # Arguments
    ///
    /// self:
    ///
    fn erf(self) -> Self;

    /// Complementary error function
    ///
    /// ```math
    /// \operatorname{erfc}(z) = 1 - \operatorname{erf}(z),
    /// ```
    /// # Arguments
    ///
    /// self
    fn erfc(self) -> Self;

    /// Inverse error function
    ///
    /// ```math
    /// \operatorname{erfinv}(x) = \operatorname{erf}^{-1}(x) \quad \text{for} \quad x \in (-1, 1)
    /// ```
    ///
    /// [For more information](https://en.wikipedia.org/wiki/Error_function#Inverse_functions)
    ///
    /// # Arguments
    ///
    /// -1.0 < x < 1.0
    ///
    /// # Panics
    ///
    /// if x < -1.0 or x > 1.0
    ///
    fn erfinv(self) -> Self;

    /// Inverse complementary error function
    ///
    /// ```math
    /// \operatorname{erfc}^{-1}(1-z) = \operatorname{erf}^{-1}(z)
    /// ```
    /// # Arguments
    ///
    /// 0 <= x <= 2.0
    ///
    /// # Panics
    ///
    /// if x < 0.0 or x > 2.0
    ///
    fn erfcinv(self) -> Self;
}


impl Error for f64
{
    fn erf(self) -> Self
    {
        if self == 0.0
        {
            return self;
        }
        self.sign() * (1.0f64 - gamma::gamma_u(0.5f64, self * self) / Self::pi().sqrt())
    }

    fn erfc(self) -> Self
    {
        1.0 - self.erf()
    }

    /// <https://en.wikipedia.org/wiki/Error_function#Inverse_functions>
    /// Using the rational approximations tabulated in:
    ///J. M. Blair, C. A. Edwards, and J. H. Johnson,
    /// "Rational Chebyshev approximations for the inverse of the error function",
    fn erfinv(self) -> Self
    {
        let factors_leq075_p: [f64; 7] = [1.603_049_558_440_662_3e1,
            -9.078_495_926_296_033e1,
            1.864_491_486_162_098_7e2,
            -1.690_014_273_464_238e2,
            6.545_466_284_794_487_e1,
            -8.642_130_115_872_478,
            0.176_058_782_139_059];

        let factors_leq075_q: [f64; 7] = [1.478_064_707_151_383_1e1,
            -9.137_416_702_426_032e1,
            2.101_579_048_620_532e2,
            -2.221_025_412_185_513_2e2,
            1.076_045_391_605_512_4e2,
            -2.060_107_303_282_654e1, 0.1e1];

        let factors_leg09375_p: [f64; 8] = [-1.523_892_634_407_261_2e-2,
            0.344_455_692_413_612_5,
            -2.934_439_867_254_247_8,
            1.176_350_570_521_782_8e1,
            -2.265_529_282_310_110_3e1,
            1.912_133_439_658_033e1,
            -5.478_927_619_598_319,
            0.23751_66890_24448];

        let factors_leg09375_q: [f64; 8] = [-1.084_651_696_020_599_5e-2,
            0.261_062_888_584_307_9,
            -2.406_831_810_439_376,
            1.069_512_997_338_701_5e1,
            -2.371_671_552_159_658e1,
            2.464_015_894_391_728_5e1,
            -1.001_437_634_978_307e1,
            0.1e1];

        let factors_p: [f64; 9] = [1.050_131_152_373_343_8e-4,
            1.053_261_131_423_333_8e-2,
            0.269_878_027_362_432_8,
            2.326_869_578_891_969,
            7.167_854_794_910_8,
            8.547_561_182_216_782,
            6.873_808_807_354_384,
            3.627_002_483_095_871,
            0.886_062_739_296_515_5];

        let factors_q: [f64; 10] = [1.050_126_668_703_033_8e-4,
            1.053_286_230_093_332_7e-2,
            0.270_198_623_737_515_6,
            2.350_143_639_797_025_2,
            7.607_802_878_580_127,
            1.118_158_610_405_690_8e1,
            1.194_878_791_843_539_6e1,
            8.192_240_974_726_99,
            4.099_387_907_636_801,
            0.1e1];

        let a: f64 = self.abs();
        if a >= 1.0
        {
            if self == 1.0
            {
                return f64::INFINITY;
            }
            else
            {
                if self == -1.0
                {
                    return f64::NEG_INFINITY;
                }
            }
            panic!("|self| has to be <= 1.0")
        }
        else
        {
            if a <= 0.75
            {
                let t: f64 = self * self - 0.5625;

                self * horner(t, &factors_leq075_p) / horner(t, &factors_leq075_q)
            } else {
                if a <= 0.9375
                {
                    let t: f64 = self * self - 0.87890625;

                    self * horner(t, &factors_leg09375_p) / horner(t, &factors_leg09375_q)
                } else {
                    let t: f64 = 1.0 / (-(1.0 - a).ln()).sqrt();

                    horner(t, &factors_p) / (copysign(t, self) * horner(t, &factors_q))
                }
            }
        }
    }

    fn erfcinv(self) -> Self
    {
        if !(0.0f64..=2.0f64).contains(&self)
        {
            panic!("self is > 2.0 or < 0.0");
        }
        (1.0 - self).erfinv()
    }
}

impl Error for f32
{
    fn erf(self) -> Self
    {
        if self == 0.0
        {
            return self;
        }
        self.sign() * (1.0f32 - gamma::gamma_u(0.5f32, self * self) / Self::pi().sqrt())
    }

    fn erfc(self) -> Self
    {
        1.0 - self.erf()
    }

    fn erfinv(self) -> Self
    {
        let factors_leq075_p: [f32; 3] = [-1.309_599_7e1,
            0.267_852_25,
            -9.289_058];

        let factors_leq075_q: [f32; 4] = [-1.207_494_3e1,
            3.096_061_5e1,
            -1.714_997_9e1,
            0.1e1];

        let factors_leg09375_p: [f32; 4] = [-1.240_256_5e-1,
            1.068_805_9,
            -1.959_455_6,
            4.230_581_2e-1];

        let factors_leg09375_q: [f32; 4] = [8.827_698e-2,
            8.900_743e-1,
            -2.175_703,
            0.1e1];

        let factors_p: [f32; 6] = [1.550_47e-1,
            1.382_719_6,
            6.909_693_5e-1,
            -1.128_081_4,
            6.805_442_6e-1,
            -1.644_415_7e-1];

        let factors_q: [f32; 3] = [1.550_248_6e-1, 1.385_228_2, 0.1e1];

        let a: f32 = self.abs();
        if a >= 1.0
        {
            if self == 1.0
            {
                return f32::INFINITY
            } else {
                if self == -1.0
                {
                    return f32::NEG_INFINITY
                }
            }
            panic!("|self| has to be <= 1.0")
        } else {
            if a <= 0.75
            {
                let t: f32 = self * self - 0.5625;

                self * horner(t, &factors_leq075_p) / horner(t, &factors_leq075_q)
            } else {
                if a <= 0.9375
                {
                    let t: f32 = self * self - 0.87890625;

                    self * horner(t, &factors_leg09375_p) / horner(t, &factors_leg09375_q)
                } else {
                    let t: f32 = 1.0 / (-(1.0 - a).ln()).sqrt();

                    horner(t, &factors_p) / (copysign(t, self) * horner(t, &factors_q))
                }
            }
        }
    }

    fn erfcinv(self) -> Self
    {
        if !(0.0f32..=2.0f32).contains(&self)
        {
            panic!("self is > 2.0 or < 0.0");
        }
        (1.0 - self).erfinv()
    }
}

/// Error Function
///
/// ```math
/// \operatorname{erf}(z) = \frac{2}{\sqrt\pi}\int_0^z e^{-t^2}\,dt.
/// ```
///
/// [For more information](https://en.wikipedia.org/wiki/Error_function)
///
/// # Example
///
/// ```
/// use mathru::special::error;
///
/// let x: f64 = 0.0;
/// let error: f64 = error::erf(x);
/// ```
pub fn erf<T>(x: T) -> T
    where T: Real + Error + Gamma
{
    x.erf()
}

/// Complementary error function
///
/// ```math
/// \operatorname{erfc}(z) = 1 - \operatorname{erf} z,
/// ```
/// # Arguments
///
/// x
///
/// # Example
///
/// ```
/// use mathru::special::error;
///
/// let x: f64 = 0.0;
/// let error: f64 = error::erfc(x);
/// ```
pub fn erfc<T>(x: T) -> T
    where T: Real + Error + Gamma
{
    x.erfc()
}

/// Inverse error function
///
/// ```math
/// \operatorname{erfinv}(x) = \operatorname{erf}^{-1}(x) \quad \text{for} \quad x \in (-1, 1)
/// ```
///
/// # Arguments
///
/// -1.0 <= x <= 1.0
///
/// # Panics
///
/// if x < -1.0 or x > 1.0
///
/// # Example
///
/// ```
/// use mathru::special::error;
///
/// let x: f64 = 0.0;
/// let error: f64 = error::erfinv(x);
/// ```
pub fn erfinv<T>(x: T) -> T
    where T: Real + Error
{
    x.erfinv()
}

fn horner<T>(x: T, c: &[T]) -> T
    where T: Real
{
    let last_idx: usize = c.len() -1;
    let mut f: T = c[last_idx] * x;

    for i in (1..last_idx).rev()
    {
        f = (f + c[i]) * x;
    }

    f + c[0]
}

fn copysign<T>(a: T, b: T) -> T
    where T: Real
{
    let a_neg: bool = a < T::zero();
    let b_neg: bool = b < T::zero();
    if (a_neg && !b_neg) || (!a_neg && b_neg)
    {
        return -a;
    }
    a
}


/// Inverse complementary error function
///
/// ```math
/// \operatorname{erfc}^{-1}(1-z) = \operatorname{erf}^{-1}(z)
/// ```
/// # Arguments
///
/// 0 <= x <= 2.0
///
/// # Panics
///
/// if x < 0.0 or x > 2.0
///
/// # Example
///
/// ```
/// use mathru::special::error;
///
/// let x: f64 = 1.0;
/// let error: f64 = error::erfcinv(x);
/// ```
pub fn erfcinv<T>(x: T) -> T
    where T: Real + Error
{
    x.erfcinv()
}
