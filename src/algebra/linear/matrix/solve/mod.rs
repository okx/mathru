#[cfg(feature = "lapack")]
pub mod lapack;
#[cfg(feature = "native")]
pub mod native;

pub trait Solve<T>
{
    /// A * x = b
    fn solve(&self, rhs: &T) -> Result<T, ()>;
}
