use mathru::statistics::test::{ChiSquare, Test};

#[test]
fn test0()
{
    let x: Vec<f64> = vec![10.0, 20.0, 30.0];
    let y: Vec<f64> = vec![6.0, 9.0, 17.0];
    let test: ChiSquare<f64> = ChiSquare::test_vector(&x, &y);

    assert_relative_eq!(0.27157465150403504, test.value(), epsilon=1.0e-10);
    assert_eq!(2, test.df());
    assert_relative_eq!(0.8730282833800732, test.p_value(), epsilon=1.0e-10);
}
