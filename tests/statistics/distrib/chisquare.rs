use mathru::statistics::distrib::{ChiSquare, Continuous};

#[test]
fn pdf0()
{
    let df: u32 = 4;
    let distrib: ChiSquare<f64> = ChiSquare::new(df);
    let x: f64 = 0.0;
    let prob: f64 = distrib.pdf(x);

    assert_relative_eq!(0.0, prob);
}

#[test]
fn pdf1()
{
    let df: u32 = 4;
    let distrib: ChiSquare<f64> = ChiSquare::new(df);
    let x: f64 = 1.0;
    let prob: f64 = distrib.pdf(x);

    assert_relative_eq!(0.1516326649281583, prob);
}

#[test]
fn cdf0()
{
    let df: u32 = 4;
    let distrib: ChiSquare<f64> = ChiSquare::new(df);

    assert_relative_eq!(0.09020401043104986, distrib.cdf(1.0))
}

#[test]
fn cdf1()
{
    let df: u32 = 4;
    let distrib: ChiSquare<f64> = ChiSquare::new(df);

    assert_relative_eq!(0.001209104274250028, distrib.cdf(0.1))
}

#[test]
fn cdf2()
{
    let df: u32 = 4;
    let distrib: ChiSquare<f64> = ChiSquare::new(df);

    assert_relative_eq!(0.7689217620241717, distrib.cdf(5.6))
}

//Very inaccurate
#[test]
fn cdf3()
{
    let df: u32 = 3;

    let distrib: ChiSquare<f64> = ChiSquare::new(df);

    assert_relative_eq!(0.08110858834532447, distrib.cdf(0.5), epsilon=3.0 * f64::EPSILON);
}

//Very inaccurate
#[test]
fn cdf4()
{
    let df: u32 = 3;

    let distrib: ChiSquare<f64> = ChiSquare::new(df);

    assert_relative_eq!(0.9539882943107686, distrib.cdf(8.0), epsilon=3.0 * f64::EPSILON);
}

#[test]
fn quantile()
{
    let df: u32 = 4;

    let distrib: ChiSquare<f64> = ChiSquare::new(df);

    assert_relative_eq!(7.779440339734858, distrib.quantile(0.9))
}

#[test]
fn quantile_1()
{
    let df: u32 = 4;

    let distrib: ChiSquare<f64> = ChiSquare::new(df);

    assert_relative_eq!(0.7107230213973239, distrib.quantile(0.05), epsilon=3.0 * f64::EPSILON )
}

#[test]
fn quantile_2()
{
    let df: u32 = 7;

    let distrib: ChiSquare<f64> = ChiSquare::new(df);

    assert_relative_eq!(14.067140449340169, distrib.quantile(0.95), max_relative=0.00001)
}

#[test]
fn quantile_3()
{
    let df: u32 = 7;

    let distrib: ChiSquare<f64> = ChiSquare::new(df);

    assert_relative_eq!(2.167349909298057, distrib.quantile(0.05), max_relative=0.00001)
}

#[test]
fn quantile_4()
{
    let df: u32 = 10;

    let distrib: ChiSquare<f64> = ChiSquare::new(df);

    assert_relative_eq!(18.307038053275146, distrib.quantile(0.95), epsilon=80.0 * f64::EPSILON)
}
