use mathru::{
    algebra::{
        abstr::Sign,
        linear::{Matrix, Vector},
        linear::vector::{VectorIterator, VectorIteratorMut, VectorIntoIterator},
    },
    elementary::Power,
};

#[test]
fn macro_vector_column()
{
    let vec: Vector<f32> = vector![1.0; 2.0; 3.0];

    let vec_ref: Vector<f32> = Vector::new_column(vec![1.0, 2.0, 3.0]);

    assert_relative_eq!(vec_ref, vec);
}

#[test]
fn macro_vector_row()
{
    let vec: Vector<f32> = vector![1.0, 2.0, 3.0];

    let vec_ref: Vector<f32> = Vector::new_row(vec![1.0, 2.0, 3.0]);

    assert_relative_eq!(vec_ref, vec);
}

#[cfg(feature = "serde")]
#[test]
fn serde_0()
{
    let mat: Vector<f64> = vector![1.0; 2.0; 3.0];
    let serialized = serde_json::to_string(&mat).unwrap();

    let mat_s: Vector<f64> = serde_json::from_str(&serialized).unwrap();

    assert_relative_eq!(mat_s, mat);
}

#[test]
fn zero()
{
    let rows: usize = 5;

    let m_zero: Vector<f32> = Vector::zero(rows);
    let (m, n) = m_zero.dim();
    assert_eq!(m, rows);
    assert_eq!(n, 1);

    for i in 0..rows
    {
        assert_relative_eq!(m_zero[i], 0.0);
    }
}

#[test]
fn partial_eq0()
{
    let lhs: Vector<f32> = Vector::new_column(vec![1.0, 2.0]);
    let rhs: Vector<f32> = Vector::new_column(vec![1.0, 2.0]);
    assert_relative_eq!(lhs, rhs);
}

#[test]
fn partial_eq1()
{
    let lhs: Vector<f32> = Vector::new_column(vec![1.0, 2.0]);
    let rhs: Vector<f32> = Vector::new_column(vec![1.0, 2.0]);
    assert_relative_eq!(lhs, rhs);
}

#[test]
fn scalar_div_owner()
{
    let a: Vector<f32> = Vector::new_column(vec![20.0, -10.0, 12.0, -4.0, -1.0]);
    let res_ref: Vector<f32> = Vector::new_column(vec![-10.0, 5.0, -6.0, 2.0, 0.5]);

    let res: Vector<f32> = a / -2.0;

    assert_relative_eq!(res, res_ref);
}

#[test]
fn scalar_div_borrow()
{
    let a: Vector<f32> = Vector::new_column(vec![20.0, -10.0, 12.0, -4.0, -1.0]);
    let res_ref: Vector<f32> = Vector::new_column(vec![-10.0, 5.0, -6.0, 2.0, 0.5]);

    let res: Vector<f32> = &a / &-2.0;

    assert_relative_eq!(res, res_ref);
}

#[test]
fn get_0()
{
    let res: Vector<f32> = Vector::new_column(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    let res_ref: Vector<f32> = Vector::new_column(vec![1.0, 2.0, 3.0, 4.0, 5.0]);

    assert_relative_eq!(res, res_ref);
}

#[test]
fn get_1()
{
    let res: Vector<f32> = Vector::new_row(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    let res_ref: Vector<f32> = Vector::new_column(vec![1.0, 2.0, 3.0, 4.0, 5.0]);

    assert_relative_eq!(res, res_ref);
}

#[test]
fn get_slice_0()
{
    let res: Vector<f32> = Vector::new_column(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    let res_ref: Vector<f32> = Vector::new_column(vec![3.0, 4.0, 5.0]);

    let slice: Vector<f32> = res.get_slice(2, 4);

    assert_relative_eq!(res_ref, slice);
}

#[test]
fn set_slice_0()
{
    let mut a: Vector<f32> = Vector::new_column(vec![1.0, 2.0, 3.0, 4.0, 5.0]);

    let b: Vector<f32> = Vector::new_column(vec![-3.0, -4.0, -5.0]);
    let res_ref: Vector<f32> = Vector::new_column(vec![1.0, 2.0, -3.0, -4.0, -5.0]);

    a.set_slice(&b, 2);

    assert_relative_eq!(res_ref, a);
}

#[test]
fn dotp_0()
{
    let a: Vector<f32> = Vector::new_column(vec![-1.0, -3.0, 6.0, -1.0]);
    let b: Vector<f32> = Vector::new_column(vec![-2.0, -5.0, -3.0, 2.0]);
    let dotp_ref: f32 = -3.0;
    let dotp: f32 = a.dotp(&b);
    assert_relative_eq!(dotp_ref, dotp);
}

#[test]
fn dotp_1()
{
    let a: Vector<f32> = Vector::new_column(vec![-1.0, -3.0, 6.0, -1.0]);
    let dotp_ref: f32 = 47.0;
    let dotp: f32 = a.dotp(&a);
    assert_relative_eq!(dotp_ref, dotp);
}

//    #[test]
//    fn crossp()
//    {
//        let a: Vector<f32> = Vector::new_column(3, vec![-1.0, -3.0, 6.0]);
//        let b: Vector<f32> = Vector::new_column(3, vec![-2.0, -5.0,-3.0]);
//        let crossp_ref : f32 = -3.0;
//        let cross : Vector<f32> = a.crossp(&b);
//        assert_relative_eq!(crossp_ref, crossp);
//    }

#[test]
fn dyadp()
{
    let x: Vector<f32> = vector![   1.0;
                                    3.0;
                                    2.0];

    let y: Vector<f32> = Vector::new_column(vec![2.0, 1.0, 0.0, 3.0]);

    let dyadp_ref: Matrix<f32> = matrix![  2.0, 1.0, 0.0, 3.0;
                                            6.0, 3.0, 0.0, 9.0;
                                            4.0, 2.0, 0.0, 6.0];

    let p: Matrix<f32> = x.dyadp(&y);

    assert_relative_eq!(dyadp_ref, p);
}

#[test]
fn p_norm()
{
    let p: f32 = 2.0;
    let v: Vector<f32> = Vector::new_column(vec![-2.0, -5.0, -3.0, 2.0]);
    let p_norm_ref: f32 = 42.0.pow(0.5);
    let p_norm: f32 = v.p_norm(&p);
    assert_relative_eq!(p_norm_ref, p_norm);
}


#[test]
fn argmax()
{
    let m: Vector<f64> = vector![1.0, -2.0, 3.0, -4.0];

    assert_eq!(2, m.argmax());
}

#[test]
fn argmin()
{
    let m: Vector<f64> = vector![1.0, -2.0, 3.0, -4.0];

    assert_eq!(3, m.argmin());
}

#[test]
fn sign()
{
    let v: Vector<f64> = vector![1.0, -2.0, 0.0, -4.0];

    let sign: Vector<f64> = vector![1.0, -1.0, 0.0, -1.0];
    let sign_hat: Vector<f64> = v.sign();

    assert_relative_eq!(sign, sign_hat);
}

#[test]
fn abs()
{
    let v: Vector<f64> = vector![1.0, -2.0, 0.0, -4.0];

    let abs_ref: Vector<f64> = vector![1.0, 2.0, 0.0, 4.0];
    let abs: Vector<f64> = v.abs();

    assert_relative_eq!(abs_ref, abs);
}

#[test]
fn iter()
{
    let v: Vector<f64> = vector![1.0, -4.0];
    let mut iter: VectorIterator<f64> = v.iter();

    assert_eq!(iter.next(), Some(&1.0f64));
    assert_eq!(iter.next(), Some(&-4.0f64));
}

#[test]
fn iter_mut()
{
    let mut v: Vector<f64> = vector![1.0, -4.0];
    let mut iter_mut: VectorIteratorMut<f64> = v.iter_mut();

    assert_eq!(iter_mut.next(), Some(&mut 1.0f64));
    let last = iter_mut.next().unwrap();
    assert_eq!(*last, -4.0f64);
    *last = 3.0;
    assert_eq!(*last, 3.0f64);
}

#[test]
fn into_iter()
{
    let v: Vector<f64> = vector![1.0, -4.0];
    let mut iter: VectorIntoIterator<f64> = v.into_iter();

    assert_eq!(iter.next(), Some(1.0f64));
    let last = iter.next().unwrap();
    assert_eq!(last, -4.0f64);
}
