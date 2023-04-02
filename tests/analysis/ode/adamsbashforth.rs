use mathru::{
    algebra::linear::Vector,
    analysis::differential_equation::ordinary::{ExplicitODE, solver::AdamsBashforth},
};

use super::problem::{ExplicitODE1, ExplicitODE2};



#[test]
fn fn1_1_step()
{
    let problem: ExplicitODE1 = ExplicitODE1::default();
    let solver: AdamsBashforth<f64> = AdamsBashforth::new(1, 0.001);

    let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();

    let len: usize = y.len();

    let time_span: (f64, f64) = problem.time_span();
    let init_cond: Vector<f64> = problem.init_cond();

    assert_relative_eq!(time_span.1, t[len - 1], epsilon=0.000000001);
    assert_relative_eq!(init_cond[0] * (2.0 * time_span.1).exp(), y[len - 1][0], epsilon=0.2);
}

#[test]
fn fn1_2_steps()
{
    let problem: ExplicitODE1 = ExplicitODE1::default();
    let solver: AdamsBashforth<f64> = AdamsBashforth::new(2, 0.001);

    let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();

    let len: usize = y.len();

    let time_span: (f64, f64) = problem.time_span();
    let init_cond: Vector<f64> = problem.init_cond();

    assert_relative_eq!(time_span.1, t[len - 1], epsilon=0.000000001);
    assert_relative_eq!(init_cond[0] * (2.0 * time_span.1).exp(), y[len - 1][0], epsilon=0.0003);
}

#[test]
fn fn1_3_steps()
{
    let problem: ExplicitODE1 = ExplicitODE1::default();
    let solver: AdamsBashforth<f64> = AdamsBashforth::new(3, 0.001);

    let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();

    let len: usize = y.len();

    let time_span: (f64, f64) = problem.time_span();
    let init_cond: Vector<f64> = problem.init_cond();

    assert_relative_eq!(time_span.1, t[len - 1], epsilon=0.000000001);
    assert_relative_eq!(init_cond[0] * (2.0 * time_span.1).exp(), y[len - 1][0], epsilon=0.00006);
}

#[test]
fn fn1_4_steps()
{
    let problem: ExplicitODE1 = ExplicitODE1::default();
    let solver: AdamsBashforth<f64> = AdamsBashforth::new(4, 0.001);

    let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();

    let len: usize = y.len();

    let time_span: (f64, f64) = problem.time_span();
    let init_cond: Vector<f64> = problem.init_cond();

    assert_relative_eq!(time_span.1, t[len - 1], epsilon=0.000000001);
    assert_relative_eq!(init_cond[0] * (2.0 * time_span.1).exp(),
                            y[len - 1][0],
                            epsilon=0.000055);
}

#[test]
fn fn1_5_steps()
{
    let problem: ExplicitODE1 = ExplicitODE1::default();
    let solver: AdamsBashforth<f64> = AdamsBashforth::new(5, 0.001);

    let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();

    let len: usize = y.len();

    let time_span: (f64, f64) = problem.time_span();
    let init_cond: Vector<f64> = problem.init_cond();

    assert_relative_eq!(time_span.1, t[len - 1], epsilon=0.000000001);
    assert_relative_eq!(init_cond[0] * (2.0 * time_span.1).exp(), y[len - 1][0], epsilon=0.000055);
}

//	#[test]
//	fn fn3_k5()
//	{
//		let problem: ExplicitODE3 = ExplicitODE3::default();
//		let solver: AdamsBashforth<f64> = AdamsBashforth::new(5, 0.00001);
//
//		let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();
//
//		let len: usize = y.len();
//
//		let time_span: (f64, f64) = problem.time_span();
//
//		assert_relative_eq!(time_span.1, t[len - 1], 0.000000001));
//		assert_relative_eq!(1.0 / (2.0 - 1.8) - *y[len -1][0), 1.89756,
// 0.00006)); 	}

#[test]
fn fn2_1_step()
{
    let problem: ExplicitODE2 = ExplicitODE2::default();
    let solver: AdamsBashforth<f64> = AdamsBashforth::new(1, 0.001);

    let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();

    let len: usize = y.len();

    let time_span: (f64, f64) = problem.time_span();

    assert_relative_eq!(time_span.1, t[len - 1], epsilon=0.00000001);
    assert_relative_eq!(time_span.1.tan(), y[len - 1][0], epsilon=0.07);
}
