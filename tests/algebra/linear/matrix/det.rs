use mathru::algebra::linear::Matrix;
use mathru::algebra::abstr::Complex;

#[test]
fn det_one_element()
{
    let a: Matrix<f64> = matrix![-2.0];
    let det: f64 = a.det();

    assert_abs_diff_eq!(-2.0, det, epsilon=1.0e-10);
}

#[test]
fn determinant_quadratic_2()
{
    let a: Matrix<f64> = matrix![   1.0, -2.0;
                                    3.0, -7.0];
    let det: f64 = a.det();

    assert_abs_diff_eq!(-1.0, det, epsilon=1.0e-10);
}

#[test]
fn determinant_quadratic_3()
{
    let a: Matrix<f32> = matrix![   1.0, -2.0, 3.0;
                                    2.0, -5.0, 12.0;
                                    1.0, 2.0, -10.0];
    let det: f32 = a.det();

    assert_abs_diff_eq!(-11.0, det, epsilon=1.0e-4);
}

#[test]
fn determinant_quadratic_4()
{
    let a: Matrix<f64> = matrix![   4.0, 1.0, -2.0, 2.0;
                                    1.0, 2.0, 0.0, -2.0;
                                    0.0, 3.0, -2.0, 2.0;
                                    2.0, 1.0, -2.0, -1.0];

    let det: f64 = a.det();

    assert_abs_diff_eq!(76.0, det, epsilon=1.0e-10);
}

#[test]
fn determinant_quadratic_f32()
{
    let a: Matrix<f32> = matrix![   1.0, -2.0;
                                    3.0, -7.0];
    let det: f32 = a.det();

    assert_relative_eq!(-1.0, det, epsilon=1.0e-5);
}

#[test]
fn determinant_quadratic_f64()
{
    let a: Matrix<f64> = matrix![   1.0, -2.0;
                                    3.0, -7.0];
    let det: f64 = a.det();

    assert_relative_eq!(-1.0, det, epsilon=1.0e-10);
}

#[test]
fn determinant_complex_f32()
{
    let a: Matrix<Complex<f32>> = matrix![  Complex::new(1.0, -1.0), Complex::new(-2.0, -1.0);
                                            Complex::new(3.0, 2.0), Complex::new(-7.0, 2.0)];
    let det: Complex<f32> = a.det();

    assert_relative_eq!(Complex::new(-1.0, 16.0), det, epsilon=Complex::new(1.0e-5, 1.0e-5));
}

#[test]
fn determinant_complex_f64()
{
    let a: Matrix<Complex<f64>> = matrix![  Complex::new(1.0, -1.0), Complex::new(-2.0, -1.0);
                                            Complex::new(3.0, 2.0), Complex::new(-7.0, 2.0)];
    let det: Complex<f64> = a.det();

    assert_relative_eq!(Complex::new(-1.0, 16.0), det, epsilon=Complex::new(1.0e-5, 1.0e-5));
}


