use mathru::statistics::distrib::{Bernoulli, Discrete};

#[test]
fn pmf0()
{
    let p: f64 = 0.1;
    let distrib: Bernoulli<f64> = Bernoulli::new(p);
    let k: u8 = 0;
    let prob: f64 = distrib.pmf(k);

    assert_eq!(1.0 - p, prob);
}

#[test]
fn pmf1()
{
    let p: f64 = 0.1;
    let distrib: Bernoulli<f64> = Bernoulli::new(p);
    let k: u8 = 1;
    let prob: f64 = distrib.pmf(k);

    assert_eq!(p, prob);
}

#[test]
fn cdf0()
{
    let p: f64 = 0.1;
    let distrib: Bernoulli<f64> = Bernoulli::new(p);
    let x: f64 = -1.0;
    let prob: f64 = distrib.cdf(x);

    assert_eq!(0.0, prob);
}

#[test]
fn cdf1()
{
    let p: f64 = 0.1;
    let distrib: Bernoulli<f64> = Bernoulli::new(p);
    let x: f64 = 1.0;
    let prob: f64 = distrib.cdf(x);

    assert_eq!(1.0, prob);
}

#[test]
fn cdf2()
{
    let p: f64 = 0.1;
    let distrib: Bernoulli<f64> = Bernoulli::new(p);
    let x: f64 = 0.3;
    let prob: f64 = distrib.cdf(x);

    assert_eq!(1.0 - p, prob);
}

#[test]
fn mean()
{
    let p: f64 = 0.1;
    let distrib: Bernoulli<f64> = Bernoulli::new(p);
    let mean: f64 = distrib.mean();

    assert_eq!(p, mean);
}

#[test]
fn variance()
{
    let p: f64 = 0.1;
    let distrib: Bernoulli<f64> = Bernoulli::new(p);
    let variance: f64 = distrib.variance();

    assert_eq!(p * (1.0 - p), variance);
}
