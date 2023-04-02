use super::{Field, Scalar};
use crate::elementary::{Exponential, Hyperbolic, Power, Trigonometry};
use crate::algebra::abstr::{AbsDiffEq, RelativeEq};

macro_rules! impl_real
{
    ($($t:ty, $id:ident);*) =>
    {
    	$(
        impl Real for $t
        {

			fn ceil(&self) -> Self
			{
				(*self).ceil()
			}

			fn floor(&self) -> Self
			{
				(*self).floor()
			}

			fn euler_gamma() -> Self
			{
                0.577_215_664_901_532_9
			}

			fn infinity() -> Self
			{
			    return Self::INFINITY
			}

			fn neg_infinity() -> Self
			{
			    return Self::NEG_INFINITY
			}
        }
        )*
    }
}

impl_real!(f32, f32; f64, f64);

/// Real number
///
///<https://en.wikipedia.org/wiki/Real_number>
pub trait Real: Field + Scalar + Exponential + Trigonometry + Power + Hyperbolic + AbsDiffEq<Epsilon = Self> + RelativeEq
{
    /// Returns the smallest integer greater than or equal to a number.
    fn ceil(&self) -> Self;

    /// Returns the largest integer less than or equal to a number.
    fn floor(&self) -> Self;

    fn min(self, a: Self) -> Self
    {
        if self <= a
        {
            self
        }
        else
        {
            a
        }
    }

    fn max(self, a: Self) -> Self
    {
        if self >= a
        {
            self
        }
        else
        {
            a
        }
    }

    /// Euler–Mascheroni constant
    fn euler_gamma() -> Self;

    fn infinity() -> Self;

    fn neg_infinity() -> Self;
}
