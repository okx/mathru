use crate::{
    algebra::abstr::Real,
    statistics::{
        distrib::{Continuous, Normal, T as TD},
        test::Test,
    },
    special::gamma::Gamma,
    special::beta::Beta,
    special::error::Error,
};
use crate::special::hypergeometric::Hypergeometric;
use std::clone::Clone;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// T-Test
///
/// Fore more information:
/// <https://en.wikipedia.org/wiki/Student%27s_t-test>
///
/// # Example
/// ```
/// use mathru::{
///     self,
///     statistics::{
///         distrib::{Distribution, Normal},
///         test::{Test, T},
///     },
/// };
///
/// let rv1 = Normal::new(1.0, 0.5).random_sequence(100);
/// let rv2 = Normal::new(1.0, 0.5).random_sequence(100);
///
/// //Test with sample with identical means
/// let mut measure: T<f64> = T::test_independence_unequal_variance(&rv1, &rv2);
/// println!("{}", measure.value());
/// measure = T::test_independence_equal_variance(&rv1, &rv2);
/// println!("{}", measure.value());
///
/// // Test with different equal mean, but unequal variances
/// let rv3 = Normal::new(1.0, 1.5).random_sequence(100);
/// measure = T::test_independence_unequal_variance(&rv1, &rv3);
/// println!("{}", measure.value());
/// measure = T::test_independence_equal_variance(&rv1, &rv3);
/// println!("{}", measure.value());
///
/// // When the sample size is not equal anymore
/// //the equal variance t-statistic is no longer equal to the unequal variance t-statistic:
/// let rv4 = Normal::new(2.0, 0.5).random_sequence(300);
/// measure = T::test_independence_unequal_variance(&rv1, &rv4);
/// println!("{}", measure.value());
/// measure = T::test_independence_equal_variance(&rv1, &rv4);
/// println!("{}", measure.value());
///
/// //t-Test with different mean, variance and sample size
/// let rv5 = Normal::new(2.0, 1.0).random_sequence(300);
/// measure = T::test_independence_unequal_variance(&rv1, &rv5);
/// println!("{}", measure.value());
/// measure = T::test_independence_equal_variance(&rv1, &rv5);
/// println!("{}", measure.value());
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug)]
pub struct T<K>
{
    p: K,
    t: K,
}

impl<K> Test<K> for T<K> where K: Real
{
    ///Test value
    fn value(&self) -> K
    {
        self.t
    }

    /// Degree of freedom
    fn df(&self) -> u32
    {
        0
    }

    ///
    fn p_value(&self) -> K
    {
        self.p
    }
}

impl<K> T<K> where K: Real + Gamma + Beta + Error + Hypergeometric
{
    /// This is a one-sided test for the null hypothesis that the expected value
    /// (mean) of a sample of independent observations a is equal
    /// to the given mean.
    ///
    /// x: observation
    pub fn one_sample(x: &Vec<K>, mu_0: K) -> T<K>
    {
        let n: u32 = x.len() as u32;

        let normal: Normal<K> = Normal::from_data(x);

        let x_bar: K = normal.mean();
        let s: K = normal.variance().sqrt();

        T { t: K::from_u32(n).sqrt() * (x_bar - mu_0) / s,
            p: K::zero() }
    }

    /// Calculates the T-test for the means of two independent samples of scores
    ///
    /// This is a two-sided test for the null hypothesis that two independent
    /// samples have identical expected values. It is assumed, that the
    /// populations have identical variances.
    pub fn test_independence_equal_variance(x: &Vec<K>, y: &Vec<K>) -> T<K>
    {
        let n_x: usize = x.len();
        let n_y: usize = y.len();

        let x_dist: Normal<K> = Normal::from_data(x);
        let y_dist: Normal<K> = Normal::from_data(y);

        let mean_x: K = x_dist.mean();
        let mean_y: K = y_dist.mean();

        let df: usize = n_x + n_y - 2;

        let s_x_squared: K = x_dist.variance();
        let s_y_squared: K = y_dist.variance();

        let nomin: K = K::from_f64((n_x - 1) as f64) * s_x_squared
                       + K::from_f64((n_y - 1) as f64) * s_y_squared;
        let denom: K = K::from((df) as f64);

        let s_p: K = (nomin / denom).sqrt();

        let t: K = (mean_x - mean_y)
                   / (s_p * K::from_f64((1.0 / (n_x as f64) + 1.0 / (n_y as f64)).sqrt()));
        T { p: K::zero(), t }
    }

    /// Calculates the T-test for the means of two independent samples of scores
    ///
    /// This is a two-sided test for the null hypothesis that two independent
    /// samples have identical expected values. It is assumed, that the
    /// populations have NOT identical variances. It performs the Welch’s t-test
    pub fn test_independence_unequal_variance(x: &Vec<K>, y: &Vec<K>) -> T<K>
    {
        let n_x: usize = x.len();
        let n_y: usize = y.len();

        let x_dist: Normal<K> = Normal::from_data(x);
        let y_dist: Normal<K> = Normal::from_data(y);

        let mean_x: K = x_dist.mean();
        let mean_y: K = y_dist.mean();

        let s_x_squared: K = x_dist.variance();
        let s_y_squared: K = y_dist.variance();

        let term1: K =
            s_x_squared / K::from_f64(n_x as f64) + s_y_squared / K::from_f64(n_y as f64);

        let df: K = term1 * term1
                    / (s_x_squared * s_x_squared / K::from_f64((n_x * n_x * (n_x - 1)) as f64)
                       + s_y_squared * s_y_squared / K::from_f64((n_y * n_y * (n_y - 1)) as f64));

        let s_p: K = term1.sqrt();

        let t: K = (mean_x - mean_y) / s_p;

        let p: K = K::from_f64(2.0) * TD::new(df).cdf(-t.abs());
        T { p, t }
    }
}
