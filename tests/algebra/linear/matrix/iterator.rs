use mathru::algebra::linear::matrix::{MatrixIterator, MatrixIteratorMut, MatrixIntoIterator, MatrixColumnIntoIterator, MatrixRowIntoIterator};
use mathru::{vector, matrix};
use mathru::algebra::linear::{Vector, Matrix};

#[test]
fn iter()
{
    let m: Matrix<f64> = matrix![1.0, -4.0];
    let mut iter: MatrixIterator<f64> = m.iter();

    assert_eq!(iter.next(), Some(&1.0f64));
    assert_eq!(iter.next(), Some(&-4.0f64));
}

#[test]
fn iter_mut()
{
    let mut m: Matrix<f64> = matrix![1.0, -4.0];
    let mut iter_mut: MatrixIteratorMut<f64> = m.iter_mut();

    assert_eq!(iter_mut.next(), Some(&mut 1.0f64));
    let last = iter_mut.next().unwrap();
    assert_eq!(*last, -4.0f64);
    *last = 3.0;
    assert_eq!(*last, 3.0f64);
}

#[test]
fn into_iter()
{
    let m: Matrix<f64> = matrix![1.0, -4.0];
    let mut iter: MatrixIntoIterator<f64> = m.into_iter();

    assert_eq!(iter.next(), Some(1.0f64));
    let last: f64 = iter.next().unwrap();
    assert_eq!(last, -4.0f64);
}

#[test]
fn column_iter()
{
    let m: Matrix<f64> = matrix![1.0f64, 2.0; 3.0, 4.0];
    let mut iter: MatrixColumnIntoIterator<f64> = m.column_into_iter();

    assert_eq!(iter.next(), Some(vector![1.0f64; 3.0]));
    assert_eq!(iter.next(), Some(vector![2.0f64; 4.0]));
    assert_eq!(iter.next(), None);
}

#[test]
fn row_iter()
{
    let m: Matrix<f64> = matrix![1.0f64, 2.0; 3.0, 4.0];
    let mut iter: MatrixRowIntoIterator<f64> = m.row_into_iter();

    assert_eq!(iter.next(), Some(vector![1.0f64, 2.0]));
    assert_eq!(iter.next(), Some(vector![3.0f64, 4.0]));
    assert_eq!(iter.next(), None);
}


