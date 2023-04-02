use mathru::algebra::linear::Matrix;


#[test]
fn matrix()
{
    let mut a: Matrix<f32> = matrix![   1.0, -2.0, -3.0;
                                        -4.0, -1.0, -2.5];

    let b: Matrix<f32> = matrix![   6.0, 3.0, 2.0;
                                    1.0, 4.0, 2.5];

    let diff_ref: Matrix<f32> = matrix![-5.0, -5.0, -5.0;
                                        -5.0, -5.0, -5.0];

    a -= b;

    assert_relative_eq!(diff_ref, a);
}

#[test]
fn scalar()
{
    let mut a: Matrix<f32> = matrix![   1.0, -2.0, -3.0;
                                        -4.0, -1.0, -2.5];

    let diff_ref: Matrix<f32> = matrix![ -4.0, -7.0, -8.0;
                                        -9.0, -6.0, -7.5];
    a -= 5.0f32;

    assert_relative_eq!(diff_ref, a);
}