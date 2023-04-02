use crate::algebra::abstr::Ring;

/// Integer
///
///<https://en.wikipedia.org/wiki/Integer>
pub trait Integer: Ring + Eq
{
}

macro_rules! impl_integer
{
    ($($t:ty),*) =>
    {
    	$(
        impl Integer for $t
        {

        }
        )*
    }
}

impl_integer!(i8, i16, i32, i64, i128);
