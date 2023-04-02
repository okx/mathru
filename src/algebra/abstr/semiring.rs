//! Semiring
use std::ops::{Add, Mul};
use std::marker::Sized;
use super::{
   monoid::{Monoid},
   operator::{Addition, Multiplication, Operator},
};

/// Semiring
///
///<https://en.wikipedia.org/wiki/Semiring>
pub trait Semiring<A: Operator = Addition, M: Operator = Multiplication>: Monoid<A> + Monoid<M>
{

}

// Defines an additive identity element for `Self`.
pub trait Zero: Sized + Add<Self, Output = Self>
{
   /// Returns the additive identity element of `Self`, `0`.
   ///
   /// # Laws
   ///
   /// ```{.text}
   /// a + 0 = a       ∀ a ∈ Self
   /// 0 + a = a       ∀ a ∈ Self
   /// ```
   fn zero() -> Self;
}

// Defines a multiplicative identity element for `Self`.
pub trait One: Sized + Mul<Self, Output = Self> {
   /// Returns the multiplicative identity element of `Self`, `1`.
   ///
   /// # Laws
   ///
   /// ```{.text}
   /// a * 1 = a       ∀ a ∈ Self
   /// 1 * a = a       ∀ a ∈ Self
   /// ```
   fn one() -> Self;
}


macro_rules! impl_semiring
{
    ($($t:ty),*) =>
    {
        $(
        impl Semiring for $t
        {

        }
        )*
    };
}

impl_semiring!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);
