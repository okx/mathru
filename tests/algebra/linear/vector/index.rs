use mathru::algebra::linear::Vector;

#[test]
fn index_row_vector()
{
    let a: Vector<f64> = Vector::new_row(vec![1.0, 0.0, 3.0, -2.0]);
    
    assert_eq!(-2.0, a[3])
}

#[test]
fn index_column_vector()
{
    let a: Vector<f64> = Vector::new_column(vec![1.0, 0.0, 3.0, -2.0]);
    
    assert_eq!(-2.0, a[3])
}


#[test]
fn index_mut_row_vector()
{
    let mut a: Vector<f64> = Vector::new_row(vec![1.0, 0.0, 3.0, -2.0]);
    
    a[3] = 2.0;
    assert_eq!(2.0, a[3])
}

#[test]
fn index_mut_column_vector()
{
    let mut a: Vector<f64> = Vector::new_column(vec![1.0, 0.0, 3.0, -2.0]);
    
    a[3] = 2.0;
    assert_eq!(2.0, a[3])
}

#[test]
fn index_column_vector_macro()
{
    let a: Vector<f64> = vector![1.0; 0.0; 3.0; -2.0];
    
    assert_eq!(-2.0, a[3])
}
