//! Operator Trait

///
pub trait Operator: Copy
{
    fn operator() -> Self;
}

/// The addition operator, commonly symbolized by $+$.
#[derive(Copy, Clone)]
pub struct Addition;

impl Operator for Addition
{
    fn operator() -> Self
    {
        Addition
    }
}

/// The multiplication operator, commonly symbolized by $*$.
#[derive(Copy, Clone)]
pub struct Multiplication;

impl Operator for Multiplication
{
    fn operator() -> Self
    {
        Multiplication
    }
}
