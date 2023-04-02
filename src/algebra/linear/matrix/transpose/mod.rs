pub mod native;

pub trait Transpose
{
    type Output;
    fn transpose(self) -> Self::Output;
}

