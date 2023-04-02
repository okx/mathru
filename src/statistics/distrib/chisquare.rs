//! Chi-Square distribution

use crate::{algebra::abstr::Real, special::error::Error, special::gamma::Gamma, statistics::distrib::Continuous};
use crate::special::gamma;
use crate::special::error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::clone::Clone;

/// Chi-Square distribution
///
/// Fore more information:
/// <https://en.wikipedia.org/wiki/Chi-square_distribution>
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug)]
pub struct ChiSquare<T>
{
    ///degree of freedom
    k: T,
}

impl<T> ChiSquare<T> where T: Real
{
    /// Creates a probability distribution
    ///
    /// # Arguments
    ///
    /// * `df`: Degree of freedom, df >= 1
    ///
    /// # Panics
    ///
    /// if df < 1
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::ChiSquare;
    ///
    /// let distrib: ChiSquare<f64> = ChiSquare::new(3);
    /// ```
    pub fn new(df: u32) -> ChiSquare<T>
    {
        if T::from_u32(df) < T::one()
        {
            panic!()
        }
        ChiSquare { k: T::from_u32(df) }
    }
}

impl<T> Continuous<T> for ChiSquare<T>
    where T: Real + Gamma + Error
{
    /// Probability density function
    ///
    /// # Arguments
    ///
    /// * `x` Random variable x &isin; &#x2115
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{ChiSquare, Continuous};
    ///
    /// let distrib: ChiSquare<f64> = ChiSquare::new(2);
    /// let x: f64 = 5.0;
    /// let p: f64 = distrib.pdf(x);
    /// ```
    fn pdf(&self, x: T) -> T
    {
        if x < T::zero()
        {
            return T::zero();
        }
        let t1: T = T::one()
                    / (T::from_f64(2.0).pow(self.k / T::from_f64(2.0))
                       * gamma::gamma(self.k / T::from_f64(2.0)));
        let t2: T = x.pow(self.k / T::from_f64(2.0) - T::one()) * (-x / T::from_f64(2.0)).exp();
        let chisquare: T = t1 * t2;

        chisquare
    }

    /// Cumulative distribution function
    ///
    /// # Arguments
    ///
    /// * `x` Random variable
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{ChiSquare, Continuous};
    ///
    /// let distrib: ChiSquare<f64> = ChiSquare::new(3);
    /// let x: f64 = 0.4;
    /// let p: f64 = distrib.cdf(x);
    /// ```
    fn cdf(&self, x: T) -> T
    {
        let t1: T = (-x / T::from_f64(2.0)).exp();

        let k_natural: u32 = self.k.to_u32();
        let p: T;

        if k_natural % 2 == 0
        {
            let mut sum: T = T::zero();
            for i in 0..(self.k / T::from_f64(2.0)).to_u32()
            {
                sum += (x / T::from_f64(2.0)).pow(T::from_u32(i)) / gamma::gamma(T::from_u32(i + 1))
            }

            p = T::one() - t1 * sum;
        }
        else
        {
            let mut sum: T = T::zero();
            for i in 0..(self.k / T::from_f64(2.0)).to_u32()
            {
                sum += (x / T::from_f64(2.0)).pow(T::from_f64((i as f64) + 0.5)) / gamma::gamma(T::from_f64((i as f64) + 1.5));
            }

            p = error::erf((x / T::from_f64(2.0)).sqrt()) - t1 * sum;
        }

        p
    }

    /// Quantile function or inverse cdf
    fn quantile(&self, p: T) -> T
    {
        T::from_f64(2.0) * (self.k / T::from_f64(2.0)).gamma_ur_inv(T::from_f64(1.0) - p)
    }

    /// Expected value
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{ChiSquare, Continuous};
    ///
    /// let distrib: ChiSquare<f64> = ChiSquare::new(2);
    /// let mean: f64 = distrib.mean();
    /// ```
    fn mean(&self) -> T
    {
        self.k
    }

    /// Variance
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{ChiSquare, Continuous};
    ///
    /// let distrib: ChiSquare<f64> = ChiSquare::new(2);
    /// let var: f64 = distrib.variance();
    /// ```
    fn variance(&self) -> T
    {
        T::from_f64(2.0) * self.k
    }

    /// Skewness is a measure of the asymmetry of the probability distribution
    /// of a real-valued random variable about its mean
    fn skewness(&self) -> T
    {
        (T::from_f64(8.0) / self.k).sqrt()
    }

    /// Median is the value separating the higher half from the lower half of a
    /// probability distribution.
    fn median(&self) -> T
    {
        let t: T = T::one() - T::from_f64(2.0 / 9.0) / self.k;

        self.k * t * t * t
    }

    ///
    fn entropy(&self) -> T
    {
        let d: T = T::from_f64(2.0) / self.k;
        d + (T::from_f64(2.0) * gamma::gamma(d)).ln() + (T::one() - d) * gamma::digamma(d)
    }
}
