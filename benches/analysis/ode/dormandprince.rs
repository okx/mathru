use criterion::Criterion;
use mathru::algebra::linear::Vector;
use mathru::analysis::differential_equation::ordinary::{solver::runge_kutta::DormandPrince54, ExplicitODE, solver::runge_kutta::ProportionalControl};

criterion_group!(dormandprince, dormandprince_mathru, /*dormandprince_fast_ode*/);

pub struct ExplicitODE1
{
    time_span: (f64, f64),
    init_cond: Vector<f64>,
}

impl Default for ExplicitODE1
{
    fn default() -> ExplicitODE1
    {
        ExplicitODE1 { time_span: (0.0, 2.0),
            init_cond: Vector::new_column(vec![0.5; 200])//vector![0.5] 
            }
    }
}

impl ExplicitODE<f64> for ExplicitODE1
{
    fn func(&self, _t: &f64, x: &Vector<f64>) -> Vector<f64>
    {
        x * &2.0f64
    }

    fn time_span(&self) -> (f64, f64)
    {
        self.time_span
    }

    fn init_cond(&self) -> Vector<f64>
    {
        self.init_cond.clone()
    }
}

// impl fast_ode::DifferentialEquation<200> for ExplicitODE1 {
//     fn ode_dot_y(&self, _t: f64, y: &fast_ode::Coord<200>) -> (fast_ode::Coord<200>, bool) {
//         let mut x = y.clone();
//         x.0.iter_mut().for_each(|x_i| *x_i *= 2.0);
//         (x, true)
//     }
// }

pub fn dormandprince_mathru(bench: &mut Criterion)
{
    let problem: ExplicitODE1 = ExplicitODE1::default();

    let h_0: f64 = 0.001;
    let fac: f64 = 0.9;
    let fac_min: f64 = 0.01;
    let fac_max: f64 = 2.0;
    let n_max: u32 = 5000;
    let abs_tol: f64 = 10e-7;
    let rel_tol: f64 = 10e-3;

    let solver = ProportionalControl::new(n_max, h_0, fac, fac_min, fac_max, abs_tol, rel_tol);
    let method = DormandPrince54::default();

    bench.bench_function("DormandPrince Mathru", move |bh| {
        bh.iter(|| solver.solve(&problem, &method).unwrap())
    });
}

// fn dormandprince_fast_ode(bench: &mut Criterion)
// {
//     let ode: ExplicitODE1 = ExplicitODE1::default();

//     bench.bench_function("DormandPrince Fast ODE", move |bh| {
//         bh.iter(|| fast_ode::solve_ivp(&ode, (0., 2.), fast_ode::Coord([0.5; 200]), |_, _| true, 1e-6, 1e-3))
//     });
// }