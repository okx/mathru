use mathru::algebra::linear::Matrix;

#[test]
fn matrix()
{
    let mut a: Matrix<f32> = matrix![   1.0, -2.0, -3.0;
                                        -4.0, -1.0, -2.5];

    let b: Matrix<f32> = matrix![   6.0, 3.0, 2.0;
                                    1.0, 4.0, 2.5];

    let sum_ref: Matrix<f32> = matrix![ 7.0, 1.0, -1.0;
                                        -3.0, 3.0, 0.0];

    a += b;

    assert_relative_eq!(sum_ref, a);
}

#[test]
fn scalar()
{
    let mut a: Matrix<f32> = matrix![   1.0, -2.0, -3.0;
                                        -4.0, -1.0, -2.5];
    let sum: Matrix<f32> = matrix![ 6.0, 3.0, 2.0;
                                    1.0, 4.0, 2.5];
    a += 5.0f32;

    assert_relative_eq!(sum, a);
}