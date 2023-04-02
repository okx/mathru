use mathru::statistics::distrib::{Continuous, RaisedCosine};
use std::f64::consts::PI;

#[test]
fn pdf0()
{
    let mu: f64 = 0.0;
    let s: f64 = PI;
    let distrib: RaisedCosine<f64> = RaisedCosine::new(mu, s);
    let x: f64 = PI;
    let prob: f64 = distrib.pdf(x);

    assert_abs_diff_eq!(0.0, prob, epsilon=1.0e-10);
}

#[test]
fn pdf1()
{
    let mu: f64 = 0.0;
    let s: f64 = PI;
    let distrib: RaisedCosine<f64> = RaisedCosine::new(mu, s);
    let x: f64 = 0.0;
    let prob: f64 = distrib.pdf(x);

    assert_relative_eq!(1.0 / PI, prob, epsilon=1.0e-10);
}

#[test]
fn pdf2()
{
    let mu: f64 = PI;
    let s: f64 = 1.0;
    let distrib: RaisedCosine<f64> = RaisedCosine::new(mu, s);
    let x: f64 = PI;
    let prob: f64 = distrib.pdf(x);

    assert_relative_eq!(1.0, prob, epsilon=1.0e-10);
}

#[test]
fn cdf0()
{
    let mu: f64 = PI;
    let s: f64 = 1.0;
    let distrib: RaisedCosine<f64> = RaisedCosine::new(mu, s);
    let x: f64 = PI;
    let prob: f64 = distrib.cdf(x);

    assert_relative_eq!(0.5, prob, epsilon=1.0e-10);
}

#[test]
fn cdf1()
{
    let mu: f64 = PI;
    let s: f64 = 0.1;
    let distrib: RaisedCosine<f64> = RaisedCosine::new(mu, s);
    let x: f64 = mu - s;
    let prob: f64 = distrib.cdf(x);

    assert_abs_diff_eq!(0.0, prob, epsilon=0.00000001);
}

#[test]
fn cdf2()
{
    let mu: f64 = PI;
    let s: f64 = 0.1;
    let distrib: RaisedCosine<f64> = RaisedCosine::new(mu, s);
    let x: f64 = PI + s;
    let prob: f64 = distrib.cdf(x);

    assert_relative_eq!(1.0, prob);
}

#[test]
fn mean()
{
    let mu: f64 = PI;
    let s: f64 = 2.0 * PI;
    let distrib: RaisedCosine<f64> = RaisedCosine::new(mu, s);
    let mean: f64 = distrib.mean();

    assert_relative_eq!(mu, mean, epsilon=1.0e-10);
}

#[test]
fn variance()
{
    let mu: f64 = 0.0;
    let s: f64 = PI;
    let distrib: RaisedCosine<f64> = RaisedCosine::new(mu, s);
    let variance: f64 = distrib.variance();

    assert_relative_eq!(s * s * (1.0 / 3.0 - 2.0 / PI / PI), variance, epsilon=1.0e-10);
}

#[test]
fn median()
{
    let mu: f64 = PI;
    let s: f64 = 2.0 * PI;
    let distrib: RaisedCosine<f64> = RaisedCosine::new(mu, s);

    assert_relative_eq!(mu, distrib.median(), epsilon=1.0e-10);
}

#[test]
fn skewness()
{
    let mu: f64 = PI;
    let s: f64 = 2.0 * PI;
    let distrib: RaisedCosine<f64> = RaisedCosine::new(mu, s);

    assert_abs_diff_eq!(0.0, distrib.skewness(), epsilon=1.0e-10);
}
