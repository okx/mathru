//! Some experiments why mathru::algebra::linear::Vector is so much slower than std::Vec
//!

use mathru::{algebra::linear::Vector, vector};
use criterion::Criterion;
use mathru::analysis::differential_equation::ordinary::{ExplicitEuler, ExplicitODE, FixedStepper};
use mathru::analysis::differential_equation::ordinary::explicit_method::ExplicitMethod;

criterion_group!(euler, bench_vector_euler, bench_vec_euler, bench_lib, bench_test);

fn vec_dgl(state: &Vec<f64>) -> Vec<f64>
{
    vec![state[0]]
}

pub fn vec_euler() -> f64 {
    let mut state = vec![1.];
    let dt: f64 = 0.01;
    for _ in 0..100000 {
        let diff = vec_dgl(&state);
        let x = state[0] + diff[0] * dt;
        state[0] = x;
        // state[0] += diff[0] * dt;
    }
    state[0]
}

fn vector_dgl(state: &Vector<f64>) -> Vector<f64>
{
    vector![state[0]]
}

pub fn vector_euler() -> f64
{
    let mut state = vector![1.];
    let dt: f64 = 0.01;
    for _ in 0..100000 {
        let diff = vector_dgl(&state);
        let x = state[0] + diff[0] * dt;
        state[0] = x;
        // state[0] += diff[0] * dt;
    }
    state[0]
}

fn bench_vec_euler(bench: &mut Criterion) {
    bench.bench_function("bench_vec_euler", move |bh| {
        bh.iter(vec_euler);
    });
}

fn bench_vector_euler(bench: &mut Criterion) {
    bench.bench_function("bench_vector_euler", move |bh| {
        bh.iter(vector_euler);
    });
}

pub struct LibDgl {
    time_span: (f64, f64),
    init_cond: Vector<f64>,
}

impl ExplicitODE<f64> for LibDgl {

    fn func(&self, t: f64, state: Vector<f64>) -> Vector<f64>
    {
        state
    }

    fn time_span(&self) -> (f64, f64) {
        self.time_span
    }

    fn init_cond(&self) -> Vector<f64> {
        self.init_cond.clone()
    }
}

fn bench_lib(bench: &mut Criterion) {
    let solver: FixedStepper<f64> = FixedStepper::new(0.001);
    let problem = LibDgl {
        time_span: (0.0, 100.),
        init_cond: vector![1.],
    };
    let mut method = ExplicitEuler::default();

    bench.bench_function("bench_lib", move |bh| {
        bh.iter(
            || {
                solver.solve(&problem, &mut method);
            }
        );
    });
}

fn vector_dgl_own(state: Vector<f64>) -> Vector<f64>
{
    state
}

fn solve() -> f64
{
    let mut x_n: Vector<f64> =  vector![1.0];
    let dt: f64 = 0.01;
    for _ in 0..100000
    {
        let diff = vector_dgl_own(x_n.clone());
        x_n[0] += diff[0] * dt;
        // x_n = vector_dgl(&x_n);
        // x_n[0] += x_n[0] * dt;
    }

    x_n[0]
}

fn bench_test(bench: &mut Criterion) {
    // let problem = LibDgl {
    //     time_span: (0.0, 100.),
    //     init_cond: vector![1.],
    // };

    bench.bench_function("bench_test", move |bh| {
        bh.iter(
            || {
                // vector_euler()
                solve() //&problem);
            }
        );
    });
}

