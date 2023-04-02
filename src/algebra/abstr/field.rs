/// Field
use crate::algebra::abstr::{AbelianGroupMul, CommutativeRing};
use std::{f32, f64};

/// Field
///
/// A field is a commutative ring, and an Abelian group under both operators.
pub trait Field: CommutativeRing + AbelianGroupMul
{
}

macro_rules! impl_field
{
    ($($t:ty),*) =>
    {
        $(
        impl Field for $t
        {

        }
        )*
    };
}

impl_field!(f32, f64);
