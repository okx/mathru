use mathru::statistics::distrib::{Discrete, Poisson};

#[test]
fn pmf0()
{
    let gamma: f64 = 1.0;
    let x: u32 = 0;
    let distrib: Poisson<f64> = Poisson::new(&gamma);
    let prob: f64 = distrib.pmf(x);

    assert_relative_eq!(0.36787944117144233, prob, epsilon=1.0e-10);
}

#[test]
fn pmf1()
{
    let gamma: f64 = 3.0;
    let x: u32 = 5;
    let distrib: Poisson<f64> = Poisson::new(&gamma);
    let prob: f64 = distrib.pmf(x);

    assert_relative_eq!(0.10081881344492448, prob, epsilon=1.0e-10);
}

#[test]
fn cdf1()
{
    let gamma: f64 = 5.0;
    let x: u32 = 5;
    let distrib: Poisson<f64> = Poisson::new(&gamma);
    let prob: f64 = distrib.cdf(x);

    assert_relative_eq!(0.6159606548330621, prob, epsilon=1.0e-10);
}

