use std::ops::Neg;

/// Sign trait
pub trait Sign: Sized + Neg<Output = Self>
{
    /// Returns the sign of a number
    ///
    /// # Param
    ///
    /// # Return
    ///
    /// -1 if self < 0
    /// 0 if self = 0
    /// 1 if self > 0
    fn sign(&self) -> Self;

    fn abs(&self) -> Self;

    fn is_positive(&self) -> bool;

    fn is_negative(&self) -> bool;
}

macro_rules! impl_sign
{
    ($neg:expr; $zero:expr; $pos:expr; $($t:ty),*) =>
    {
        $(
        impl Sign for $t
        {
			fn sign(&self) -> Self
			{
 				match *self
 				{
                    n if n > $zero => $pos,
                    n if n >= $zero && n <= $zero => $zero, //float comparing is not allowed anymore
                    _ => $neg,
                }
			}

			fn abs(&self) -> Self
			{
				if self.is_negative()
				{
					return -*self;
				}
				*self
			}

            fn is_positive(&self) -> bool
            {
            	*self > $zero
            }

            fn is_negative(&self) -> bool
            {
            	*self < $zero
            }
        }
        )*
    };
}

impl_sign!(-1; 0; 1; i8, i16, i32, i64, i128);

impl_sign!(-1.0; 0.0; 1.0; f32, f64);
