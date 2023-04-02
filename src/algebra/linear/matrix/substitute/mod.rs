#[cfg(feature = "lapack")]
pub mod lapack;
#[cfg(feature = "native")]
pub mod native;

pub trait Substitute<T>
{
    fn substitute_forward(&self, b: T) -> Result<T, ()>;

    fn substitute_backward(&self, b: T) -> Result<T, ()>;
}
