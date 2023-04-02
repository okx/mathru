
use mathru::{
    algebra::linear::Vector,
    analysis::differential_equation::ordinary::solver::runge_kutta::{Fehlberg21, ProportionalControl},
    analysis::differential_equation::ordinary::ExplicitODE
};


pub struct ExplicitODELocal
{
    time_span: (f64, f64),
    init_cond: Vector<f64>,
}

impl Default for ExplicitODELocal
{
    fn default() -> ExplicitODELocal
    {
        ExplicitODELocal { time_span: (0.0, 1.0),
                       init_cond: vector![0.5; 2.0] }
    }
}

impl ExplicitODE<f64> for ExplicitODELocal
{
    fn func(&self, _t: &f64, x: &Vector<f64>) -> Vector<f64>
    {
        return x * &2.0f64;
    }

    fn time_span(&self) -> (f64, f64)
    {
        return self.time_span;
    }

    fn init_cond(&self) -> Vector<f64>
    {
        return self.init_cond.clone();
    }
}


#[test]
fn test_proportional_control()
{
    let problem: ExplicitODELocal = ExplicitODELocal::default();

    let h_0: f64 = 0.2;
    let fac: f64 = 0.9;
    let fac_min: f64 = 0.01;
    let fac_max: f64 = 1.4;
    let n_max: u32 = 30;
    let abs_tol: f64 = 10e-6;
    let rel_tol: f64 = 10e-6;
    
    let solver: ProportionalControl<f64> = ProportionalControl::new(n_max, h_0, fac, fac_min, fac_max, abs_tol, rel_tol);
    let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem, &Fehlberg21::default()).unwrap();

    let len: usize = y.len();
    let time_span: (f64, f64) = problem.time_span();
    let init_cond: Vector<f64> = problem.init_cond();

    assert_relative_eq!(time_span.1, t[len - 1], epsilon=0.001);
    assert_relative_eq!(init_cond[0] * (2.0 * time_span.1).exp(), y[len - 1][0], epsilon=0.01);
    assert_relative_eq!(init_cond[1] * (2.0 * time_span.1).exp(), y[len - 1][1], epsilon=0.03);
}