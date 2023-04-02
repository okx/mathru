use mathru::statistics::distrib::{Discrete, Multinomial};
use mathru::algebra::linear::{Vector};

#[test]
fn pmf0()
{
    let p: Vector<f64> = vector![0.3; 0.7];
    let distrib: Multinomial<f64> = Multinomial::new(p);
    let x: Vector<u32> = vector![3; 7];
    let prob: f64 = distrib.pmf(x);

    assert_relative_eq!(prob, 0.2668 , epsilon=0.0001);
}

#[test]
fn pmf1()
{
    let p: Vector<f64> = vector![0.2; 0.3; 0.5];
    let n: Vector<u32> = vector![1; 2; 3];
    let distrib : Multinomial<f64> = Multinomial::new(p);
    let prob: f64 = distrib.pmf(n);

    assert_relative_eq!(prob, 0.135, epsilon=0.0001);
}

//    #[test]
//    fn pmf2()
//    {
//        let p: f64 = 0.2;
//        let n: u32 = 2;
//        let distrib : Multinomial<f64> = Multinomial::new(&n, &p);
//        let k: u32 = 1;
//        let prob: f64 = distrib.pmf(k);
//
//        assert_eq!(0.32000000000000006, prob);
//    }
//
//    #[test]
//    fn pmf3()
//    {
//        let p : f64 = 0.1;
//        let n : u32 = 5;
//        let distrib : Multinomial = Multinomial::new(&n, &p);
//        let k : u32 = 2;
//        let prob : f64 = distrib.pmf(k);
//
//        assert_eq!(0.07290000000000002, prob);
//    }
//
//    #[test]
//    fn cdf0()
//    {
//        let p : f64 = 0.1;
//        let n : u32 = 2;
//        let distrib : Multinomial = Multinomial::new(&n, &p);
//        let k : f64 = 0.0;
//        let prob : f64 = distrib.cdf(k);
//
//        assert_eq!(0.81, prob);
//    }
//
//    #[test]
//    fn cdf1()
//    {
//        let p : f64 = 0.1;
//        let n : u32 = 2;
//        let distrib : Multinomial = Multinomial::new(&n, &p);
//        let k : f64 = 1.0;
//        let prob : f64 = distrib.cdf(k);
//
//        assert_eq!(0.9900000000000001, prob);
//    }
//
//    #[test]
//    fn cdf2()
//    {
//        let p : f64 = 0.1;
//        let n : u32 = 2;
//        let distrib : Multinomial = Multinomial::new(&n, &p);
//        let k : f64 = 3.0;
//        let prob : f64 = distrib.cdf(k);
//
//        assert_eq!(1.0, prob);
//    }
