//use super::Semiring;

/// Natural number
///
///<https://en.wikipedia.org/wiki/Natural_number>
pub trait Natural: Eq
{
}

macro_rules! impl_natural
{
	($($t:ty),*) =>
    {
    	$(
        impl Natural for $t
        {

        }
        )*
    }
}

impl_natural!(u8, u16, u32, u64, u128);
