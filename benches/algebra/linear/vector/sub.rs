use mathru::{algebra::linear::Vector};
use criterion::Criterion;

criterion_group!(vector_sub, bench_vector_sub_assign_scalar, bench_vector_sub_scalar, bench_vec_sub_scalar, bench_vector_sub_vector, bench_vec_sub_vec);

fn bench_vector_sub_assign_scalar(bench: &mut Criterion)
{
    bench.bench_function("bench_vector_sub_assign_scalar", move |bh| {
        bh.iter(vector_sub_assign_scalar);
    });
}

fn vector_sub_assign_scalar()
{
    let mut vec: Vector<f64> = Vector::new_column(vec![3.0; 100000]);

    vec -= 3.0f64;
}

fn bench_vector_sub_scalar(bench: &mut Criterion)
{
    bench.bench_function("bench_vector_sub_scalar", move |bh| {
        bh.iter(vector_sub_scalar);
    });
}

fn vector_sub_scalar()
{
    let mut vec: Vector<f64> = Vector::new_column(vec![3.0; 100000]);

    let _ = (&mut vec) - &3.0f64;
}

fn bench_vec_sub_scalar(bench: &mut Criterion)
{
    bench.bench_function("bench_vec_sub_scalar", move |bh| {
        bh.iter(vec_sub_scalar);
    });
}

fn vec_sub_scalar()
{
    let mut vec: Vec<f64> = vec![3.0; 100000];

    vec.iter_mut().for_each(|a: &mut f64| *a -= 3.0);
}

fn bench_vector_sub_vector(bench: &mut Criterion)
{
    bench.bench_function("bench_vector_sub_vector", move |bh| {
        bh.iter(vector_sub_vector);
    });
}

fn vector_sub_vector()
{
    let mut vec1: Vector<f64> = Vector::new_column(vec![3.0; 100000]);
    let vec2: Vector<f64> = Vector::new_column(vec![3.0; 100000]);

    let _: &mut Vector<f64> = &mut vec1 - &vec2;
}

fn bench_vec_sub_vec(bench: &mut Criterion)
{
    bench.bench_function("bench_vec_sub_vec", move |bh| {
        bh.iter(vec_sub_vec);
    });
}

fn vec_sub_vec()
{
    let mut vec1: Vec<f64> = vec![3.0; 100000];
    let vec2: Vec<f64> = vec![3.0; 100000];

    vec1.iter_mut().zip(&vec2).for_each(|(a, b)| *a -= *b);
}