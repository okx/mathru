use mathru::{
    algebra::linear::{Vector},
    optimization::{Newton},
};
use crate::optimization::problem::Rosenbrock;


#[test]
fn test_minimization()
{
    let rosenbrock: Rosenbrock = Rosenbrock::new();

    let optimizer: Newton<f64> = Newton::new(15, 0.1, 0.00001);
    let x_0: Vector<f64> = vector![0.0; -0.1];
    let x_opt: Vector<f64> = optimizer.minimize(&rosenbrock, &x_0).arg();

    let x_opt_ref: Vector<f64> = vector![1.0; 1.0];

    assert_eq!(x_opt_ref, x_opt);
}
