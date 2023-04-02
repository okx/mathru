use mathru::analysis::{NewtonRaphson, Jacobian, Function};
use mathru::{algebra::linear::{Vector, Matrix}, vector};


/// Square root calculation with the Newton-Raphson method
///
/// ```math
/// y = x^{2}
/// ```
/// We define a new function which has a root
/// ```math
/// f(x) = x^{2} -y
/// ```
///
/// ```math
/// \frac{df(x)}{dx} = f'(x) = 2x
/// ```
///
fn main()
{
    let x: f64 = Sqrt::sqrt(17.0);

    println!("Sqrt: {:?}", x);
}


struct Sqrt
{
    y: f64
}

impl Sqrt
{
    ///
    ///
    ///
    pub fn sqrt(y: f64) -> f64
    {
        let newton: NewtonRaphson<f64> = NewtonRaphson::default();

        let sqrt: Sqrt = Sqrt{y};

        let x: f64 = newton.find_root(&sqrt, &vector![y]).unwrap()[0];
        return x;
    }
}

impl Jacobian<f64> for Sqrt
{
    fn jacobian(&self, x: &Vector<f64>) -> Matrix<f64>
    {
        return Matrix::from(x.clone().transpose()) * 2.0;
    }
}

impl Function<Vector<f64>> for Sqrt
{
    type Codomain = Vector<f64>;
    fn eval(&self, x: &Vector<f64>) -> Self::Codomain
    {
        return x.clone().apply(&|x: &f64|-> f64 { return x * x - self.y});
    }
}
