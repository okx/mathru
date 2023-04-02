use mathru::algebra::linear::Vector;

#[test]
fn vector_owner()
{
    let a: Vector<f32> = Vector::new_column(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    let b: Vector<f32> = Vector::new_column(vec![1.0, 4.0, -1.0, 0.0, -7.0]);
    let res_ref: Vector<f32> = Vector::new_column(vec![2.0, 6.0, 2.0, 4.0, -2.0]);

    let res: Vector<f32> = a + b;

    assert_relative_eq!(res_ref, res);
}

#[test]
fn vector_borrow()
{
    let a: Vector<f32> = Vector::new_column(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    let b: Vector<f32> = Vector::new_column(vec![1.0, 4.0, -1.0, 0.0, -7.0]);
    let res_ref: Vector<f32> = Vector::new_column(vec![2.0, 6.0, 2.0, 4.0, -2.0]);

    let res: Vector<f32> = &a + &b;

    assert_relative_eq!(res, res_ref);
}

#[test]
fn vector_borrow_mut()
{
    let mut a: Vector<f32> = Vector::new_column(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    let b: Vector<f32> = Vector::new_column(vec![1.0, 4.0, -1.0, 0.0, -7.0]);
    let res_ref: Vector<f32> = Vector::new_column(vec![2.0, 6.0, 2.0, 4.0, -2.0]);

    let res = &mut a + &b;

    assert_relative_eq!(*res, res_ref);
}

#[test]
fn scalar_owner()
{
    let a: Vector<f32> = Vector::new_column(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    let res_ref: Vector<f32> = Vector::new_column(vec![6.0, 7.0, 8.0, 9.0, 10.0]);

    let res: Vector<f32> = a + 5.0 ;

    assert_relative_eq!(res, res_ref);
}

#[test]
fn scalar_borrow()
{
    let a: Vector<f32> = Vector::new_column(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    let res_ref: Vector<f32> = Vector::new_column(vec![6.0, 7.0, 8.0, 9.0, 10.0]);

    let res: Vector<f32> = &a + &5.0;

    assert_relative_eq!(res, res_ref);
}

#[test]
fn scalar_borrow_mut()
{
    let mut a: Vector<f32> = Vector::new_column(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    let res_ref: Vector<f32> = Vector::new_column(vec![6.0, 7.0, 8.0, 9.0, 10.0]);

    let _ = &mut a + &5.0;

    assert_relative_eq!(a, res_ref);
}


