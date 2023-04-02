//! Bernoulli Distribution
use crate::{algebra::abstr::Real, statistics::distrib::Discrete};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::clone::Clone;

/// Bernoulli distribution
///
/// Fore more information:
///
/// <https://en.wikipedia.org/wiki/Bernoulli_distribution>
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug)]
pub struct Bernoulli<T>
{
    p: T,
}

impl<T> Bernoulli<T> where T: Real
{
    /// Create a probability distribution with p(X=1) = p
    ///
    /// # Arguments
    ///
    /// * `p` Probability that random varibale X=1, 0.0 <= p <= 1.0
    ///
    /// # Panics
    ///
    /// if p < 0 || p > 1.0
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::Bernoulli;
    ///
    /// let distrib: Bernoulli<f64> = Bernoulli::new(0.2);
    /// ```
    pub fn new(p: T) -> Bernoulli<T>
    {
        if p < T::zero() || p > T::one()
        {
            panic!()
        }

        Bernoulli { p }
    }
}

impl<T> Discrete<T, u8, T> for Bernoulli<T> where T: Real
{
    /// Probability mass function of the Bernoulli distribution
    ///
    ///
    /// # Arguments
    ///
    /// * `x` Random variable x &isin; {0, 1}
    ///
    /// # Panics
    ///
    /// if x &notin; {0, 1}
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Bernoulli, Discrete};
    ///
    /// let distrib: Bernoulli<f64> = Bernoulli::new(0.2);
    /// let x: u8 = 0;
    /// let p: f64 = distrib.pmf(x);
    /// ```
    fn pmf(&self, x: u8) -> T
    {
        if (x == 1) || (x == 0)
        {
            if x == 0
            {
                T::one() - self.p
            } else {
                self.p
            }
        }
        else
        {
            panic!()
        }
    }

    /// Cumulative distribution function of the Bernoulli distribution
    ///
    /// # Arguments
    ///
    /// * `x` Random variable x &isin; {0, 1}
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Bernoulli, Discrete};
    ///
    /// let distrib: Bernoulli<f64> = Bernoulli::new(0.2);
    /// let x: f64 = 0.4;
    /// let p: f64 = distrib.cdf(x);
    /// ```
    fn cdf(&self, x: T) -> T
    {
        if x >= T::one()
        {
            return T::one();
        }

        if x <= T::zero()
        {
            T::zero()
        } else {
            T::one() - self.p
        }
    }

    /// Expected value
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Bernoulli, Discrete};
    ///
    /// let distrib: Bernoulli<f64> = Bernoulli::new(0.2);
    /// let mean: f64 = distrib.mean();
    /// ```
    fn mean(&self) -> T
    {
        self.p
    }

    /// Variance
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Bernoulli, Discrete};
    ///
    /// let distrib: Bernoulli<f64> = Bernoulli::new(0.2);
    /// let var: f64 = distrib.variance();
    /// ```
    fn variance(&self) -> T
    {
        self.p * (T::one() - self.p)
    }
}
