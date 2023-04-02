use mathru::{algebra::linear::Vector};
use criterion::Criterion;

criterion_group!(vector_add_assign, bench_add_assign_vector, bench_add_assign_scalar,);

fn bench_add_assign_vector(bench: &mut Criterion)
{
    bench.bench_function("vector add_assign vector", move |bh| {
        bh.iter(add_assign_vector);
    });
}

fn add_assign_vector()
{
    let mut vec: Vector<f64> = Vector::new_column(vec![3.0; 100000]);
    let b: Vector<f64> = Vector::new_column(vec![3.0; 100000]);
    vec += b;
}

fn bench_add_assign_scalar(bench: &mut Criterion)
{
    bench.bench_function("vector add_assign scalar", move |bh| {
        bh.iter(add_assign_scalar);
    });
}

fn add_assign_scalar()
{
    let mut vec: Vector<f64> = Vector::new_column(vec![3.0; 100000]);

    vec += 3.0f64;
}


