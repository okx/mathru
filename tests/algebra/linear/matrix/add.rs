use mathru::algebra::linear::Matrix;
use crate::mathru::algebra::abstr::cast::FromPrimitive;
use mathru::algebra::abstr::Complex;

#[test]
fn add_matrix_own_f32()
{
    let a: Matrix<f32> = matrix![   1.0, -2.0, -3.0;
                                    -4.0, -1.0, -2.5];

    let b: Matrix<f32> = matrix![   -2.0, -7.0, 3.0;
                                    -5.0, 3.5, 0.0];

    let sum_ref: Matrix<f32> = matrix![ -1.0, -9.0, 0.0;
                                        -9.0, 2.5, -2.5];

    assert_relative_eq!(sum_ref, a + b);
}

#[test]
fn add_matrix_own()
{
    let a: Matrix<f64> = matrix![   1.0, -2.0, -3.0;
                                    -4.0, -1.0, -2.5];

    let b: Matrix<f64> = matrix![   1.0, -2.0, -3.0;
                                    -4.0, -1.0, -2.5];

    let sum_ref: Matrix<f64> = matrix![ 2.0, -4.0, -6.0;
                                        -8.0, -2.0, -5.0];

    assert_relative_eq!(sum_ref, a + b);
}

#[test]
fn add_matrix_borrow()
{
    let a: Matrix<f64> = matrix![   1.0, -2.0, -3.0;
                                    -4.0, -1.0, -2.5];

    let b: Matrix<f64> = matrix![   1.0, -2.0, -3.0;
                                    -4.0, -1.0, -2.5];

    let sum_ref: Matrix<f64> = matrix![ 2.0, -4.0, -6.0;
                                        -8.0, -2.0, -5.0];

    assert_relative_eq!(sum_ref, &a + &b);
}

#[test]
fn add_matrix_mut_borrow()
{
    let mut a: Matrix<f64> = matrix![   1.0, -2.0, -3.0;
                                    -4.0, -1.0, -2.5];

    let b: Matrix<f64> = matrix![   1.0, -2.0, -3.0;
                                    -4.0, -1.0, -2.5];

    let sum_ref: Matrix<f64> = matrix![ 2.0, -4.0, -6.0;
                                        -8.0, -2.0, -5.0];

    assert_relative_eq!(sum_ref, &mut a + &b);
}

#[test]
fn add_scalar_own()
{
    let a: Matrix<f64> = matrix![   1.0, -2.0, -3.0;
                                    -4.0, -1.0, -2.5];

    let sum_ref: Matrix<f64> = matrix![ 6.0, 3.0, 2.0;
                                        1.0, 4.0, 2.5];

    assert_relative_eq!(sum_ref, a + 5.0f64);
}

#[test]
fn add_scalar_borrow()
{
    let a: Matrix<f32> = matrix![   1.0, -2.0, -3.0;
                                    -4.0, -1.0, -2.5];

    let sum_ref: Matrix<f32> = matrix![ 6.0, 3.0, 2.0;
                                        1.0, 4.0, 2.5];

    assert_relative_eq!(sum_ref, &a + &5.0f32);
}

#[test]
fn add_scalar_mut_borrow()
{
    let mut a: Matrix<f32> = matrix![   1.0, -2.0, -3.0;
                                    -4.0, -1.0, -2.5];

    let sum_ref: Matrix<f32> = matrix![ 6.0, 3.0, 2.0;
                                        1.0, 4.0, 2.5];

    assert_relative_eq!(sum_ref, &mut a + &5.0f32);
}


#[test]
fn add_complex_f32()
{
    let a: Matrix<Complex<f32>> = matrix![  Complex::from_f32(1.0), Complex::from_f32(-2.0), Complex::from_f32(-3.0);
                                            Complex::from_f32(-4.0), Complex::from_f32(-1.0), Complex::from_f32(-2.5)];

    let b: Matrix<Complex<f32>> = matrix![  Complex::from_f32(-2.0), Complex::from_f32(-7.0), Complex::from_f32(3.0);
                                            Complex::from_f32(-5.0), Complex::from_f32(3.5), Complex::from_f32(0.0)];

    let sum_ref: Matrix<Complex<f32>> = matrix![    Complex::from_f32(-1.0), Complex::from_f32(-9.0), Complex::from_f32(0.0);
                                                    Complex::from_f32(-9.0), Complex::from_f32(2.5), Complex::from_f32(-2.5)];

    assert_relative_eq!(sum_ref, a + b);
}

#[test]
fn add_complex_f64()
{
    let a: Matrix<Complex<f64>> = matrix![  Complex::new(1.0, 1.0), Complex::new(-2.0, 2.0) ;
                                            Complex::new(-4.0, 3.0), Complex::new(1.0, -5.0)];

    let b: Matrix<Complex<f64>> = matrix![  Complex::new(-2.0, 3.0), Complex::new(-7.0, 5.0);
                                            Complex::new(-5.0, 2.0), Complex::new(-1.0, -4.0)];

    let sum_ref: Matrix<Complex<f64>> = matrix![    Complex::new(-1.0, 4.0), Complex::new(-9.0, 7.0);
                                                    Complex::new(-9.0, 5.0), Complex::new(0.0, -9.0)];

    assert_relative_eq!(sum_ref, a + b);
}