use mathru::special::gamma;
use crate::mathru::algebra::abstr::Real;
use std::f64::consts::PI as f64_PI;
use std::f32::consts::PI as f32_PI;


#[test]
fn gamma_0()
{
    let x: f64 = 0.5;

    let gamma: f64 = gamma::gamma(x);

    assert_relative_eq!(f64_PI.sqrt(), gamma, epsilon=3.0 * f64::EPSILON);
}

#[test]
fn gamma_1()
{
    let x: f64 = -0.5;

    let gamma: f64 = gamma::gamma(x);

    assert_relative_eq!(-2.0 * f64_PI.sqrt(), gamma, epsilon=15.0 * f64::EPSILON);
}

#[test]
fn gamma_2()
{
    let x: f32 = -0.5;

    let gamma: f32 = gamma::gamma(x);

    assert_relative_eq!(-2.0 * f32_PI.sqrt(), gamma, epsilon=5.0 * f32::EPSILON)
}

#[test]
fn gamma_3()
{
    let x: f64 = 1.0;

    let gamma: f64 = gamma::gamma(x);

    assert_relative_eq!(1.0, gamma);
}

#[test]
fn gamma_4()
{
    let x: f64 = 2.0;

    let gamma: f64 = gamma::gamma(x);

    assert_relative_eq!(1.0, gamma, epsilon=3.0 * f64::EPSILON);
}

#[test]
fn gamma_5()
{
    let x: f64 = 3.0;

    let gamma: f64 = gamma::gamma(x);

    assert_relative_eq!(2.0, gamma, epsilon=15.0 * f64::EPSILON);
}

#[test]
fn gamma_6()
{
    let x: f64 = 4.0;

    let gamma: f64 = gamma::gamma(x);

    assert_relative_eq!(6.0, gamma, epsilon=40.0 * f64::EPSILON);
}

#[test]
fn gamma_7()
{
    let x: f64 = 6.0;

    let gamma: f64 = gamma::gamma(x);

    assert_relative_eq!(120.00000000000026, gamma, epsilon=10.0 * f64::EPSILON);
}

#[test]
fn digamma_0()
{
    let x: f64 = 1.0;

    let digamma: f64 = gamma::digamma(x);

    assert_relative_eq!(-f64::euler_gamma(), digamma, epsilon=2.0 * f64::EPSILON);
}

#[test]
fn digamma_1()
{
    let x: f64 = 0.5;

    let digamma: f64 = gamma::digamma(x);

    assert_relative_eq!(- 2.0 * 2.0_f64.ln() - f64::euler_gamma(), digamma);
}

#[test]
fn digamma_2()
{
    let x: f64 = 0.25;

    let digamma: f64 = gamma::digamma(x);

    assert_relative_eq!(-f64_PI / 2.0 - 3.0 * 2.0_f64.ln() - f64::euler_gamma(), digamma);
}

#[test]
fn gamma_ur0()
{
    let x: f64 = 1.5;
    let a: f64 = 1.0;

    let gamma: f64 = gamma::gamma_ur(a, x);

    assert_relative_eq!(gamma::gamma_u(a, x) / gamma::gamma(a), gamma);
}

#[test]
fn gamma_lr0()
{
    let x: f64 = 1.5;
    let a: f64 = 1.0;

    let gamma: f64 = gamma::gamma_lr(a, x);

    assert_relative_eq!(gamma::gamma_l(a, x) / gamma::gamma(a), gamma);
}

#[test]
fn gamma_ur_inv_0()
{
    let x: f64 = 0.5;
    let a: f64 = 0.5;

    let y: f64 = gamma::gamma_ur(a, x);

    assert_relative_eq!(gamma::gamma_ur_inv(a, y), x);
}

#[test]
fn gamma_ur_inv_a_is_one()
{
    let x: f64 = 0.4;
    let a: f64 = 1.0;

    let y: f64 = gamma::gamma_ur(a, x);

    assert_relative_eq!(gamma::gamma_ur_inv(a, y), x);
}

#[test]
fn gamma_ur_inv_x_0_first()
{
    let x: f64 = 0.5;
    let a: f64 = 0.3;

    let y: f64 = gamma::gamma_ur(a, x);

    assert_relative_eq!(gamma::gamma_ur_inv(a, y), x);
}

#[test]
fn gamma_ur_inv_1()
{
    let x: f64 = 0.2;
    let a: f64 = 2.0;

    let y: f64 = gamma::gamma_ur(a, x);

    assert_relative_eq!(gamma::gamma_ur_inv(a, y), x);
}

#[test]
fn gamma_ur_inv_2()
{
    let x: f64 = 0.1;
    let a: f64 = 2.0;

    let y: f64 = gamma::gamma_ur(a, x);

    assert_relative_eq!(gamma::gamma_ur_inv(a, y), x, epsilon=2.0 * f64::EPSILON);
}

#[test]
fn gamma_l0()
{
    let x: f64 = 1.5;
    let a: f64 = 1.0;

    let gamma: f64 = gamma::gamma_l(a, x);

    assert_relative_eq!(1.0 - (-x).exp(), gamma);
}

#[test]
fn gamma_u0()
{
    let x: f64 = 1.5;
    let a: f64 = 1.0;

    let gamma: f64 = gamma::gamma_u(a, x);

    assert_relative_eq!((-x).exp(), gamma);
}