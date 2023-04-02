use criterion::Criterion;
use mathru::{algebra::linear::Vector, vector};
use mathru::algebra::abstr::cast::FromPrimitive;

criterion_group!(vector_one1, bench_one, bench_one_1, bench_one_2, bench_one_3, bench_one_4);


fn bench_one(bench: &mut Criterion)
{
    bench.bench_function("bench_vector_one", move |bh| {
        bh.iter(vector_one::<f64>);
    });
}

fn vector_one<T>()
    where T: FromPrimitive + Copy
{
    let mut v: Vec<T> = Vec::<T>::with_capacity(1000000);
    for i in 0..1000000
    {
        v.push(T::from_i32(i));
    }
}

fn bench_one_1(bench: &mut Criterion)
{
    bench.bench_function("bench_vector_one_1", move |bh| {
        bh.iter(vector_one_1);
    });
}

fn vector_one_1()
{
    let mut v: Vec<f64> = Vec::<f64>::with_capacity(1000000);
    for i in 0..1000000
    {
        v.push(i as f64)
    }
}

fn bench_one_2(bench: &mut Criterion)
{
    bench.bench_function("bench_vector_one_2", move |bh| {
        bh.iter(vector_one_2);
    });
}

fn vector_one_2()
{
    let mut v: Vec<Vector<f64>> = Vec::<Vector<f64>>::with_capacity(1000000);
    for i in 0..1000000
    {
        v.push(vector![i as f64])
    }
}

fn bench_one_3(bench: &mut Criterion)
{
    bench.bench_function("bench_vector_one_3", move |bh| {
        bh.iter(vector_one_3::<f64>);
    });
}

fn vector_one_3<T>()
    where T: FromPrimitive + Copy
{
    let mut v: Vec<Vector<T>> = Vec::<Vector<T>>::with_capacity(1000000);
    for i in 0..1000000
    {
        v.push(vector![T::from_i32(i)])
    }
}

fn bench_one_4(bench: &mut Criterion)
{
    bench.bench_function("bench_vector_one_4", move |bh| {
        bh.iter(vector_one_4::<f64>);
    });
}

fn vector_one_4<T>()
    where T: FromPrimitive + Copy
{
    let mut v: Vec<Vec<f64>> = Vec::<Vec<f64>>::with_capacity(1000000);
    for i in 0..1000000
    {
        v.push(vec![i as f64])
    }
}