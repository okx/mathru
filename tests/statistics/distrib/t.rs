use mathru::statistics::distrib::{Continuous, T};

#[test]
fn pdf0()
{
    let n: f64 = 2.0;
    let x: f64 = 0.0;

    let t: T<f64> = T::new(n);

    assert_relative_eq!(0.3535, t.pdf(x), epsilon=0.0001);
}

#[test]
fn pdf1()
{
    let n: f64 = 5.0;
    let x: f64 = -1.0;

    let t: T<f64> = T::new(n);

    assert_relative_eq!(0.2196, t.pdf(x), epsilon=0.0001);
}

#[test]
fn pdf2()
{
    let n: f64 = 7.753;
    let x: f64 = -0.7559;

    let t: T<f64> = T::new(n);

    assert_relative_eq!(0.283, t.pdf(x), epsilon=0.001);
}

#[test]
fn cdf0()
{
    let n: f64 = 2.0;
    let x: f64 = 0.0;

    let t: T<f64> = T::new(n);

    assert_relative_eq!(0.5, t.cdf(x), epsilon=0.001);
}

#[test]
fn cdf1()
{
    let n: f64 = 2.0;
    let x: f64 = -0.5;

    let t: T<f64> = T::new(n);

    assert_relative_eq!(0.3333, t.cdf(x), epsilon=0.0001);
}



#[test]
#[should_panic(expected = "Skewness is not defined if degrees of freedom is smaller or equal 3")]
fn skewness()
{
    let n: f64 = 3.0;

    let t: T<f64> = T::new(n);

    assert_abs_diff_eq!(0.0, t.skewness(), epsilon=0.0001);
}

#[test]
fn median()
{
    let n: f64 = 3.0;

    let t: T<f64> = T::new(n);

    assert_abs_diff_eq!(0.0, t.median(), epsilon=0.0001);
}
