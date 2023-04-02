use mathru::algebra::linear::Vector;

#[test]
fn transpose()
{
    let (m_ref, n_ref): (usize, usize) = (4, 1);
    let vec: Vector<f32> = Vector::new_column(vec![2.0, 6.0, -2.5, 0.0]);
    let vec_trans: Vector<f32> = vec.transpose();

    let (m, n): (usize, usize) = vec_trans.dim();

    assert_eq!((n_ref, m_ref), (m, n));
}