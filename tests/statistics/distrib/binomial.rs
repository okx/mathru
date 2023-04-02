use mathru::statistics::distrib::{Binomial, Discrete};

#[test]
fn pmf0()
{
    let p: f64 = 0.1;
    let n: u32 = 5;
    let distrib: Binomial<f64> = Binomial::new(n, p);
    let k: u32 = 0;
    let p: f64 = distrib.pmf(k);

    assert_relative_eq!(0.5904900000000001, p);
}

#[test]
fn pmf1()
{
    let p: f64 = 0.1;
    let n: u32 = 3;
    let distrib: Binomial<f64> = Binomial::new(n, p);
    let k: u32 = 0;
    let prob: f64 = distrib.pmf(k);

    assert_relative_eq!(0.7290000000000001, prob);
}

#[test]
fn pmf2()
{
    let p: f64 = 0.2;
    let n: u32 = 2;
    let distrib: Binomial<f64> = Binomial::new(n, p);
    let k: u32 = 1;
    let prob: f64 = distrib.pmf(k);

    assert_relative_eq!(0.32000000000000006, prob);
}

#[test]
fn pmf3()
{
    let p: f64 = 0.1;
    let n: u32 = 5;
    let distrib: Binomial<f64> = Binomial::new(n, p);
    let k: u32 = 2;
    let prob: f64 = distrib.pmf(k);

    assert_relative_eq!(0.07290000000000002, prob);
}

#[test]
fn cdf0()
{
    let p: f64 = 0.1;
    let n: u32 = 2;
    let distrib: Binomial<f64> = Binomial::new(n, p);
    let k: f64 = 0.0;
    let prob: f64 = distrib.cdf(k);

    assert_relative_eq!(0.81, prob);
}

#[test]
fn cdf1()
{
    let p: f64 = 0.1;
    let n: u32 = 2;
    let distrib: Binomial<f64> = Binomial::new(n, p);
    let k: f64 = 1.0;
    let prob: f64 = distrib.cdf(k);

    assert_relative_eq!(0.9900000000000001, prob);
}

#[test]
fn cdf2()
{
    let p: f64 = 0.1;
    let n: u32 = 2;
    let distrib: Binomial<f64> = Binomial::new(n, p);
    let k: f64 = 3.0;
    let prob: f64 = distrib.cdf(k);

    assert_relative_eq!(1.0, prob);
}
