use crate::analysis::integral::newton_cotes::ClosedFixedIntervalIterator;
use crate::algebra::abstr::Real;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Closed Newton-Cotes
///
/// <https://en.wikipedia.org/wiki/Newton-Cotes_formulas>
///
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct NewtonCotes<T>
    where T: Real
{
    weight: Vec<T>,
}


impl<T> NewtonCotes<T>
    where T: Real
{
    /// # Arguments
    ///
    /// n:
    /// 1 => Trapezoidal rule
    /// 2 => Simpson's rule
    /// 3 => Simpson's 3/8 rule
    /// 4 => Boole0s rule
    /// 5 =>
    ///
    /// # Panics
    ///
    /// Panics if n < 1 || n > 5
    ///
    /// # Examples
    /// ```
    /// # #[macro_use]
    /// # extern crate mathru;
    /// # fn main()
    /// # {
    /// use mathru::analysis::integral::newton_cotes::{NewtonCotes};
    ///
    /// let nc = NewtonCotes::new(1);
    /// let f = | x | {x};
    ///
    /// let integral = nc.integrate(f, 2.0, 4.0, 4);
    ///
    /// assert_relative_eq!(integral, 6.0)
    /// # }
    /// ```
    pub fn new(n: u8) -> NewtonCotes<T>
    {
        if !(1..=5).contains(&n)
        {
            panic!("'n' is out of bounds");
        }
        let weight= match n {
             1 => vec![T::from_f32(0.5), T::from_f32(0.5)],
             2 => vec![T::from_f64(1.0/6.0), T::from_f64(2.0/3.0), T::from_f64(1.0/6.0)],
             3 => vec![T::from_f64(1.0/8.0), T::from_f64(3.0/8.0), T::from_f64(3.0/8.0), T::from_f64(1.0/8.0)],
             4 => vec![T::from_f64(7.0/90.0), T::from_f64(16.0/45.0), T::from_f64(2.0/15.0), T::from_f64(16.0/45.0), T::from_f64(7.0/90.0)],
             5 => vec![T::from_f64(19.0/288.0), T::from_f64(25.0/96.0), T::from_f64(25.0/144.0), T::from_f64(25.0/144.0), T::from_f64(25.0/96.0), T::from_f64(19.0/288.0)],
             _ => panic!("")
        };
        NewtonCotes{
            weight
        }
    }

    ///
    /// ```math
    /// \int_{a}^{b}f(x)\,dx
    /// ```
    ///
    /// # Arguments
    /// * 'a'
    /// * 'b'
    ///  num_subintervals: \[a,b\] into smaller subintervals,
    /// #
    pub fn integrate<F>(&self, f: F, a: T, b: T, num_subintervals: u32) -> T
        where
            F: Fn(T) -> T,
    {
        let mut sub_interval:ClosedFixedIntervalIterator<T> = ClosedFixedIntervalIterator::new(a, b, num_subintervals);

        let mut lower_bound: T = sub_interval.next().unwrap();
        let num_intervals = self.weight.len() as u32 - 1;
        let mut sum = T::zero();
        sub_interval.for_each(|x_i: T| {
            let interval: ClosedFixedIntervalIterator<T> = ClosedFixedIntervalIterator::new(lower_bound, x_i, num_intervals);
            let h = x_i - lower_bound;
            let sum_i = interval.zip(self.weight.iter()).map(|(x, w)| {
                                     *w * f(x)
                                 })
                                    // .sum()
                                 .fold(
                                     T::zero(),
                                     |a, b| a + b
                                 ) * h;

            sum += sum_i;

            lower_bound = x_i;
        }
        );

        sum

    }

}
