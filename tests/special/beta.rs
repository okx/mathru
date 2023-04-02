use mathru::special::beta;
use std::f64::consts::PI as f64_PI;
use mathru::elementary::Power;

#[test]
fn beta0_()
{
    let x: f64 = 5.0;
    let y: f64 = 1.0;

    let beta: f64 = beta::beta(x, y);

    assert_relative_eq!(1.0 / x, beta, epsilon=3.0 * f64::EPSILON);
}

#[test]
fn beta_1()
{
    let x: f64 = 0.5;

    let beta: f64 = beta::beta(x, 1.0 - x);

    assert_relative_eq!(f64_PI, beta, epsilon=8.0 * f64::EPSILON);
}

#[test]
fn beta_inc_0()
{
    let x: f64 = 0.7;
    let a: f64 = 1.0;
    let b: f64 = 0.3;

    let beta: f64 = beta::beta_inc(x, a, b);

    assert_relative_eq!(1.0105156602135055, beta, epsilon=8.0 * f64::EPSILON);
}


#[test]
fn beta_inc_reg_0()
{
    let x: f64 = 0.0;
    let p: f64 = 0.5;
    let q: f64 = 0.3;

    let beta: f64 = beta::beta_inc_reg(x, p, q);

    assert_relative_eq!(0.0, beta, epsilon=8.0 * f64::EPSILON);
}

#[test]
fn beta_inc_reg_1()
{
    let x: f64 = 1.0;
    let p: f64 = 0.5;
    let q: f64 = 0.3;

    let beta: f64 = beta::beta_inc_reg(x, p, q);

    assert_relative_eq!(1.0, beta, epsilon=1.0 * f64::EPSILON);
}

#[test]
fn beta_inc_reg_2()
{
    let x: f64 = 0.7;
    let a: f64 = 0.3;
    let b: f64 = 1.0;

    let beta: f64 = beta::beta_inc_reg(x, a, b);

    assert_relative_eq!(x.pow(a), beta, epsilon=1.0 * f64::EPSILON);
}

#[test]
fn beta_inc_reg_3()
{
    let x: f64 = 0.7;
    let a: f64 = 1.0;
    let b: f64 = 0.3;

    let beta: f64 = beta::beta_inc_reg(x, a, b);

    assert_relative_eq!(1.0 - (1.0 - x).pow(b), beta, epsilon=5.0 * f64::EPSILON);
}
