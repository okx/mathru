//! Loop

use super::{
    identity::Identity,
    operator::{Addition, Multiplication, Operator},
    quasigroup::Quasigroup,
};

/// A quasigroup with an unique identity element.
///
/// $\exists e \in \mathbb{Q}, \forall a \in \mathbb{Q}, \exists r, l \in
/// \mathbb{Q}$ such that $l ∘ a = a ∘ r = e $ The left inverse $r$ and
/// right inverse $l$ are not required to be equal. The following property is
/// added to the quasigroup structure:
///
/// This property follows from
///
/// $\forall a \in \mathbb{Q}, \exists e \in \mathbb{Q}$, such that $e ∘ a =
/// a ∘ e = a$.
pub trait Loop<O: Operator>: Quasigroup<O> + Identity<O>
{
}

macro_rules! impl_loop(
    ($T:ty, $($S:ty),*) =>
    {
        $(
        impl Loop<$T> for $S
        {
        }
        )*
    }
);

impl_loop!(Addition, i8, i16, i32, i64, i128, f32, f64);
impl_loop!(Multiplication, f32, f64);
