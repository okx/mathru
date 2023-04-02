use crate::{
    algebra::abstr::Real,
    statistics::{
        distrib::{ChiSquare as ChiSquareDistrib, Continuous},
        test::Test,
    },
    special::{error::Error, gamma::Gamma},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::clone::Clone;

/// Chi-Square Test
///
/// Fore more information:
/// <https://en.wikipedia.org/wiki/Chi-square_test>
///
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug)]
pub struct ChiSquare<T>
{
    df: u32,
    chi_square: T,
}

impl<T> ChiSquare<T> where T: Real
{
    ///
    /// alpha: significance level
    pub fn test_vector(x: &Vec<T>, y: &Vec<T>) -> ChiSquare<T>
    {
        if x.len() != y.len()
        {
            panic!();
        }

        let df: u32 = (y.len() - 1) as u32;

        let mut sum_x: T = T::zero();
        for n_i in x.iter()
        {
            sum_x += *n_i;
        }

        let mut sum_y: T = T::zero();
        for n_j in y.iter()
        {
            sum_y += *n_j;
        }

        let n: T = sum_x + sum_y;

        let mut chi_square: T = T::zero();
        let m: usize = x.len();
        for j in 0..m
        {
            for k in 0..2
            {
                let n_jk: T;
                let mut n_k: T = T::zero();
                if k == 0
                {
                    n_jk = x[j];
                    for l in x.iter().take(m)
                    {
                        n_k += *l;
                    }
                }
                else
                {
                    n_jk = y[j];
                    for l in y.iter().take(m)
                    {
                        n_k += *l
                    }
                }

                let n_j_: T = x[j] + y[j];

                let n_jks: T = (n_k * n_j_) / (n);
                chi_square += (n_jk - n_jks).pow(T::from_f64(2.0)) / n_jks
            }
        }

        ChiSquare { chi_square, df }
    }
}

impl<T> Test<T> for ChiSquare<T>
    where T: Real + Gamma + Error
{
    fn df(&self) -> u32
    {
        self.df
    }

    fn value(&self) -> T
    {
        self.chi_square
    }

    fn p_value(&self) -> T
    {
        let distrib: ChiSquareDistrib<T> = ChiSquareDistrib::new(self.df);
        T::one() - distrib.cdf(self.chi_square)
    }
}
