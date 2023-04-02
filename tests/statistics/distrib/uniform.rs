use mathru::statistics::distrib::{Continuous, Uniform};

#[test]
fn pdf_lower_a()
{
    let a: f64 = -0.2;
    let b: f64 = 0.4;
    let x: f64 = -0.3;

    let distrib: Uniform<f64> = Uniform::new(a, b);

    assert_abs_diff_eq!(0.0, distrib.pdf(x));
}

#[test]
fn pdf_higher_b()
{
    let a: f64 = -0.2;
    let b: f64 = 0.4;
    let x: f64 = 0.5;

    let distrib: Uniform<f64> = Uniform::new(a, b);

    assert_abs_diff_eq!(0.0, distrib.pdf(x));
}

#[test]
fn pdf()
{
    let a: f64 = -0.2;
    let b: f64 = 0.4;
    let x: f64 = 0.3;

    let distrib: Uniform<f64> = Uniform::new(a, b);

    assert_relative_eq!(1.6666666666666665, distrib.pdf(x));
}

#[test]
fn cdf_lower_a()
{
    let a: f64 = -0.2;
    let b: f64 = 0.4;
    let x: f64 = -0.3;

    let distrib: Uniform<f64> = Uniform::new(a, b);

    assert_abs_diff_eq!(0.0, distrib.cdf(x));
}

#[test]
fn cdf_higher_b()
{
    let a: f64 = -0.2;
    let b: f64 = 0.4;
    let x: f64 = 0.5;

    let distrib: Uniform<f64> = Uniform::new(a, b);

    assert_relative_eq!(1.0, distrib.cdf(x));
}

#[test]
fn cdf()
{
    let a: f64 = -0.2;
    let b: f64 = 0.4;
    let x: f64 = 0.3;

    let distrib: Uniform<f64> = Uniform::new(a, b);

    assert_relative_eq!(0.8333333333333333, distrib.cdf(x));
}

#[test]
fn quantile()
{
    let a: f64 = -0.2;
    let b: f64 = 0.4;
    let x: f64 = 0.3;

    let distrib: Uniform<f64> = Uniform::new(a, b);
    let q: f64 = distrib.cdf(x);
    assert_relative_eq!(0.8333333333333333, q);
    assert_relative_eq!(x, distrib.quantile(q));
}

#[test]
fn mean()
{
    let a: f64 = -0.2;
    let b: f64 = 0.4;

    let distrib: Uniform<f64> = Uniform::new(a, b);

    assert_relative_eq!((a + b) / 2.0, distrib.mean());
}

#[test]
fn variance()
{
    let a: f64 = -0.2;
    let b: f64 = 0.4;
    let diff: f64 = b - a;

    let distrib: Uniform<f64> = Uniform::new(a, b);

    assert_relative_eq!(diff * diff / 12.0, distrib.variance());
}

#[test]
fn skewness()
{
    let a: f64 = -0.2;
    let b: f64 = 0.4;

    let distrib: Uniform<f64> = Uniform::new(a, b);

    assert_relative_eq!(0.0, distrib.skewness());
}

#[test]
fn entropy()
{
    let a: f64 = 0.2;
    let b: f64 = 0.5;
    let distrib: Uniform<f64> = Uniform::new(a, b);
    let entropy: f64 = distrib.entropy();
    assert_relative_eq!((b - a).ln(), entropy);
}
