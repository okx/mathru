use crate::{
    algebra::abstr::Real,
    statistics::distrib::{Continuous, Distribution},
};
use rand::{rngs::ThreadRng, Rng};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::clone::Clone;

/// Uniform distribution
///
/// Fore more information:
/// <https://en.wikipedia.org/wiki/Uniform_distribution_(continuous)>
///
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug)]
pub struct Uniform<T>
{
    a: T,
    b: T,
}

impl<T> Uniform<T> where T: Real
{
    /// Uniform distribution
    ///
    /// # Arguments
    ///
    /// -\inf < a < b < \inf
    ///
    /// a: lower bound
    /// b: upper bound
    ///
    /// a < b
    ///
    /// # Panic
    ///
    /// a >= b
    pub fn new(a: T, b: T) -> Uniform<T>
    {
        if a >= b
        {
            panic!();
        }

        Uniform { a, b }
    }
}

impl<T> Distribution<T> for Uniform<T> where T: Real
{
    fn random(&self) -> T
    {
        let mut rng: ThreadRng = rand::thread_rng();
        T::from_f64(rng.gen_range(self.a.to_f64()..self.b.to_f64()))
    }
}

impl<T> Continuous<T> for Uniform<T> where T: Real
{
    /// Probability density function
    ///
    /// # Arguments
    ///
    /// x:
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Continuous, Uniform};
    ///
    /// let distrib: Uniform<f64> = Uniform::new(-0.1, 0.3);
    /// let x: f64 = 5.0;
    /// let p: f64 = distrib.pdf(x);
    /// ```
    fn pdf(&self, x: T) -> T
    {
        if self.a <= x && x <= self.b
        {
            T::one() / (self.b - self.a)
        }
        else
        {
            T::zero()
        }
    }

    /// Cumulative distribution function
    ///
    /// # Arguments
    ///
    /// * `x`
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Continuous, Uniform};
    ///
    /// let distrib: Uniform<f64> = Uniform::new(0.0, 0.5);
    /// let x: f64 = 0.3;
    /// let p: f64 = distrib.cdf(x);
    /// ```
    fn cdf(&self, x: T) -> T
    {
        if x < self.a
        {
            T::zero()
        }
        else
        {
            if x > self.b
            {
                T::one()
            }
            else
            {
                (x - self.a) / (self.b - self.a)
            }
        }
    }

    /// Quantile function or inverse cdf
    ///
    /// # Arguments
    ///
    /// * `q`: quantile 0 <= q <= 1
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Continuous, Uniform};
    ///
    /// let distrib: Uniform<f64> = Uniform::new(0.0, 0.5);
    /// let q: f64 = 0.3;
    /// let x: f64 = distrib.quantile(q);
    /// ```
    fn quantile(&self, q: T) -> T
    {
        if q > T::one() || q < T::zero()
        {
            panic!("Quantile q is out of bounds");
        }

        q * (self.b - self.a) + self.a
    }

    /// Mean
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Continuous, Uniform};
    ///
    /// let a: f64 = 0.2;
    /// let b: f64 = 0.5;
    ///
    /// let distrib: Uniform<f64> = Uniform::new(a, b);
    /// let mean: f64 = distrib.mean();
    /// assert_eq!((a + b) / 2.0, mean);
    /// ```
    fn mean(&self) -> T
    {
        (self.a + self.b) / T::from_f64(2.0)
    }

    /// Variance
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Continuous, Uniform};
    ///
    /// let distrib: Uniform<f64> = Uniform::new(0.2, 0.5);
    /// let var: f64 = distrib.variance();
    /// ```
    fn variance(&self) -> T
    {
        (self.b - self.a) * (self.b - self.a) / T::from_f64(12.0)
    }

    /// Skewness
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Continuous, Uniform};
    ///
    /// let distrib: Uniform<f64> = Uniform::new(0.2, 0.5);
    /// let skewness: f64 = distrib.skewness();
    /// assert_eq!(0.0, skewness);
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
    /// use mathru::statistics::distrib::{Continuous, Uniform};
    ///
    /// let a: f64 = 0.2;
    /// let b: f64 = 0.5;
    ///
    /// let distrib: Uniform<f64> = Uniform::new(a, b);
    /// let median: f64 = distrib.median();
    /// assert_eq!((a + b) / 2.0, median);
    /// ```
    fn median(&self) -> T
    {
        self.mean()
    }

    /// Entropy
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Continuous, Uniform};
    ///
    /// let a: f64 = 0.2;
    /// let b: f64 = 0.5;
    ///
    /// let distrib: Uniform<f64> = Uniform::new(a, b);
    /// let entropy: f64 = distrib.entropy();
    /// assert_eq!((b - a).ln(), entropy);
    /// ```
    fn entropy(&self) -> T
    {
        (self.b - self.a).ln()
    }
}
