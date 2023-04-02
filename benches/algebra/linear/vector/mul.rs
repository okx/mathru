use mathru::{algebra::linear::Vector};
use criterion::Criterion;

criterion_group!(vector_mul, bench_vector_mul_scalar, bench_vec_mul_scalar);

fn bench_vector_mul_scalar(bench: &mut Criterion)
{
    bench.bench_function("bench_vector_mul_scalar", move |bh| {
        bh.iter(vector_mul_scalar);
    });
}

fn vector_mul_scalar()
{
    let mut vec: Vector<f64> = Vector::new_column(vec![3.0; 100000]);
    let _: &mut Vector<f64> = &mut vec * &3.0f64;
}

fn bench_vec_mul_scalar(bench: &mut Criterion)
{
    bench.bench_function("bench_vec_mul_scalar", move |bh| {
        bh.iter(vec_mul_scalar);
    });
}

fn vec_mul_scalar()
{
    let mut vec: Vec<f64> = vec![3.0; 100000];
    vec.iter_mut().for_each(|a: &mut f64| *a *= 3.0);
}