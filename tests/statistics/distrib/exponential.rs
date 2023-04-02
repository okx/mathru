use mathru::statistics::distrib::{Continuous, Exponential};

#[test]
fn pdf0()
{
    let lambda: f64 = 1.0;
    let distrib: Exponential<f64> = Exponential::new(lambda);
    let x: f64 = 1.0;
    let prob: f64 = distrib.pdf(x);

    assert_relative_eq!((-1.0_f64).exp(), prob);
}

#[test]
fn cdf0()
{
    let lambda: f64 = 0.5;
    let distrib: Exponential<f64> = Exponential::new(lambda);

    assert_relative_eq!(1.0 - (-1.0_f64).exp(), distrib.cdf(2.0))
}

#[test]
fn quantile()
{
    let lambda: f64 = 0.5;
    let distrib: Exponential<f64> = Exponential::new(lambda);

    assert_relative_eq!(1.3862943611198906, distrib.quantile(0.5), epsilon=1.0e-10)
}

#[test]
fn skewnes()
{
    let lambda: f64 = 0.5;
    let distrib: Exponential<f64> = Exponential::new(lambda);

    assert_relative_eq!(2.0, distrib.skewness(), epsilon=1.0e-10);
}

#[test]
fn median()
{
    let lambda: f64 = 0.5;
    let distrib: Exponential<f64> = Exponential::new(lambda);

    assert_relative_eq!((2.0f64).ln() / lambda, distrib.median(), epsilon=1.0e-10);
}

#[test]
fn entropy()
{
    let lambda: f64 = 0.5;
    let distrib: Exponential<f64> = Exponential::new(lambda);

    assert_relative_eq!(1.0 - lambda.ln(), distrib.entropy(), epsilon=1.0e-10);
}
