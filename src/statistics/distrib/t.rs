use crate::{
    algebra::abstr::Real,
    special::{beta, gamma, hypergeometric},
    statistics::distrib::Continuous,
    special::beta::Beta,
    special::gamma::Gamma,
};
use crate::special::hypergeometric::Hypergeometric;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::clone::Clone;

/// T distribution
///
/// Fore more information:
/// <https://en.wikipedia.org/wiki/T_distribution>
///
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug)]
pub struct T<K>
{
    // degrees of freedom
    n: K,
}

impl<K> T<K> where K: Real
{
    /// Create a probability distribution
    ///
    /// # Arguments
    ///
    /// * `n`: > 0.0
    ///
    /// # Panics
    ///
    /// if n <= 0.0
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Continuous, T};
    ///
    /// let distrib: T<f64> = T::new(1.2);
    /// ```
    pub fn new(n: K) -> T<K>
    {
        if n < K::zero()
        {
            panic!()
        }

        T { n }
    }
}

impl<K> Continuous<K> for T<K> where K: Real + Beta + Hypergeometric + Gamma
{
    /// Probability density function
    ///
    ///
    /// # Arguments
    ///
    /// * `x` Random variable x &isin &#2115;
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Continuous, T};
    ///
    /// let distrib: T<f64> = T::new(2.0);
    /// let x: f64 = 0.5;
    /// let p: f64 = distrib.pdf(x);
    /// ```
    fn pdf(&self, x: K) -> K
    {
        gamma::gamma((self.n + K::one()) / K::from_f64(2.0)) * (K::one() + x * x / self.n).pow(-((self.n + K::one()) / K::from_f64(2.0)))
        / ((self.n * K::pi()).sqrt() * gamma::gamma(self.n / K::from_f64(2.0)))
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
    /// use mathru::statistics::distrib::{Continuous, T};
    ///
    /// let distrib: T<f64> = T::new(1.3);
    /// let x: f64 = 0.4;
    /// let p: f64 = distrib.cdf(x);
    /// ```
    fn cdf(&self, x: K) -> K
    {
        let k: K = (self.n + K::one()) / K::from_f64(2.0);
        let f21: K = hypergeometric::f21(K::from_f64(0.5),
                                         k,
                                         K::from_f64(1.5),
                                         -(x.pow(K::from_f64(2.0))) / self.n);
        K::from_f64(0.5) + x * gamma::gamma(k) * f21 / ((self.n * K::pi()).sqrt() * gamma::gamma(self.n / K::from_f64(2.0)))
    }

    /// Quantile function of inverse cdf
    fn quantile(&self, _p: K) -> K
    {
        unimplemented!();
    }

    /// Expected value
    ///
    /// # Panics
    ///
    /// if self.n <= 1.0
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Continuous, T};
    ///
    /// let distrib: T<f64> = T::new(1.2);
    /// let mean: f64 = distrib.mean();
    /// ```
    fn mean(&self) -> K
    {
        if self.n > K::one()
        {
            return K::zero();
        }
        panic!();
    }

    /// Variance
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Continuous, T};
    ///
    /// let distrib: T<f64> = T::new(2.2);
    /// let var: f64 = distrib.variance();
    /// ```
    fn variance(&self) -> K
    {
        if self.n > K::from_f64(2.0)
        {
            return self.n / (self.n - K::from_f64(2.0))
        }
        if self.n > K::one()
        {
            K::from_f64(std::f64::INFINITY)
        }
        else
        {
            panic!()
        }
    }

    ///
    /// # Panics
    ///
    /// if self.n <= 3
    fn skewness(&self) -> K
    {
        if self.n <= K::from_f64(3.0)
        {
            panic!("Skewness is not defined if degrees of freedom is smaller or equal 3");
        }
        K::zero()
    }

    /// Median is the value separating the higher half from the lower half of a
    /// probability distribution.
    fn median(&self) -> K
    {
        K::zero()
    }

    ///
    fn entropy(&self) -> K
    {
        let a: K = (self.n + K::one()) / K::from_f64(2.0);
        let b: K = self.n / K::from_f64(2.0);

        (a * (gamma::gamma(a) - gamma::gamma(b))) + (self.n.sqrt() * beta::beta(a, K::from_f64(0.5))).ln()
    }
}
