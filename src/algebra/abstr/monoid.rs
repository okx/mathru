//! Monoid
use super::{
    identity::Identity,
    operator::{Addition, Multiplication, Operator},
    semigroup::{Semigroup, SemigroupAdd, SemigroupMul},
};

/// A Monoid is a triple $(\mathbb{M}, \circ, e)$, composed by a set
/// $\mathbb{M}$ and a binary inner operation $\circ$ and the element $e
/// \in \mathbb{M}$
///
/// ```math
/// \circ: \mathbb{M} \times \mathbb{M} \rightarrow \mathbb{M} , (x, y) \mapsto x \circ y
/// ```
/// 1. associativity <br>
/// $\forall x, y, z \in \mathbb{M}$: $x \circ (y \circ z) = (x \circ y)
/// \circ z$ 2. $e$ neutral element <br>
/// $\forall x \in \mathbb{M}$: $x \circ e = e \circ x = x$
pub trait Monoid<O: Operator>: Semigroup<O> + Identity<O>
{
}

macro_rules! impl_monoid
(
    ($O:ty; $op: ident; $($T:ty),*) =>
    {
        $(
            impl Monoid<$O> for $T
            {
            }
        )*
    }
);

impl_monoid!(Addition; add; u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);
impl_monoid!(Multiplication; mul; u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

pub trait MonoidAdd: Monoid<Addition> + SemigroupAdd + Zero
{
}

macro_rules! impl_monoidadd
(
    ($($T:ty),*) =>
    {
        $(
            impl MonoidAdd for $T
            {

            }
        )*
    }
);

impl_monoidadd!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

pub trait MonoidMul: Monoid<Multiplication> + SemigroupMul + One
{
}

macro_rules! impl_monoidmul
(
    ($($T:ty),*) =>
    {
        $(
            impl MonoidMul for $T
            {

            }
        )*
    }
);

impl_monoidmul!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

pub trait One
{
    fn one() -> Self;
}

macro_rules! impl_one
{
    ($v:expr; $($t:ty),+) =>
    {
    	$(
        impl One for $t
        {
            fn one() -> Self
            {
                return $v;
            }
        }
        )*
    };
}

impl_one!(1; u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
impl_one!(1.0; f32, f64);

pub trait Zero
{
    fn zero() -> Self;
}

macro_rules! impl_zero
{
    ($v:expr; $($t:ty),*) =>
    {
    	$(
        impl Zero for $t
        {
            fn zero() -> Self
            {
                return $v;
            }
        }
        )*
    };
}

impl_zero!(0; u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
impl_zero!(0.0; f32, f64);
