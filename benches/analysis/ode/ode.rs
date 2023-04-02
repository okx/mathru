use criterion::Criterion;

extern crate mathru;

use super::ode_problems::ExplicitODE1;
use mathru::analysis::differential_equation::ordinary::solver::runge_kutta::{
    FixedStepper, ProportionalControl,
    ExplicitEuler, Heun2, Kutta3, RungeKutta4, Fehlberg54,
};

criterion_group!(ode,
                 forward_euler,
                 heun,
                 kutta3,
                 rungekutta4,
                 fehlberg54,
                 );

fn forward_euler(bench: &mut Criterion)
{
    let problem: ExplicitODE1 = ExplicitODE1::default();
    let solver: FixedStepper<f64> = FixedStepper::new(0.001);
    let mut method = ExplicitEuler::default();

    bench.bench_function("Forward Euler", move |bh| {
             bh.iter(|| solver.solve(&problem, &mut method).unwrap())
         });
}

fn heun(bench: &mut Criterion)
{
    let problem: ExplicitODE1 = ExplicitODE1::default();
    let solver: FixedStepper<f64> = FixedStepper::new(0.001);
    let mut method = Heun2::default();

    bench.bench_function("Heun", move |bh| {
             bh.iter(|| solver.solve(&problem, &mut method).unwrap())
         });
}

fn kutta3(bench: &mut Criterion)
{
    let problem: ExplicitODE1 = ExplicitODE1::default();
    let solver: FixedStepper<f64> = FixedStepper::new(0.001);
    let mut method = Kutta3::default();

    bench.bench_function("Kutta3", move |bh| {
             bh.iter(|| solver.solve(&problem, &mut method).unwrap())
         });
}

fn rungekutta4(bench: &mut Criterion)
{
    let problem: ExplicitODE1 = ExplicitODE1::default();
    let solver: FixedStepper<f64> = FixedStepper::new(0.001);
    let mut method = RungeKutta4::default();

    bench.bench_function("Kutta4", move |bh| {
             bh.iter(|| solver.solve(&problem, &mut method).unwrap())
         });
}

fn fehlberg54(bench: &mut Criterion)
{
    let problem: ExplicitODE1 = ExplicitODE1::default();

    let h_0: f64 = 0.001;
    let fac: f64 = 0.9;
    let fac_min: f64 = 0.01;
    let fac_max: f64 = 2.0;
    let n_max: u32 = 5000;
    let abs_tol: f64 = 10e-7;
    let rel_tol: f64 = 10e-3;

    let solver: ProportionalControl<f64> = ProportionalControl::new(n_max, h_0, fac, fac_min, fac_max, abs_tol, rel_tol);
    let method = Fehlberg54::default();

    bench.bench_function("RungeKuttaFehlberg54", move |bh| {
             bh.iter(|| solver.solve(&problem, &method).unwrap())
         });
}

