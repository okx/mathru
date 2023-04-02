use mathru::algebra::linear::{Matrix/*, Vector*/};

#[test]
fn matrix()
{
    let mut a: Matrix<f32> = matrix![   1.0, -2.0, -3.0;
                                        -4.0, -1.0, -2.0];

    let b: Matrix<f32> = matrix![   6.0, 3.0;
                                    2.0, 1.0;
                                    4.0, 2.0];

    let prod_ref: Matrix<f32> = matrix![ -10.0, -5.0;
                                        -34.0, -17.0];

    a *= b;

    assert_relative_eq!(prod_ref, a);
}

#[test]
fn scalar()
{
    let mut a: Matrix<f32> = matrix![   1.0, -2.0, -3.0;
                                        -4.0, -1.0, 2.0];

    let prod_ref: Matrix<f32> = matrix![ 5.0, -10.0, -15.0;
                                        -20.0, -5.0, 10.0];
    a *= 5.0f32;

    assert_relative_eq!(prod_ref, a);
}