use mathru::{
    algebra::{
        linear::{Vector},
    },
    optimization::{LevenbergMarquardt},
};
use crate::optimization::problem::QuadraticFunction;

#[test]
fn minimization()
{
    let problem: QuadraticFunction = QuadraticFunction::new();

    let lm: LevenbergMarquardt<f64> = LevenbergMarquardt::new(30, 0.3, 0.95);

    let x_0: Vector<f64> = vector![ 1.0;
                                    -2.1];

    let x_opt: Vector<f64> = lm.minimize(&problem, &x_0).unwrap().arg();

    let x_opt_ref: Vector<f64> = vector![   0.0;
                                            0.0];

    assert_abs_diff_eq!(x_opt_ref, x_opt, epsilon=0.002f64);
}