use crate::{
    algebra::abstr::Real,
    statistics::distrib::{ChiSquare, Continuous},
    special::gamma::Gamma,
    special::error::Error,
};
use std::clone::Clone;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// G-Test
///
/// Fore more information:
/// <https://de.wikipedia.org/wiki/G-Test>
///
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug)]
pub struct G<T>
{
    df: u32,
    g: T,
}

impl<T> G<T> where T: Real + Gamma + Error
{
    ///
    /// \sum_{i}{y_{i}} = n = \sum_i{xs_i}
    /// b = \sum_{i}{x_{i}}
    /// k = n/b
    /// xs_{i} = x{i}*k
    ///
    /// x: observation
    /// y: expectation
    pub fn test_vector(x: &Vec<T>, y: &Vec<T>) -> G<T>
    {
        if x.len() != y.len()
        {
            panic!();
        }

        let df: u32 = (x.len() - 1) as u32;

        let mut n: T = T::zero();
        for y_i in y.iter()
        {
            n += *y_i;
        }

        let mut b: T = T::zero();
        for x_i in x.iter()
        {
            b += *x_i;
        }

        let k: T = n / b;

        let mut g: T = T::zero();

        for i in 0..x.len()
        {
            g += x[i] * (x[i] / (y[i] / k)).ln()
        }

        G { df,
            g: T::from_f64(2.0) * g }
    }

    pub fn df(&self) -> u32
    {
        self.df
    }

    pub fn g(&self) -> T
    {
        self.g
    }

    pub fn p_value(&self) -> T
    {
        let distrib: ChiSquare<T> = ChiSquare::new(self.df);
        T::one() - distrib.cdf(self.g)
    }
}
