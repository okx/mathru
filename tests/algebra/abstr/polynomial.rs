use mathru::algebra::abstr::Polynomial;
use crate::mathru::algebra::abstr::Zero;

#[test]
fn fmt_constant()
{
    let poly: Polynomial<f64> = Polynomial::from_coef(vec![1.0]);

    let output = format!("{}", poly);

    assert_eq!("1", output)
}

#[test]
fn fmt_coefficient_zero()
{
    let poly: Polynomial<f64> = Polynomial::from_coef(vec![1.0, 0.0, 1.0]);

    let output = format!("{}", poly);

    assert_eq!("1 + 1x^2", output)
}

#[test]
fn fmt_start_zero_term()
{
    let poly: Polynomial<f64> = Polynomial::from_coef(vec![1.0, 0.0]);

    let output = format!("{}", poly);

    assert_eq!("1x", output)
}

#[test]
fn fmt()
{
    let poly: Polynomial<f64> = Polynomial::from_coef(vec![3.0, 2.0, 1.0]);

    let output = format!("{}", poly);

    assert_eq!("1 + 2x + 3x^2", output)
}

#[test]
fn from_root_1()
{
    let poly: Polynomial<f64> = Polynomial::from_root(vec![1.0]);
    let poly_ref: Polynomial<f64> = Polynomial::from_coef(vec![1.0, -1.0]);

    assert_eq!(poly_ref, poly)
}

#[test]
fn from_root_2()
{
    let poly: Polynomial<f64> = Polynomial::from_root(vec![2.0, 1.0]);
    let poly_ref: Polynomial<f64> = Polynomial::from_coef( vec![1.0, -3.0, 2.0]);

    assert_eq!(poly_ref, poly)
}

#[test]
fn eval_degree_0()
{
    let poly: Polynomial<f64> = Polynomial::from_coef(vec![1.0]);

    let value = poly.eval(2.0);

    assert_eq!(1.0, value)
}

#[test]
fn eval_degree_odd()
{
    let poly: Polynomial<f64> = Polynomial::from_coef(vec![3.0, 2.0, 1.0]);

    let value = poly.eval(2.0);

    assert_eq!(17.0, value)
}

#[test]
fn eval_degree_even()
{
    let poly: Polynomial<f64> = Polynomial::from_coef(vec![2.0, 1.0]);

    let value = poly.eval(2.0);

    assert_eq!(5.0, value)
}

#[test]
fn add_with_zero()
{
    let a: Polynomial<f64> = Polynomial::from_coef(vec![3.0, 2.0, 1.0]);
    let b: Polynomial<f64> = Polynomial::from_coef(vec![0.0]);

    assert_eq!(a, &a + &b)
}

#[test]
fn add()
{
    let a: Polynomial<f64> = Polynomial::from_coef(vec![3.0, 2.0, 1.0]);
    let b: Polynomial<f64> = Polynomial::from_coef(vec![2.0, 1.0]);
    let c: Polynomial<f64> = Polynomial::from_coef(vec![3.0, 4.0, 2.0]);

    assert_eq!(c, &a + &b)
}

#[test]
fn sub_with_zero()
{
    let a: Polynomial<f64> = Polynomial::from_coef(vec![3.0, 2.0, 1.0]);
    let b: Polynomial<f64> = Polynomial::from_coef(vec![0.0]);

    assert_eq!(a, &a - &b)
}

#[test]
fn sub()
{
    let a: Polynomial<f64> = Polynomial::from_coef(vec![3.0, 2.0, 1.0]);
    let b: Polynomial<f64> = Polynomial::from_coef(vec![2.0, 1.0]);
    let c: Polynomial<f64> = Polynomial::from_coef(vec![3.0, 0.0, 0.0]);

    assert_eq!(c, &a - &b)
}

#[test]
fn sub2()
{
    let a: Polynomial<f64> = Polynomial::from_coef(vec![3.0, 2.0, 1.0]);
    let b: Polynomial<f64> = Polynomial::from_coef(vec![2.0, 1.0]);
    let c: Polynomial<f64> = Polynomial::from_coef(vec![-3.0, 0.0, 0.0]);

    assert_eq!(c, &b - &a)
}


#[test]
fn neg_ref()
{
    let a: Polynomial<f64> = Polynomial::from_coef(vec![1.0, 2.0, 3.0]);
    let c: Polynomial<f64> = Polynomial::from_coef(vec![-1.0, -2.0, -3.0]);

    assert_eq!(c, -&a)
}

#[test]
fn neg()
{
    let a: Polynomial<f64> = Polynomial::from_coef(vec![1.0, 2.0, 3.0]);
    let c: Polynomial<f64> = Polynomial::from_coef(vec![-1.0, -2.0, -3.0]);

    assert_eq!(c, -a)
}

#[test]
fn degree()
{
    let a: Polynomial<f64> = Polynomial::from_coef(vec![1.0, 2.0, 3.0]);

    assert_eq!(2, a.degree())
}


#[test]
fn mul_with_degree_zero()
{
    let a: Polynomial<f64> = Polynomial::from_coef(vec![1.0, 2.0, 3.0]);
    let b: Polynomial<f64> = Polynomial::from_coef(vec![0.0]);
    let c: Polynomial<f64> = Polynomial::from_coef(vec![0.0, 0.0, 0.0]);

    assert_eq!(c, &a * &b)
}

#[test]
fn mul_with_degree_one()
{
    let a: Polynomial<f64> = Polynomial::from_coef(vec![1.0, 2.0, 3.0]);
    let b: Polynomial<f64> = Polynomial::from_coef(vec![1.0, 1.0]);
    let c: Polynomial<f64> = Polynomial::from_coef(vec![1.0, 3.0, 5.0, 3.0]);

    assert_eq!(c, &a * &b)
}

#[test]
fn reduce()
{
    let a: Polynomial<f64> = Polynomial::from_coef(vec![0.0, 0.0, 1.0, 0.0, 2.0, 1.0]);
    let reduced: Polynomial<f64> = Polynomial::from_coef(vec![1.0, 0.0, 2.0, 1.0]);

    assert_eq!(reduced, a.reduce())
}

#[test]
fn div_1()
{
    let a: Polynomial<f64> = Polynomial::from_coef(vec![1.0, 2.0, 3.0]);
    let b: Polynomial<f64> = Polynomial::from_coef(vec![1.0, 1.0]);

    assert_eq!(Polynomial::from_coef(vec![0.0]), (&b / &a).0);
    assert_eq!(b, (&b / &a).1);
}

#[test]
fn div_2()
{
    let a: Polynomial<f64> = Polynomial::from_coef(vec![1.0, 2.0, 3.0]);
    let b: Polynomial<f64> = Polynomial::from_coef(vec![1.0, 1.0]);
    let c: Polynomial<f64> = Polynomial::from_coef(vec![1.0, 3.0, 5.0, 3.0]);

    assert_eq!(b, (&c / &a).0);
    assert_eq!(Polynomial::zero(), (&c / &a).1)
}

#[test]
fn div_own()
{
    let a: Polynomial<f64> = Polynomial::from_coef(vec![1.0, 2.0, 3.0]);
    let b: Polynomial<f64> = Polynomial::from_coef(vec![1.0, 1.0]);

    assert_eq!(Polynomial::from_coef(vec![0.0]), (b.clone() / a.clone()).0);
    assert_eq!(b.clone(), (b / a).1);
}

#[test]
fn div_with_remainder()
{
    let a: Polynomial<f64> = Polynomial::from_coef(vec![3.0, 2.0, 1.0]);
    let b: Polynomial<f64> = Polynomial::from_coef(vec![1.0, 1.0]);
    let c: Polynomial<f64> = Polynomial::from_coef(vec![3.0, 5.0, 3.0, 0.0]);

    assert_eq!(b, (&c / &a).0);
    assert_eq!(Polynomial::from_coef(vec![-1.0]), (&c / &a).1)
}

#[test]
fn differentiate_degree_zero()
{
    let c: Polynomial<f64> = Polynomial::from_coef(vec![1.0]);
    let c_s: Polynomial<f64> = Polynomial::from_coef(vec![0.0]);

    assert_eq!(c_s, c.differentiate());
}

#[test]
fn differentiate_degree_one()
{
    let c: Polynomial<f64> = Polynomial::from_coef(vec![3.0, 1.0]);
    let c_s: Polynomial<f64> = Polynomial::from_coef(vec![3.0]);

    assert_eq!(c_s, c.differentiate());
}

#[test]
fn differentiate_general()
{
    let c: Polynomial<f64> = Polynomial::from_coef(vec![3.0, 5.0, 3.0, 1.0]);
    let c_s: Polynomial<f64> = Polynomial::from_coef(vec![9.0, 10.0, 3.0]);

    assert_eq!(c_s, c.differentiate());
}

#[test]
fn integrate_degree_zero_null()
{
    let c: Polynomial<f64> = Polynomial::from_coef(vec![0.0]);
    let c_s: Polynomial<f64> = Polynomial::from_coef(vec![0.0]);

    assert_eq!(c_s, c.integrate());
}

#[test]
fn integrate_degree_zero_not_null()
{
    let c: Polynomial<f64> = Polynomial::from_coef(vec![1.0]);
    let c_s: Polynomial<f64> = Polynomial::from_coef(vec![1.0, 0.0]);

    assert_eq!(c_s, c.integrate());
}

#[test]
fn integrate_general()
{
    let c: Polynomial<f64> = Polynomial::from_coef(vec![3.0, 2.0, 1.0]);
    let c_s: Polynomial<f64> = Polynomial::from_coef(vec![1.0, 1.0, 1.0, 0.0]);

    assert_eq!(c_s, c.integrate());
}

#[test]
fn from_legendre_degree0()
{
    let p: Polynomial<f64> = Polynomial::from_legendre(0);
    let p_ref = Polynomial::from_coef(vec![1.0f64]);

    assert_eq!(p_ref, p);
}

#[test]
fn from_legendre_degree1()
{
    let p: Polynomial<f64> = Polynomial::from_legendre(1);
    let p_ref = Polynomial::from_coef(vec![1.0f64, 0.0f64]);

    assert_eq!(p_ref, p);
}

#[test]
fn from_legendre_degree2()
{
    let p: Polynomial<f64> = Polynomial::from_legendre(2);
    let p_ref = Polynomial::from_coef(vec![1.5f64, 0.0f64, -0.5f64]);

    assert_eq!(p_ref, p);
}

#[test]
fn from_chebyshev_t_0()
{
    let p: Polynomial<f64> = Polynomial::from_chebyshev_t(0);
    let p_ref = Polynomial::from_coef(vec![1.0]);

    assert_eq!(p_ref, p);
}

#[test]
fn from_chebyshev_t_1()
{
    let p: Polynomial<f64> = Polynomial::from_chebyshev_t(1);
    let p_ref = Polynomial::from_coef(vec![1.0, 0.0]);

    assert_eq!(p_ref, p);
}

#[test]
fn from_chebyshev_t_2()
{
    let p: Polynomial<f64> = Polynomial::from_chebyshev_t(2);
    let p_ref = Polynomial::from_coef(vec![2.0, 0.0, -1.0]);

    assert_eq!(p_ref, p);
}

#[test]
fn from_chebyshev_u_0()
{
    let p: Polynomial<f64> = Polynomial::from_chebyshev_u(0);
    let p_ref = Polynomial::from_coef(vec![1.0]);

    assert_eq!(p_ref, p);
}

#[test]
fn from_chebyshev_u_1()
{
    let p: Polynomial<f64> = Polynomial::from_chebyshev_u(1);
    let p_ref = Polynomial::from_coef(vec![2.0, 0.0]);

    assert_eq!(p_ref, p);
}

#[test]
fn from_chebyshev_u_2()
{
    let p: Polynomial<f64> = Polynomial::from_chebyshev_u(2);
    let p_ref = Polynomial::from_coef(vec![4.0, 0.0, -1.0]);

    assert_eq!(p_ref, p);
}