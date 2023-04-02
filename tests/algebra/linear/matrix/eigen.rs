use mathru::algebra::linear::{Matrix, Vector};
use mathru::algebra::abstr::Complex;
use crate::mathru::algebra::abstr::cast::FromPrimitive;

#[cfg(not(feature = "intel-mkl"))]
#[test]
fn eigen_f32()
{
    let a: Matrix<f32> = matrix![   1.0, -3.0, 3.0;
                                    3.0, -5.0,  3.0;
                                    6.0, -6.0,  4.0];

    let eig_ref: Vector<f32> = vector![ 4.0;
                                        -2.0;
                                        -2.0];

    // let eig_vector_ref: Matrix<f32> = matrix![  1.0, 1.0, 2.0;
    //                                             -1.0, 0.0, 1.0;
    //                                             1.0, 1.0, 0.0];

    let value: Vector<f32> = a.dec_eigen().unwrap().value();

    assert_relative_eq!(value, eig_ref, epsilon=1.0e-5);
    // assert_relative_eq!(vector, eig_vector_ref, epsilon=1.0e-5);
}

#[test]
fn eigen_f64()
{
    let a: Matrix<f64> = matrix![   1.0, -3.0, 3.0;
                                    3.0, -5.0, 3.0;
                                    6.0, -6.0, 4.0];

    let eig_ref: Vector<f64> = vector![ 4.0;
                                        -2.0;
                                        -2.0];

    // let eig_vector_ref: Matrix<f64> = matrix![  1.0, 1.0, 2.0;
    //                                             -1.0, 0.0, 1.0;
    //                                             1.0, 1.0, 0.0];

    let value: Vector<f64> = a.dec_eigen().unwrap().value();

    assert_relative_eq!(value, eig_ref, epsilon=1.0e-5);
    // assert_relative_eq!(vector, eig_vector_ref, epsilon=1.0e-5);
}

// #[test]
// fn eigen_f64_2()
// {
//     let a: Matrix<f64> = matrix![   2.0, 0.0, 0.0;
//                                     0.0, -5.0, 3.0;
//                                     0.0, -6.0, 4.0];
//
//     let eig_ref: Vector<f64> = vector![ 1.0;
//                                         -2.0;
//                                         2.0];
//
//     // let eig_vector_ref: Matrix<f64> = matrix![  1.0, 1.0, 2.0;
//     //                                             -1.0, 0.0, 1.0;
//     //                                             1.0, 1.0, 0.0];
//
//     let value: Vector<f64> = a.dec_eigen().unwrap().value();
//
//     assert_relative_eq!(value, eig_ref, epsilon=1.0e-5);
//     // assert_relative_eq!(vector, eig_vector_ref, epsilon=1.0e-5);
// }

#[cfg(feature = "native")]
#[test]
fn eigen_complex_f32()
{
    let a: Matrix<Complex<f32>> = matrix![  Complex::from_f32(1.0), Complex::from_f32(-3.0), Complex::from_f32(3.0);
                                            Complex::from_f32(3.0), Complex::from_f32(-5.0), Complex::from_f32(3.0);
                                            Complex::from_f32(6.0), Complex::from_f32(-6.0), Complex::from_f32(4.0)];

    let eig_value_ref: Vector<Complex<f32>> = vector![  Complex::from_f32(4.0);
                                                        Complex::from_f32(-2.0);
                                                        Complex::from_f32(-2.0)];

    // let eig_vector_ref: Matrix<Complex<f32>> = matrix![ Complex::new(0.5773503, 0.0), Complex::new(0.5773502, 0.0), Complex::new(-0.45936325, -0.0);
    //                                                     Complex::new(0.5773502, 0.0), Complex::new(-0.5773502, 0.0), Complex::new(0.864806, 0.0);
    //                                                     Complex::new(-0.57735026, 0.0), Complex::new(0.57735026, 0.0), Complex::new(-0.20272121, 0.0)];

    let value: Vector<Complex<f32>> = a.dec_eigen().unwrap().value();

    assert_relative_eq!(value, eig_value_ref, epsilon=Complex::new(2.0e-5, 2.0e-5));
    // assert_relative_eq!(vector, eig_vector_ref, epsilon=Complex::new(2.0e-5, 2.0e-5));
}

#[cfg(feature = "lapack")]
#[test]
fn eigen_complex_f32()
{
    let a: Matrix<Complex<f32>> = matrix![  Complex::from_f32(1.0), Complex::from_f32(-3.0), Complex::from_f32(3.0);
                                            Complex::from_f32(3.0), Complex::from_f32(-5.0), Complex::from_f32(3.0);
                                            Complex::from_f32(6.0), Complex::from_f32(-6.0), Complex::from_f32(4.0)];

    let eig_value_ref: Vector<Complex<f32>> = vector![  Complex::from_f32(-2.0);
                                                        Complex::from_f32(4.0);
                                                        Complex::from_f32(-2.0)];

    // let eig_vector_ref: Matrix<Complex<f32>> = matrix![ Complex::new(0.5773503, 0.0), Complex::new(0.5773502, 0.0), Complex::new(-0.45936325, -0.0);
    //                                                     Complex::new(0.5773502, 0.0), Complex::new(-0.5773502, 0.0), Complex::new(0.864806, 0.0);
    //                                                     Complex::new(-0.57735026, 0.0), Complex::new(0.57735026, 0.0), Complex::new(-0.20272121, 0.0)];

    let value: Vector<Complex<f32>> = a.dec_eigen().unwrap().value();

    assert_relative_eq!(value, eig_value_ref, epsilon=Complex::new(2.0e-5, 2.0e-5));
    // assert_relative_eq!(vector, eig_vector_ref, epsilon=Complex::new(2.0e-5, 2.0e-5));
}

#[test]
fn eigen_complex_f64()
{
    let a: Matrix<Complex<f64>> = matrix![  Complex::from_f64(1.0), Complex::from_f64(-3.0), Complex::from_f64(3.0);
                                            Complex::from_f64(3.0), Complex::from_f64(-5.0), Complex::from_f64(3.0);
                                            Complex::from_f64(6.0), Complex::from_f64(-6.0), Complex::from_f64(4.0)];

    let eig_value_ref: Vector<Complex<f64>> = vector![  Complex::from_f64(4.0);
                                                        Complex::from_f64(-2.0);
                                                        Complex::from_f64(-2.0)];

    // let eig_vector_ref: Matrix<Complex<f64>> = matrix![ Complex::new(-0.4082, 0.0), Complex::new(0.2440, -0.4070), Complex::new(0.2440, 0.4070);
    //                                                     Complex::new(-0.4082, 0.0), Complex::new(-0.4162, -0.4070), Complex::new(-0.4162, 0.4070);
    //                                                     Complex::new(-0.8165, 0.0), Complex::new(-0.6602, 0.0), Complex::new(-0.6602, 0.0)];

    let value: Vector<Complex<f64>> = a.dec_eigen().unwrap().value();

    assert_relative_eq!(value, eig_value_ref, epsilon=Complex::new(1.0e-10, 1.0e-10));
    // assert_relative_eq!(vector, eig_vector_ref, epsilon=Complex::new(1.0e-10, 1.0e-10));
}

// #[test]
// fn eigenvalue_complex_f64_2()
// {
//     let a: Matrix<Complex<f64>> = matrix![  Complex::from_f64(-3.00000), Complex::from_f64(-3.0), Complex::from_f64(3.0);
//                                     Complex::from_f64(3.0), Complex::from_f64(-9.00000), Complex::from_f64(3.0);
//                                     Complex::from_f64(6.0), Complex::from_f64(-6.0), Complex::from_f64(0.000000)];
//
//     let eig_ref: Vector<Complex<f64>> = vector![ Complex::from_f64(0.0);
//                                         Complex::from_f64(-6.0);
//                                         -Complex::from_f64(6.0)];
//
//     let eig_vector_ref: Matrix<Complex<f64>> = matrix![  Complex::from_f64(1.0), Complex::from_f64(1.0), Complex::from_f64(2.0);
//                                                 Complex::from_f64(-1.0), Complex::from_f64(0.0), Complex::from_f64(1.0);
//                                                 Complex::from_f64(1.0), Complex::from_f64(1.0), Complex::from_f64(0.0)];
//
//     let (value, vector): (Vector<Complex<f64>>, Matrix<Complex<f64>>) = a.dec_eigen().unwrap().pair();
//
//     assert_relative_eq!(value, eig_ref, epsilon=Complex::new(1.0e-5, 1.0e-5));
//     assert_relative_eq!(vector, eig_vector_ref, epsilon=Complex::new(1.0e-5, 1.0e-5));
// }

