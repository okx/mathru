use criterion::Criterion;
use mathru::algebra::linear::Matrix;

criterion_group!(add, bench_add_matrix_own, bench_add_matrix_borrow, bench_add_matrix_mut_borrow, bench_add_scalar_own, bench_add_scalar_mut_borrow, bench_add_scalar_borrow /*, bench_vector_add_vector, bench_vec_add_vec*/);

fn bench_add_matrix_own(bench: &mut Criterion)
{
    bench.bench_function("add matrix own", move |bh| {
        bh.iter(||
            {
                let a: Matrix<f64> = Matrix::new(1000, 1000, vec![3.0; 1000000]);
                let b: Matrix<f64> = Matrix::new(1000, 1000, vec![3.0; 1000000]);
                let _ = a + b;
            }
        );
    });
}

fn bench_add_matrix_borrow(bench: &mut Criterion)
{
    let a: Matrix<f64> = Matrix::new(1000, 1000, vec![3.0; 1000000]);
    let b: Matrix<f64> = Matrix::new(1000, 1000, vec![3.0; 1000000]);

    bench.bench_function("add matrix borrow", move |bh| {
        bh.iter(||
            {
                let _ = &a + &b;
            }
        );
    });
}

fn bench_add_matrix_mut_borrow(bench: &mut Criterion)
{
    let mut a: Matrix<f64> = Matrix::new(1000, 1000, vec![3.0; 1000000]);
    let b: Matrix<f64> = Matrix::new(1000, 1000, vec![3.0; 1000000]);

    bench.bench_function("add matrix mut borrow", move |bh| {
        bh.iter(||
            {
                let _ = &mut a + &b;
            }
        );
    });
}

fn bench_add_scalar_own(bench: &mut Criterion)
{
    bench.bench_function("add scalar own", move |bh| {
        bh.iter(||
        {
            let matrix: Matrix<f64> = Matrix::new(1000, 1000, vec![3.0; 1000000]);
            let _ = matrix + 3.0f64;
        }
        );
    });
}

fn bench_add_scalar_borrow(bench: &mut Criterion)
{
    let matrix: Matrix<f64> = Matrix::new(1000, 1000, vec![3.0; 1000000]);
    bench.bench_function("add scalar borrow", move |bh| {
        bh.iter(||
            {
                let _ = &matrix + &3.0f64;
            }
        );
    });
}

fn bench_add_scalar_mut_borrow(bench: &mut Criterion)
{
    let mut matrix: Matrix<f64> = Matrix::new(1000, 1000, vec![3.0; 1000000]);
    bench.bench_function("add scalar mut borrow", move |bh| {
        bh.iter(|| {let _ = (&mut matrix) + &3.0f64;});
    });
}