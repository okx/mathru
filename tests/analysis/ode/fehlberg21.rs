use super::problem::ExplicitODE2;
use mathru::{
    algebra::linear::Vector,
    analysis::differential_equation::ordinary::solver::runge_kutta::{Fehlberg21, ExplicitRKEmbeddedMethod},
    elementary::Trigonometry
};

#[test]
fn test_1()
{
    let problem: ExplicitODE2 = ExplicitODE2::default();

    let rk = Fehlberg21::default();
    let t_0 = 0.9;
    let h = 0.1;
    let t_1 = t_0 + h;
    
    let x_0 = vector![t_0.tan()];

    let (x_1, x_1_s) = rk.tableau().do_step(&problem, &t_0, &x_0, &h);

    let x_1_ref = vector![t_1.tan()];

    assert_relative_eq!(x_1, x_1_ref, epsilon=0.0042);
    assert_relative_eq!(x_1_s, x_1_ref, epsilon=0.0043);
}
