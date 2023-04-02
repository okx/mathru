use criterion::Criterion;
use mathru::algebra::linear::Matrix;

criterion_group!(mul, bench_mul_matrix_own, bench_mul_matrix_borrow, bench_mul_matrix_mut_borrow, bench_mul_scalar_own, bench_mul_scalar_mut_borrow, bench_mul_scalar_borrow /*, bench_vector_mul_vector, bench_vec_mul_vec*/);

fn bench_mul_matrix_own(bench: &mut Criterion)
{
    bench.bench_function("mul matrix own", move |bh| {
        bh.iter(||
            {
                let a: Matrix<f64> = Matrix::new(100, 100, vec![3.0; 10000]);
                let b: Matrix<f64> = Matrix::new(100, 100, vec![3.0; 10000]);
                let _ = a * b;
            }
        );
    });
}

fn bench_mul_matrix_borrow(bench: &mut Criterion)
{
    let a: Matrix<f64> = Matrix::new(100, 100, vec![3.0; 10000]);
    let b: Matrix<f64> = Matrix::new(100, 100, vec![3.0; 10000]);

    bench.bench_function("mul matrix borrow", move |bh| {
        bh.iter(||
            {
                let _ = &a * &b;
            }
        );
    });
}

fn bench_mul_matrix_mut_borrow(bench: &mut Criterion)
{
    let mut a: Matrix<f64> = Matrix::new(100, 100, vec![3.0; 10000]);
    let b: Matrix<f64> = Matrix::new(100, 100, vec![3.0; 10000]);

    bench.bench_function("mul matrix mut borrow", move |bh| {
        bh.iter(||
            {
                let _ = &mut a * &b;
            }
        );
    });
}

fn bench_mul_scalar_own(bench: &mut Criterion)
{
    bench.bench_function("mul scalar own", move |bh| {
        bh.iter(||
            {
                let matrix: Matrix<f64> = Matrix::new(100, 100, vec![3.0; 10000]);
                let _ = matrix * 3.0f64;
            }
        );
    });
}

fn bench_mul_scalar_borrow(bench: &mut Criterion)
{
    let matrix: Matrix<f64> = Matrix::new(100, 100, vec![3.0; 10000]);
    bench.bench_function("mul scalar borrow", move |bh| {
        bh.iter(||
            {
                let _ = &matrix * &3.0f64;
            }
        );
    });
}

fn bench_mul_scalar_mut_borrow(bench: &mut Criterion)
{
    let mut matrix: Matrix<f64> = Matrix::new(100, 100, vec![3.0; 10000]);
    bench.bench_function("mul scalar mut borrow", move |bh| {
        bh.iter(|| {let _ = (&mut matrix) * &3.0f64;});
    });
}