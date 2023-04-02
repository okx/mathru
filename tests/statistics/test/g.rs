use mathru::statistics::test::G;

#[test]
fn test0()
{
    let x: Vec<f64> = vec![70.0, 79.0, 3.0, 4.0];
    let y: Vec<f64> = vec![0.54, 0.40, 0.05, 0.01];
    let test: G<f64> = G::test_vector(&x, &y);

    assert_relative_eq!( test.g(), 13.1447992204914);
    assert_eq!(3, test.df());
    assert_relative_eq!(test.p_value(), 0.004333706171918306)
}

#[test]
fn test1()
{
    let x: Vec<f64> = vec![1752.0, 1895.0];
    let y: Vec<f64> = vec![0.5, 0.5];
    let test: G<f64> = G::test_vector(&x, &y);

    assert_relative_eq!(test.g(), 5.608511956526968_f64);
    assert_eq!(1, test.df());
}
