use mathru::{
    algebra::{
        linear::{Vector},
    },
    optimization::{ConjugateGradient},
};
use crate::optimization::problem::LinearEquation;


#[test]
fn linear_equation_minimization()
{
    let optim: ConjugateGradient<f64> = ConjugateGradient::new(10, 0.01);
    let leq: LinearEquation = LinearEquation::new();

    let x_0: Vector<f64> = vector![1.0; 1.0];

    let x_min: Vector<f64> = optim.minimize(&leq, &x_0).arg();

    let x_opt: Vector<f64> = vector![14.0; -7.0];

    assert_relative_eq!(x_opt, x_min, epsilon=0.0001);
}

