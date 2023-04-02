//! Gamma distribution
use crate::{algebra::abstr::Real, special::gamma, statistics::distrib::Continuous};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::clone::Clone;

/// Gamma distribution
///
/// Fore more information:
/// <https://en.wikipedia.org/wiki/Gamma_distribution>
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug)]
pub struct Gamma<T>
{
    alpha: T,
    beta: T,
}

impl<T> Gamma<T> where T: Real
{
    /// Creates a probability distribution
    ///
    /// # Arguments
    ///
    /// * `alpha` > 0.0
    /// * `beta` > 0.0
    ///
    /// # Panics
    ///
    /// if alpha <= 0.0 || beta <= 0.0
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::Gamma;
    ///
    /// let distrib: Gamma<f64> = Gamma::new(0.3, 0.2);
    /// ```
    pub fn new(alpha: T, beta: T) -> Gamma<T>
    {
        if alpha <= T::zero() || beta <= T::zero()
        {
            panic!()
        }
        Gamma { alpha, beta }
    }
}

impl<T> Continuous<T> for Gamma<T>
    where T: Real + gamma::Gamma
{
    /// Probability density function
    ///
    /// # Arguments
    ///
    /// * `x` Random variable x &isin; &#x2115 | x > 0.0
    ///
    /// # Panics
    ///
    /// if x <= 0.0
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Continuous, Gamma};
    ///
    /// let distrib: Gamma<f64> = Gamma::new(0.3, 0.2);
    /// let x: f64 = 5.0;
    /// let p: f64 = distrib.pdf(x);
    /// ```
    fn pdf(&self, x: T) -> T
    {
        if x <= T::zero()
        {
            panic!();
        }
        self.beta.pow(self.alpha) / gamma::gamma(self.alpha) * x.pow(self.alpha - T::one()) * (-self.beta * x).exp()
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
    /// use mathru::statistics::distrib::{Continuous, Gamma};
    ///
    /// let distrib: Gamma<f64> = Gamma::new(0.3, 0.2);
    /// let x: f64 = 0.4;
    /// let p: f64 = distrib.cdf(x);
    /// ```
    fn cdf(&self, x: T) -> T
    {
        if x == T::zero()
        {
            return T::zero();
        }
        gamma::gamma_lr(self.alpha, self.beta * x)
    }

    /// Quantile function of inverse cdf
    fn quantile(&self, _p: T) -> T
    {
        unimplemented!();
    }

    /// Expected value
    fn mean(&self) -> T
    {
        self.alpha / self.beta
    }

    /// Variance
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Continuous, Gamma};
    ///
    /// let distrib: Gamma<f64> = Gamma::new(0.2, 0.5);
    /// let var: f64 = distrib.variance();
    /// ```
    fn variance(&self) -> T
    {
        self.alpha / self.beta.pow(T::from_f64(2.0))
    }

    ///
    fn skewness(&self) -> T
    {
        T::from_f64(2.0) / self.alpha.sqrt()
    }

    /// Median is the value separating the higher half from the lower half of a
    /// probability distribution.
    fn median(&self) -> T
    {
        unimplemented!();
    }

    ///
    fn entropy(&self) -> T
    {
        self.alpha - self.beta.ln() + gamma::gamma(self.alpha).ln() + (T::one() - self.alpha) * gamma::digamma(self.alpha)
    }
}
