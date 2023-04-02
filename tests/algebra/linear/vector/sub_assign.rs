use mathru::algebra::linear::Vector;

#[test]
fn vector()
{
    let mut a: Vector<f32> = vector![ 1.0, -2.0, -3.0];
    let b: Vector<f32> = vector![2.0, 3.0, -4.0];
    let diff_ref: Vector<f32> = vector![-1.0, -5.0, 1.0];

    a -= b;

    assert_relative_eq!(diff_ref, a);
}

#[test]
fn scalar()
{
    let mut a: Vector<f32> = vector![ 1.0, -2.0, -3.0];
    let diff_ref: Vector<f32> = vector![-4.0, -7.0, -8.0];

    a -= 5.0f32;

    assert_relative_eq!(diff_ref, a);
}