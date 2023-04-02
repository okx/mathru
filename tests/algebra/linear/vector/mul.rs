use mathru::algebra::linear::{Vector, Matrix};

#[test]
fn matrix_owner()
{
    let m = matrix![1.0, 2.0; 3.0, 4.0];
    let v = vector![1.0, 2.0];
    let prod_ref = vector![7.0, 10.0];

    let res = v * m;

    assert_relative_eq!(prod_ref, res);
}

#[test]
fn matrix_borrow()
{
    let m: Matrix<f64> = matrix![1.0, 2.0; 3.0, 4.0];
    let v: Vector<f64> = vector![1.0, 2.0];
    let prod_ref: Vector<f64> = vector![7.0, 10.0];

    let res = &v * &m;

    assert_relative_eq!(prod_ref, res);
}


#[test]
fn scalar_owner()
{
    let a: Vector<f32> = Vector::new_column(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    let res_ref: Vector<f32> = Vector::new_column(vec![5.0, 10.0, 15.0, 20.0, 25.0]);

    let res: Vector<f32> = a * 5.0;

    assert_relative_eq!(res, res_ref);
}

#[test]
fn scalar_borrow()
{
    let a: Vector<f32> = Vector::new_column(vec![1.0, 2.0, 3.0, 4.0, -5.0]);
    let res_ref: Vector<f32> = Vector::new_column(vec![-5.0, -10.0, -15.0, -20.0, 25.0]);

    let res: Vector<f32> = &a * &-5.0;

    assert_relative_eq!(res, res_ref);
}

#[test]
fn scalar_borrow_mut()
{
    let mut a: Vector<f32> = Vector::new_column(vec![1.0, 2.0, 3.0, 4.0, -5.0]);
    let res_ref: Vector<f32> = Vector::new_column(vec![-5.0, -10.0, -15.0, -20.0, 25.0]);

    let _ = &mut a * &-5.0;

    assert_relative_eq!(a, res_ref);
}