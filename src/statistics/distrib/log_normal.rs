use crate::{algebra::abstr::Real, statistics::distrib::{Distribution, Continuous, Normal}};
use crate::special::{gamma::Gamma, error::Error};
use std::f64::consts::PI;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::clone::Clone;

/// Log-Normal distribution
///
/// Fore more information:
/// <https://en.wikipedia.org/wiki/Log-normal_distribution>
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug)]
pub struct LogNormal<T>
{
    mu: T,
    sigma_squared: T,
}

impl<T> LogNormal<T> where T: Real
{
    /// Creates a probability distribution
    ///
    /// # Arguments
    ///
    /// * `mu`:
    /// * `sigma_squared`:
    ///
    /// # Panics
    ///
    /// if sigma_squared <= 0.0
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::LogNormal;
    ///
    /// let distrib: LogNormal<f64> = LogNormal::new(0.3, 0.2);
    /// ```
    pub fn new(mu: T, sigma_squared: T) -> Self
    {
        if sigma_squared <= T::zero()
        {
            panic!();
        }

        LogNormal { mu, sigma_squared }
    }

    /// It is assumed that data are normal distributed.
    ///
    /// data.len() >= 2
    pub fn from_data(_data: &Vec<T>) -> Self
    {
        unimplemented!()
    }

}

impl<T> Continuous<T> for LogNormal<T> where T: Real + Error + Gamma
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
    /// use mathru::statistics::distrib::{Continuous, LogNormal};
    ///
    /// let distrib: LogNormal<f64> = LogNormal::new(0.3, 0.2);
    /// let x: f64 = 5.0;
    /// let p: f64 = distrib.pdf(x);
    /// ```
    fn pdf(&self, x: T) -> T
    {
        if x < T::zero()
        {
            return T::zero();
        }
        let z: T = T::from_f64(-0.5) * (x.ln() - self.mu).pow(T::from_f64(2.0)) / self.sigma_squared;
        let f: T = T::one() / (x * (self.sigma_squared * T::from_f64(2.0) * T::pi()).sqrt());

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
    /// use mathru::statistics::distrib::{Continuous, LogNormal};
    ///
    /// let distrib: LogNormal<f64> = LogNormal::new(0.3, 0.2);
    /// let x: f64 = 0.4;
    /// let p: f64 = distrib.cdf(x);
    /// ```
    fn cdf(&self, x: T) -> T
    {
        if x <= T::zero()
        {
            return T::zero();
        }
        let p: T = (x.ln() - self.mu) / (T::from_f64(2.0) * self.sigma_squared).sqrt();
        T::from_f64(0.5) + T::from_f64(0.5) * p.erf()
    }

    /// Quantile: function of inverse cdf
    ///
    /// # Panics
    ///
    /// if  p <= 0.0 || p >= 1.0
    fn quantile(&self, p: T) -> T
    {
        if p <= T::zero() || p >= T::one()
        {
            panic!()
        }
        let std_distrib: Normal<T> = Normal::new(T::zero(), T::one());
        (self.mu + std_distrib.quantile(p) * self.sigma_squared.sqrt()).exp()
    }

    /// Expected value
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::{
    ///     self,
    ///     statistics::distrib::{Continuous, LogNormal},
    /// };
    ///
    /// let distrib: LogNormal<f64> = LogNormal::new(0.0, 0.2);
    /// let mean: f64 = distrib.mean();
    /// ```
    fn mean(&self) -> T
    {
        (self.mu + self.sigma_squared / T::from_f64(2.0)).exp()
    }

    /// Variance
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::{
    ///     self,
    ///     statistics::distrib::{Continuous, LogNormal},
    /// };
    ///
    /// let sigma_squared: f64 = 0.2;
    /// let distrib: LogNormal<f64> = LogNormal::new(0.0, sigma_squared);
    /// let var: f64 = distrib.variance();
    /// assert_eq!((sigma_squared.exp() - 1.0) * sigma_squared.exp(),  var )
    /// ```
    fn variance(&self) -> T
    {
        (self.sigma_squared.exp() - T::one()) * (T::from_f64(2.0) * self.mu + self.sigma_squared).exp()
    }

    /// Skewness
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::{
    ///     self,
    ///     statistics::distrib::{Continuous, LogNormal},
    /// };
    /// let mu: f64 = 1.0;
    /// let sigma_squared: f64 = 0.5;
    /// let distrib: LogNormal<f64> = LogNormal::new(mu, sigma_squared);
    /// ```
    fn skewness(&self) -> T
    {
        (self.sigma_squared.exp() + T::from_f64(2.0)) * (self.sigma_squared.exp() - T::one()).sqrt()
    }

    /// Median
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::{
    ///     self,
    ///     statistics::distrib::{Continuous, LogNormal},
    /// };
    ///
    /// let mu: f64 = 0.0;
    ///
    /// let distrib: LogNormal<f64> = LogNormal::new(mu, 0.2);
    /// let median: f64 = distrib.median();
    /// assert_eq!(median, 1.0);
    /// ```
    fn median(&self) -> T
    {
        self.mu.exp()
    }

    /// Entropy
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::{
    ///     self,
    ///     statistics::distrib::{Continuous, LogNormal},
    /// };
    ///
    /// let mu: f64 = 1.0;
    /// let sigma_squared: f64 = 0.5;
    /// let distrib: LogNormal<f64> = LogNormal::new(mu, sigma_squared);
    ///
    /// ```
    fn entropy(&self) -> T
    {
        let k: T = self.sigma_squared.sqrt() * (self.mu + T::from_f64(0.5)).exp() * T::from_f64((2.0 * PI).sqrt());
        k.ln() / (T::from_f64(2.0)).ln()
    }
}

impl<T> Distribution<T> for LogNormal<T> where T: Real
{
    fn random(&self) -> T
    {
        unimplemented!()
    }
}