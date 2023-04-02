//! Normal distribution
use crate::{
    algebra::abstr::Real,
    special::error,
    statistics::distrib::{Continuous, Distribution},
    special::gamma::Gamma,
    special::error::Error,
};
use rand;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::clone::Clone;

/// Normal distribution
///
/// Fore more information:
/// <https://en.wikipedia.org/wiki/Normal_distribution>
///
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug)]
pub struct Normal<T>
{
    mean: T,
    variance: T,
}

impl<T> Normal<T> where T: Real
{
    /// Creates a probability distribution
    ///
    /// # Arguments
    ///
    /// * `mean`: Expected value
    /// * `variance`: variance > 0.0
    ///
    /// # Panics
    ///
    /// if variance <= 0.0
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::Normal;
    ///
    /// let distrib: Normal<f64> = Normal::new(0.3, 0.2);
    /// ```
    pub fn new(mean: T, variance: T) -> Self
    {
        if variance <= T::zero()
        {
            panic!();
        }

        Normal { mean, variance }
    }

    /// It is assumed that data are normal distributed.
    ///
    /// data.len() >= 2
    pub fn from_data(data: &Vec<T>) -> Self
    {
        let n: usize = data.len();
        if n < 2
        {
            panic!()
        }

        let mean: T = Normal::calc_mean(data);
        let variance: T = Normal::calc_variance(data, mean);

        Normal::new(mean, variance)
    }

    fn calc_mean(data: &Vec<T>) -> T
    {
        let n: usize = data.len();
        let mut sum: T = T::zero();

        for x in data.iter()
        {
            sum += *x;
        }

        sum / T::from_u64(n as u64)
    }

    fn calc_variance(data: &Vec<T>, mean: T) -> T
    {
        let n: usize = data.len();
        let mut sum: T = T::zero();

        for x in data.iter()
        {
            sum += (*x - mean).pow(T::from_f64(2.0));
        }

        sum / T::from_u64((n - 1) as u64)
    }
}

impl<T> Continuous<T> for Normal<T>
    where T: Real + Gamma + Error
{
    /// Probability density function
    ///
    /// # Arguments
    ///
    /// * `x`:  x &isin; &#x2115
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Continuous, Normal};
    ///
    /// let distrib: Normal<f64> = Normal::new(0.3, 0.2);
    /// let x: f64 = 5.0;
    /// let p: f64 = distrib.pdf(x);
    /// ```
    fn pdf(&self, x: T) -> T
    {
        let z: T = T::from_f64(-0.5) * ((x - self.mean) / self.variance).pow(T::from_f64(2.0));
        let f: T = T::one() / (self.variance * T::from_f64(2.0) * T::pi()).sqrt();

        f * z.exp()
    }

    /// Cumulative distribution function
    ///
    /// # Arguments
    ///
    /// * `x`:
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Continuous, Normal};
    ///
    /// let distrib: Normal<f64> = Normal::new(0.3, 0.2);
    /// let x: f64 = 0.4;
    /// let p: f64 = distrib.cdf(x);
    /// ```
    fn cdf(&self, x: T) -> T
    {
        let k: T = (x - self.mean) / ((T::from_f64(2.0) * self.variance).sqrt());
        let prob: T = T::from_f64(0.5) * (T::one() + error::erf(k));
        prob
    }

    /// Quantile: function of inverse cdf
    ///
    /// The Percentage Points of the Normal Distribution
    ///  Author(s): Michael J. Wichura
    /// Year 1988
    /// Journal of the Royal Statistical Society
    /// 0.0 < p < 1.0
    ///
    /// # Panics
    ///
    /// if  p <= 0.0 || p >= 1.0
    fn quantile(&self, p: T) -> T
    {
        if p <= T::zero() || p >= T::one()
        {
            panic!();
        }

        let mut ppnd16: T;
        let mut r: T;
        let q: T = p - T::from_f64(0.5);
        if q.abs() <= T::from_f64(0.425)
        {
            let r: T = T::from_f64(0.180625) - q * q;
            ppnd16 = q * Normal::r1(r);
        }
        else
        {
            if q < T::zero()
            {
                r = p
            }
            else
            {
                r = T::one() - p
            }
            if r <= T::zero()
            {
                ppnd16 = T::zero();
                return ppnd16;
            }
            r = (-r.ln()).sqrt();
            if r <= T::from_f64(5.)
            {
                r -= T::from_f64(1.6);
                ppnd16 = Normal::r2(r);
            }
            else
            {
                r -= T::from_f64(5.0);
                ppnd16 = Normal::r3(r);
            }
            if q <= T::zero()
            {
                ppnd16 = -ppnd16;
            }
        }

        self.mean + self.variance.sqrt() * ppnd16
    }

    /// Expected value
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::{
    ///     self,
    ///     statistics::distrib::{Continuous, Normal},
    /// };
    ///
    /// let distrib: Normal<f64> = Normal::new(0.0, 0.2);
    /// let mean: f64 = distrib.mean();
    /// ```
    fn mean(&self) -> T
    {
        self.mean
    }

    /// Variance
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::{
    ///     self,
    ///     statistics::distrib::{Continuous, Normal},
    /// };
    ///
    /// let distrib: Normal<f64> = Normal::new(0.0, 0.2);
    /// let var: f64 = distrib.variance();
    /// ```
    fn variance(&self) -> T
    {
        self.variance
    }

    /// Skewness
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::{
    ///     self,
    ///     statistics::distrib::{Continuous, Normal},
    /// };
    /// let mean: f64 = 1.0;
    /// let variance: f64 = 0.5;
    /// let distrib: Normal<f64> = Normal::new(mean, variance);
    /// assert_eq!(0.0, distrib.skewness());
    /// ```
    fn skewness(&self) -> T
    {
        T::zero()
    }

    /// Median
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::{
    ///     self,
    ///     statistics::distrib::{Continuous, Normal},
    /// };
    ///
    /// let mean: f64 = 0.0;
    ///
    /// let distrib: Normal<f64> = Normal::new(mean, 0.2);
    /// let median: f64 = distrib.median();
    /// ```
    fn median(&self) -> T
    {
        self.mean
    }

    /// Entropy
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::{
    ///     self,
    ///     statistics::distrib::{Continuous, Normal},
    /// };
    /// use std::f64::consts::{E, PI};
    ///
    /// let mean: f64 = 1.0;
    /// let variance: f64 = 0.5;
    /// let distrib: Normal<f64> = Normal::new(mean, variance);
    ///
    /// let entropy: f64 =  distrib.entropy();
    /// ```
    fn entropy(&self) -> T
    {
        T::from_f64(2.0) * T::pi() * T::e() * self.variance
    }
}

impl<T> Distribution<T> for Normal<T> where T: Real
{
    ///
    ///  See Knuth The Art of Computer Programming Vol 2 3.4.1 C Algorithm P
    fn random(&self) -> T
    {
        let mut s: T = T::one();
        let mut v1: T = T::one();
        let mut v2: T;

        while s >= T::one()
        {
            let u1: T = T::from_f64(rand::random::<f64>());
            let u2: T = T::from_f64(rand::random::<f64>());
            v1 = T::from_f64(2.0) * u1 - T::one();
            v2 = T::from_f64(2.0) * u2 - T::one();
            s = v1 * v1 + v2 * v2
        }
        let x1: T = v1 * (-T::from_f64(2.0) * s.ln() / s).sqrt();
        x1 * self.variance.sqrt() + self.mean
    }
}

impl<T> Normal<T> where T: Real
{
    fn r1(t: T) -> T
    {
        let r: f64 = t.to_f64();
        let value: f64 = (((((((r * 2_509.080_928_730_122_7 + 33_430.575_583_588_13) * r
                               + 67_265.770_927_008_7)
                              * r
                              + 45_921.953_931_549_87)
                             * r
                             + 13_731.693_765_509_46)
                            * r
                            + 1_971.590_950_306_551_3)
                           * r
                           + 133.141_667_891_784_38)
                          * r
                          + 3.387_132_872_796_366_5)
                         / (((((((r * 5_226.495_278_852_854 + 28_729.085_735_721_943) * r
                                 + 39_307.895_800_092_71)
                                * r
                                + 21_213.794_301_586_597)
                               * r
                               + 5_394.196_021_424_751)
                              * r
                              + 687.187_007_492_057_9)
                             * r
                             + 42.313_330_701_600_91)
                            * r
                            + 1.);
        T::from_f64(value)
    }

    fn r2(t: T) -> T
    {
        let r: f64 = t.to_f64();
        let value: f64 = (((((((r * 7.745_450_142_783_414e-4 + 0.022_723_844_989_269_184) * r
                               + 0.241_780_725_177_450_6)
                              * r
                              + 1.270_458_252_452_368_4)
                             * r
                             + 3.647_848_324_763_204_5)
                            * r
                            + 5.769_497_221_460_691)
                           * r
                           + 4.630_337_846_156_546)
                          * r
                          + 1.423_437_110_749_683_5)
                         / (((((((r * 1.050_750_071_644_416_9e-9 + 5.475_938_084_995_345e-4)
                                 * r
                                 + 0.015_198_666_563_616_457)
                                * r
                                + 0.148_103_976_427_480_08)
                               * r
                               + 0.689_767_334_985_1)
                              * r
                              + 1.676_384_830_183_803_8)
                             * r
                             + 2.053_191_626_637_759)
                            * r
                            + 1.);

        T::from_f64(value)
    }

    fn r3(t: T) -> T
    {
        let r: f64 = t.to_f64();
        let value: f64 =
            (((((((r * 2.010_334_399_292_288_1e-7 + 2.711_555_568_743_487_6e-5) * r
                  + 0.001_242_660_947_388_078_4)
                 * r
                 + 0.026_532_189_526_576_124)
                * r
                + 0.296_560_571_828_504_87)
               * r
               + 1.784_826_539_917_291_3)
              * r
              + 5.463_784_911_164_114)
             * r
             + 6.657_904_643_501_103)
            / (((((((r * 2.044_263_103_389_939_7e-15 + 1.421_511_758_316_446e-7) * r
                    + 1.846_318_317_510_054_8e-5)
                   * r
                   + 7.868_691_311_456_133e-4)
                  * r
                  + 0.014_875_361_290_850_615)
                 * r
                 + 0.136_929_880_922_735_8)
                * r
                + 0.599_832_206_555_888)
               * r
               + 1.);

        T::from_f64(value)
    }
}
