use mathru::{
    algebra::linear::Vector,
    analysis::differential_equation::ordinary::{problem, solver::runge_kutta::ImplicitEuler},
};


#[test]
fn fn1()
{
    // Create an ODE instance
    let problem: problem::Euler<f64> = problem::Euler::default();
    let solver: ImplicitEuler<f64> = ImplicitEuler::new(0.0001);

    let (_x, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();

    assert_relative_eq!(0.9852, y.last().unwrap()[0], epsilon=0.0001);
}
