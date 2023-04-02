use mathru::{algebra::linear::Vector};
use criterion::Criterion;

criterion_group!(vector_sub_assign, bench_sub_assign_vector, bench_sub_assign_scalar,);

fn bench_sub_assign_vector(bench: &mut Criterion)
{
    bench.bench_function("vector sub_assign vector", move |bh| {
        bh.iter(sub_assign_vector);
    });
}

fn sub_assign_vector()
{
    let mut vec: Vector<f64> = Vector::new_column(vec![3.0; 100000]);
    let b: Vector<f64> = Vector::new_column(vec![3.0; 100000]);
    vec -= b;
}

fn bench_sub_assign_scalar(bench: &mut Criterion)
{
    bench.bench_function("vector sub_assign scalar", move |bh| {
        bh.iter(sub_assign_scalar);
    });
}

fn sub_assign_scalar()
{
    let mut vec: Vector<f64> = Vector::new_column(vec![3.0; 100000]);

    vec -= 3.0f64;
}