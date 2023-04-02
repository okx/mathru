pub struct OptimResult<T>
{
    arg: T,
}

impl<T> OptimResult<T>
{
    pub fn new(arg: T) -> OptimResult<T>
    {
        OptimResult { arg }
    }

    pub fn arg(self) -> T
    {
        self.arg
    }
}
