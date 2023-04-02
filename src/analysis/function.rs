pub trait Function<Domain>
{
    type Codomain;
    fn eval(&self, input: &Domain) -> Self::Codomain;
}
