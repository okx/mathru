use criterion::{Criterion};

extern crate mathru;
use mathru::algebra::linear::Matrix;

criterion_group!(matrix,
                 mat1000_add_mat1000,
                 mat1000_mul_mat1000,
                 mat200_add_mat200,
                 mat500_add_mat500,
                 mat100_mul_mat100,
                 mat200_mul_mat200,
                 mat500_mul_mat500,
                 mat500_add_scalar_borrow,
                 mat500_add_scalar_ownership
                 );

fn mat1000_add_mat1000(bench: &mut Criterion)
{
    let a: Matrix<f64> = Matrix::new_random(1000, 1000);
    let b: Matrix<f64> = Matrix::new_random(1000, 1000);

    bench.bench_function("mat1000_add_mat1000", move |bh| bh.iter(|| &a + &b ));
}

fn mat1000_mul_mat1000(bench: &mut Criterion)
{
    let a: Matrix<f64> = Matrix::new_random(1000, 1000);
    let b: Matrix<f64> = Matrix::new_random(1000, 1000);

    bench.bench_function("mat1000_mul_mat1000", move |bh| bh.iter(|| &a * &b ));
}

fn mat200_add_mat200(bench: &mut Criterion)
{
    let a: Matrix<f64> = Matrix::new_random(200, 200);
    let b: Matrix<f64> = Matrix::new_random(200, 200);

    bench.bench_function("mat200_add_mat200", move |bh| bh.iter(|| &a + &b ));
}

fn mat500_add_mat500(bench: &mut Criterion)
{
    let a: Matrix<f64> = Matrix::new_random(500, 500);
    let b: Matrix<f64> = Matrix::new_random(500, 500);

    bench.bench_function("mat500_add_mat500", move |bh| bh.iter(|| &a + &b ));
}

fn mat100_mul_mat100(bench: &mut Criterion)
{
    let a: Matrix<f64> = Matrix::new_random(100, 100);
    let b: Matrix<f64> = Matrix::new_random(100, 100);

    bench.bench_function("mat100_mul_mat100", move |bh| bh.iter(|| &a * &b ));
}

fn mat200_mul_mat200(bench: &mut Criterion)
{
    let a: Matrix<f64> = Matrix::new_random(200, 200);
    let b: Matrix<f64> = Matrix::new_random(200, 200);

    bench.bench_function("mat200_mul_mat200", move |bh| bh.iter(|| &a * &b ));
}

fn mat500_mul_mat500(bench: &mut Criterion)
{
    let a: Matrix<f64> = Matrix::new_random(500, 500);
    let b: Matrix<f64> = Matrix::new_random(500, 500);

    bench.bench_function("mat500_mul_mat500", move |bh| bh.iter(|| &a * &b ));
}

fn mat500_add_scalar_ownership(bench: &mut Criterion)
{
    let a: Matrix<f64> = Matrix::new_random(500, 500);

    bench.bench_function("mat500_add_scalar_ownership", move |bh| {
             bh.iter(|| a.clone() + 5.0)
         });
}

fn mat500_add_scalar_borrow(bench: &mut Criterion)
{
    let a: Matrix<f64> = Matrix::new_random(500, 500);

    bench.bench_function("mat500_add_scalar_borrow", move |bh| bh.iter(|| &a + &5.0));
}
