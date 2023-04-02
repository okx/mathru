pub trait Test<T>
{
    ///Test value
    fn value(&self) -> T;

    /// Degree of freedom
    fn df(&self) -> u32;

    ///
    fn p_value(&self) -> T;
}
