use crate::algebra::abstr::Real;
use std::iter;

pub trait Distribution<T>
    where T: Real
{
    fn random(&self) -> T;

    fn random_sequence(&self, size: u32) -> Vec<T>
    {
        let mut v: Vec<T> = Vec::new();
        v.extend(iter::repeat_with(&|| self.random()).take(size as usize));

        v
    }
}

/// Continuous distribution
pub trait Continuous<T>
    where T: Real
{
    /// Probability density function
    ///
    /// # Arguments
    ///
    /// *`x`:
    fn pdf(&self, x: T) -> T;

    /// Cumulative distribution function
    ///
    /// # Arguments
    ///
    /// *`x`:
    fn cdf(&self, x: T) -> T;

    /// Quantile function, inverse cdf
    fn quantile(&self, p: T) -> T;

    /// Mean
    fn mean(&self) -> T;

    /// Variance
    fn variance(&self) -> T;

    /// Skewness is a measure of the asymmetry of the probability distribution
    /// of a real-valued random variable about its mean
    fn skewness(&self) -> T;

    /// Median is the value separating the higher half from the lower half of a
    /// probability distribution.
    fn median(&self) -> T;

    ///
    fn entropy(&self) -> T;
}

/// Discrete distribution
pub trait Discrete<T, A, B>
{
    /// Probability mass function
    ///
    /// # Arguments
    ///
    /// *`x`:
    fn pmf(&self, x: A) -> T;

    ///Cumulative distribution function
    ///
    /// # Arguments
    ///
    /// * `x`:
    fn cdf(&self, x: B) -> T;

    /// Mean
    fn mean(&self) -> T;

    /// Variance
    fn variance(self: & Self) -> T;
}
