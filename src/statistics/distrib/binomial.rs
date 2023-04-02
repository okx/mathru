// Binomial distribution
use crate::{
    algebra::abstr::Real,
    statistics::{combins, distrib::Discrete},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::clone::Clone;

/// Binomial distribution
///
/// Fore more information:
/// <https://en.wikipedia.org/wiki/Binomial_distribution>
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug)]
pub struct Binomial<T>
{
    p: T,
    n: u32,
}

impl<T> Binomial<T>
{
    /// Create a probability distribution with
    ///
    /// # Arguments
    ///
    /// * `p` Probability that random variable, p &isin; [0, 1]
    /// * `n` number of trials, n &isin; &#x2115;
    ///
    /// # Panics
    ///
    /// if p < 0 || p > 1.0
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::Binomial;
    ///
    /// let distrib: Binomial<f64> = Binomial::new(5, 0.3);
    /// ```
    pub fn new(n: u32, p: T) -> Binomial<T>
    {
        Binomial { p, n }
    }
}

impl<T> Discrete<T, u32, T> for Binomial<T> where T: Real
{
    /// Probability mass function
    ///
    /// # Arguments
    ///
    /// * `x` Random variable x &isin &#2115;
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Binomial, Discrete};
    ///
    /// let distrib: Binomial<f64> = Binomial::new(5, 0.3);
    /// let x: u32 = 0;
    /// let p: f64 = distrib.pmf(x);
    /// ```
    fn pmf(&self, x: u32) -> T
    {
        if x > self.n
        {
            return T::zero();
        }
        let f: T = T::from_u32(combins::binom(self.n, x));
        let diff: i32 = (self.n as i32) - (x as i32);
        let pdf: T = f * (self.p.pow(T::from_u32(x))) * ((T::one() - self.p).pow(T::from_i32(diff)));

        pdf
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
    /// use mathru::statistics::distrib::{Binomial, Discrete};
    ///
    /// let distrib: Binomial<f64> = Binomial::new(5, 0.3);
    /// let x: f64 = 0.4;
    /// let p: f64 = distrib.cdf(x);
    /// ```
    fn cdf(&self, x: T) -> T
    {
        let x_supremum: u32 = x.floor().to_u32();
        let mut prob: T = T::zero();

        for k in 0..x_supremum + 1
        {
            prob += self.pmf(k);
        }
        prob
    }

    /// Expected value
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Binomial, Discrete};
    ///
    /// let distrib: Binomial<f64> = Binomial::new(5, 0.3);
    /// let mean: f64 = distrib.mean();
    /// ```
    fn mean(&self) -> T
    {
        T::from_u32(self.n) * self.p
    }

    /// Variance
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Binomial, Discrete};
    ///
    /// let distrib: Binomial<f64> = Binomial::new(5, 0.3);
    /// let var: f64 = distrib.variance();
    /// ```
    fn variance(&self) -> T
    {
        self.mean() * (T::one() - self.p)
    }
}
