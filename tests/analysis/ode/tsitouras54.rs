use super::problem::ExplicitODE2;
use mathru::{
    algebra::linear::Vector,
    analysis::differential_equation::ordinary::solver::runge_kutta::{ExplicitRKEmbeddedMethod, Tsitouras54},
};


#[test]
fn test_1()
{
    let problem: ExplicitODE2 = ExplicitODE2::default();

    let rk = Tsitouras54::default();
    
    let x_0 = vector![1.2601588];

    let (x_1, x_1_s) = rk.tableau().do_step(&problem, &0.9f64, &x_0, &0.1f64);

    let x_1_ref = vector![1.5574064];

    assert_relative_eq!(x_1, x_1_ref, epsilon=0.000003);
}
