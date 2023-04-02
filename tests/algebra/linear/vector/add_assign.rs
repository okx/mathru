use mathru::algebra::linear::Vector;

#[test]
fn scalar()
{
    let mut a: Vector<f32> = vector![ 1.0, -2.0, -3.0];

    a += 5.0f32;

    let sum: Vector<f32> = vector![6.0, 3.0, 2.0];

    assert_relative_eq!(sum, a);
}

#[test]
fn vector()
{
    let mut a: Vector<f32> = vector![ 1.0, -2.0, -3.0];
    let b: Vector<f32> = vector![2.0, 3.0, -4.0];

    a += b;

    let sum: Vector<f32> = vector![3.0, 1.0, -7.0];

    assert_relative_eq!(sum, a);
}