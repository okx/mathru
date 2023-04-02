use mathru::algebra::linear::Matrix;
use crate::mathru::algebra::abstr::cast::FromPrimitive;
use mathru::algebra::abstr::Complex;

#[test]
fn sub_matrix_own()
{
    let a: Matrix<f64> = matrix![   1.0, -2.0, -3.0;
                                    -4.0, -1.0, -2.5];

    let b: Matrix<f64> = matrix![   -2.0, -7.0, 3.0;
                                    -5.0, 3.5, 0.0];

    let sum_ref: Matrix<f64> = matrix![ 3.0, 5.0, -6.0;
                                        1.0, -4.5, -2.5];

    assert_relative_eq!(sum_ref, a - b);
}

#[test]
fn sub_matrix_borrow()
{
    let a: Matrix<f64> = matrix![   1.0, -2.0, -3.0;
                                    -4.0, -1.0, -2.5];

    let b: Matrix<f64> = matrix![   -2.0, -7.0, 3.0;
                                    -5.0, 3.5, 0.0];

    let sum_ref: Matrix<f64> = matrix![ 3.0, 5.0, -6.0;
                                        1.0, -4.5, -2.5];

    assert_relative_eq!(sum_ref, &a - &b);
}

#[test]
fn sub_matrix_mut_borrow()
{
    let mut a: Matrix<f64> = matrix![   1.0, -2.0, -3.0;
                                    -4.0, -1.0, -2.5];

    let b: Matrix<f64> = matrix![   -2.0, -7.0, 3.0;
                                    -5.0, 3.5, 0.0];

    let sum_ref: Matrix<f64> = matrix![ 3.0, 5.0, -6.0;
                                        1.0, -4.5, -2.5];

    assert_relative_eq!(sum_ref, &mut a - &b);
}

#[test]
fn sub_scalar_own()
{
    let a: Matrix<f64> = matrix![   1.0, -2.0, -3.0;
                                    -4.0, -1.0, -2.5];

    let sum_ref: Matrix<f64> = matrix![ -4.0, -7.0, -8.0;
                                        -9.0, -6.0, -7.5];

    assert_relative_eq!(sum_ref, a - 5.0f64);
}

#[test]
fn sub_scalar_borrow()
{
    let a: Matrix<f64> = matrix![   1.0, -2.0, -3.0;
                                    -4.0, -1.0, -2.5];

    let sum_ref: Matrix<f64> = matrix![ -4.0, -7.0, -8.0;
                                        -9.0, -6.0, -7.5];

    assert_relative_eq!(sum_ref, &a - &5.0f64);
}

#[test]
fn sub_scalar_mut_borrow()
{
    let mut a: Matrix<f64> = matrix![   1.0, -2.0, -3.0;
                                        -4.0, -1.0, -2.5];

    let sum_ref: Matrix<f64> = matrix![ -4.0, -7.0, -8.0;
                                        -9.0, -6.0, -7.5];

    assert_relative_eq!(sum_ref, &mut a - &5.0f64);
}

#[test]
fn sub_complex_f32()
{
    let a: Matrix<Complex<f32>> = matrix![  Complex::from_f32(1.0), Complex::from_f32(-2.0), Complex::from_f32(-3.0);
                                            Complex::from_f32(-4.0), Complex::from_f32(-1.0), Complex::from_f32(-2.5)];

    let b: Matrix<Complex<f32>> = matrix![  Complex::from_f32(-2.0), Complex::from_f32(-7.0), Complex::from_f32(3.0);
                                            Complex::from_f32(-5.0), Complex::from_f32(3.5), Complex::from_f32(0.0)];

    let sum_ref: Matrix<Complex<f32>> = matrix![    Complex::from_f32(3.0), Complex::from_f32(5.0), Complex::from_f32(-6.0);
                                                    Complex::from_f32(1.0), Complex::from_f32(-4.5), Complex::from_f32(-2.5)];

    assert_relative_eq!(sum_ref, a - b);
}

#[test]
fn sub_complex_f64()
{
    let a: Matrix<Complex<f64>> = matrix![  Complex::new(1.0, 1.0), Complex::new(-2.0, 2.0) ;
                                            Complex::new(-4.0, 3.0), Complex::new(1.0, -5.0)];

    let b: Matrix<Complex<f64>> = matrix![  Complex::new(-2.0, 3.0), Complex::new(-7.0, 5.0);
                                            Complex::new(-5.0, 2.0), Complex::new(-1.0, -4.0)];

    let sum_ref: Matrix<Complex<f64>> = matrix![    Complex::new(3.0, -2.0), Complex::new(5.0, -3.0);
                                                    Complex::new(1.0, 1.0), Complex::new(2.0, -1.0)];

    assert_relative_eq!(sum_ref, a - b);
}

