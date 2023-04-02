use mathru::algebra::linear::{Vector, Matrix};
use mathru::analysis::NewtonRaphson;
use mathru::analysis::{Function, Jacobian};

/// $ f(x) = x^2 -2 $
struct Func1
{

}

impl Func1
{
	pub fn new() -> Func1
	{
		return Func1
			{

			};
	}
}

impl Function<Vector<f64>> for Func1
{
	type Codomain = Vector<f64>;

	fn eval(&self, input: &Vector<f64>) -> Vector<f64>
	{
		let x: f64 = input[0];
		return 	vector![x * x - 2.0];
	}
}

impl Jacobian<f64> for Func1
{
	fn jacobian(&self, input: &Vector<f64>) -> Matrix<f64>
	{
		let x: f64 = input[0];

		return matrix![2.0 * x];
	}
}

#[test]
fn find_root_new()
{
	let nr: NewtonRaphson<f64> = NewtonRaphson::new(100, 10e-6);
	let x_0: Vector<f64> = vector![1.0];
	let root: Vector<f64> = vector![2.0_f64.sqrt()];

	let problem: Func1 = Func1::new();

	let root_hat: Vector<f64> = nr.find_root(&problem, &x_0).unwrap();

	assert_relative_eq!(root, root_hat, epsilon=10e-6);
}


#[test]
fn find_root_default()
{
	let nr: NewtonRaphson<f64> = NewtonRaphson::default();
	let x_0: Vector<f64> = vector![1.0];
	let root: Vector<f64> = vector![2.0_f64.sqrt()];

	let problem: Func1 = Func1::new();

	let root_hat: Vector<f64> = nr.find_root(&problem, &x_0).unwrap();

	assert_relative_eq!(root, root_hat ,epsilon=10e-3);
}

