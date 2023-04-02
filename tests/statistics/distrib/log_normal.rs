use mathru::statistics::distrib::{Continuous, LogNormal};
use std::f64::consts::{PI};

#[test]
fn pdf_negative()
{
    let mu: f64 = 0.0;
    let sigma_squared: f64 = 0.25;
    let distrib: LogNormal<f64> = LogNormal::new(mu, sigma_squared);
    let x: f64 = -1.0;
    let prob: f64 = distrib.pdf(x);

    assert_relative_eq!(0.0, prob);
}

#[test]
fn pdf0()
{
    let mu: f64 = 0.0;
    let sigma_squared: f64 = 0.25 * 0.25;
    let distrib: LogNormal<f64> = LogNormal::new(mu, sigma_squared);
    let x: f64 = 1.0;
    let prob: f64 = distrib.pdf(x);

    assert_relative_eq!(prob, 1.5957691216057308);
}

//Does not work all the time, because the used function random is not mocked.
// #[test]
// fn random()
// {
//    let mu_1 : f64 = 0.0;
//    let variance_1: f64 = 1.0;
//    let distrib_1 : Normal<f64> = Normal::new(mu_1, variance_1);
//    let mut data: Vec<f64> = Vec::new();
//
//    for _i in 0..10000
//    {
//        data.push(distrib_1.random());
//    }
//
//    let distrib_2: Normal<f64> = Normal::from_data(&data);
//
//    assert!(distrib_2.mu() < mu_1 + 0.01);
//    assert!(distrib_2.mu() > mu_1 - 0.01);
//    assert!(distrib_2.variance() < 1.02 * variance_1);
//    assert!(distrib_2.variance() > 0.98 * variance_1);
// }
#[test]
fn cdf_negative()
{
    let mu: f64 = 0.0;
    let sigma_squared: f64 = 1.0;
    let distrib: LogNormal<f64> = LogNormal::new(mu, sigma_squared);

    assert_relative_eq!(0.0, distrib.cdf(-1.0))
}

#[test]
fn cdf_zero()
{
    let mu: f64 = 0.0;
    let sigma_squared: f64 = 1.0;
    let distrib: LogNormal<f64> = LogNormal::new(mu, sigma_squared);

    assert_relative_eq!(0.0, distrib.cdf(0.0))
}

#[test]
fn cdf0()
{
    let mu: f64 = 0.0;
    let sigma_squared: f64 = 1.0;
    let distrib: LogNormal<f64> = LogNormal::new(mu, sigma_squared);

    assert_relative_eq!(0.5, distrib.cdf(1.0))
}

#[test]
fn cdf1()
{
    let mu: f64 = 0.0;
    let sigma_squared: f64 = 100.0;
    let distrib: LogNormal<f64> = LogNormal::new(mu, sigma_squared);

    assert_relative_eq!(0.5, distrib.cdf(1.0))
}


#[test]
fn quantile0()
{
    let mu: f64 = 0.0;
    let variance: f64 = 1.0;
    let distrib: LogNormal<f64> = LogNormal::new(mu, variance);

    assert_relative_eq!(1.0, distrib.quantile(0.5));
}

#[test]
fn quantile1()
{
    let mu: f64 = 0.0;
    let variance: f64 = 1.0;
    let distrib: LogNormal<f64> = LogNormal::new(mu, variance);

    assert_relative_eq!(3.6022244792791582, distrib.quantile(0.9));
}

#[test]
fn mean()
{
    let mu: f64 = 1.0;
    let sigma_squared: f64 = 0.5;
    let distrib: LogNormal<f64> = LogNormal::new(mu, sigma_squared);

    assert_relative_eq!((mu + sigma_squared / 2.0).exp(), distrib.mean());
}

#[test]
fn median()
{
    let mu: f64 = 1.0;
    let sigma_squared: f64 = 0.5;
    let distrib: LogNormal<f64> = LogNormal::new(mu, sigma_squared);

    assert_relative_eq!(mu.exp(), distrib.median());
}

#[test]
fn variance()
{
    let mu: f64 = 1.0;
    let sigma_squared: f64 = 0.5;
    let distrib: LogNormal<f64> = LogNormal::new(mu, sigma_squared);

    assert_relative_eq!((sigma_squared.exp() - 1.0) * (2.0 * mu + sigma_squared).exp(), distrib.variance());
}
//
#[test]
fn skewness()
{
    let mu: f64 = 1.0;
    let sigma_squared: f64 = 0.5;
    let distrib: LogNormal<f64> = LogNormal::new(mu, sigma_squared);

    assert_relative_eq!((sigma_squared.exp() + 2.0) * (sigma_squared.exp() - 1.0).sqrt() , distrib.skewness());
}


#[test]
fn entropy()
{
    let mu: f64 = 1.0;
    let sigma_squared: f64 = 0.5;
    let distrib: LogNormal<f64> = LogNormal::new(mu, sigma_squared);

    assert_relative_eq!((sigma_squared.sqrt() * (mu + 0.5).exp() * (2.0 * PI).sqrt()).log(2.0), distrib.entropy());
}
//
// //    #[test]
// //    fn from_data()
// //    {
// //        let mu: f64 = 5.0;
// //        let variance: f64 = 10.0;
// //        let num_samples: usize = 100;
// //        let data: Vector<f64> = Normal::new(mu,
// // variance).random_vector(num_samples);
// //
// //        let distrib: Normal = Normal::from_data(&data);
// //
// //        assert!((mu - distrib.mu()).abs() < 0.5);
// //        assert!((variance - distrib.variance()) < 1.0);
// //    }
