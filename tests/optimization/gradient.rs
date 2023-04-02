use mathru::{
    algebra::linear::{Vector},
    optimization::{Gradient},
};
use crate::optimization::problem::{Rosenbrock, QuadraticFunction};


#[test]
fn minimization_quadratic()
{
    let optim: Gradient<f64> = Gradient::new(0.1, 100);
    let function: QuadraticFunction = QuadraticFunction::new();

    let x_0: Vector<f64> = Vector::new_column(vec![1.0, -1.0]);

    let x_min: Vector<f64> = optim.minimize(&function, &x_0).arg();

    assert_abs_diff_eq!(x_min[0], 0.0f64, epsilon=0.05f64);
    assert_abs_diff_eq!(x_min[1], 0.0f64, epsilon=0.05f64);
}

#[test]
fn minimization_rosenbrock()
{
    let rosenbrock: Rosenbrock = Rosenbrock::new();

    let optim: Gradient<f64> = Gradient::new(0.1, 1500);
    let x_0: Vector<f64> = vector![-2.0; -1.0];
    let x_opt: Vector<f64> = optim.minimize(&rosenbrock, &x_0).arg();

    let x_opt_ref: Vector<f64> = vector![1.0; 1.0];

    assert_relative_eq!(x_opt_ref, x_opt, epsilon=0.1f64);
}

