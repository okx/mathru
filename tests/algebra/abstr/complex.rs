use mathru::algebra::abstr::{One, Zero};
use mathru::algebra::abstr::{Complex};
use mathru::algebra::abstr::cast::{FromPrimitive};

#[test]
fn arg0() {
    let a: Complex<f64> = Complex::new(-3.5, -6.0);
    let angle : Complex<f64> = Complex::new(-6.0_f64.atan2(-3.5), 0.0_f64);
    assert_eq!(angle, a.arg());
}

#[test]
fn arg1() {
    let phi: Complex<f64> = Complex::new(0.0, 0.0_f64);
    let a: Complex<f64>= Complex::new(0.5, 0.0);
    assert_eq!(phi, a.arg());
}

#[test]
fn add() {
    let a : Complex<f32> = Complex::new(-3.5, 6.5);
    let b : Complex<f32> = Complex::new(2.0, -3.0);
    let sum: Complex<f32> = Complex::new(-1.5, 3.5);
    assert_eq!(sum, a + b);
}

#[test]
fn mul() {
    let a : Complex<f32> = Complex::new(-3.5, 6.5);
    let b : Complex<f32> = Complex::new(2.0, -3.0);
    let prod: Complex<f32> = Complex::new(-3.5 * 2.0 - 6.5 * -3.0, 6.5 * 2.0 + -3.5 * -3.0);
    assert_eq!(prod, a * b);
}

#[test]
fn mulassign() {
    let mut a: Complex<f32> = Complex::new(-3.5, 6.5);
    let b: Complex<f32> = Complex::new(2.0, -3.0);
    let prod: Complex<f32> = a * b;
    a *= b;
    assert_eq!(prod, a);
}

#[test]
fn zero() {
    let reference: Complex<f32> = Complex::new(0.0, 0.0);
    assert_eq!(reference, Complex::zero());
}

#[test]
fn one() {
    let reference: Complex<f64> = Complex::new(1.0, 0.0);
    assert_eq!(reference, Complex::one());
}

#[test]
fn conj() {
    let a_real: f32 = -3.5;
    let a_imag: f32 = 6.5;
    let a: Complex<f32> = Complex::new(a_real, a_imag);
    let conj: Complex<f32> = Complex::new(a_real, -a_imag);
    assert_eq!(conj, a.conj());
}

#[test]
fn div0() {
    let a_real: f32 = -3.5;
    let a_imag: f32 = 6.5;
    let b_real: f32 = 2.0;
    let b_imag: f32 = -3.0;
    let a: Complex<f32> = Complex::new(a_real, a_imag);
    let b: Complex<f32> = Complex::new(b_real, b_imag);
    let prod: Complex<f32> = Complex::new((a_real * b_real + a_imag * b_imag)/(b_real* b_real +
        b_imag * b_imag), (a_imag * b_real - a_real * b_imag)/(b_real * b_real + b_imag * b_imag));

    assert_eq!(prod, a / b);
}

#[test]
fn div1()
{
    let a_real: f32 = -3.5;
    let a_imag: f32 = 6.5;
    let b_real: f32 = 0.0;
    let b_imag: f32 = 3.0;
    let a: Complex<f32> = Complex::new(a_real, a_imag);
    let b: Complex<f32> = Complex::new(b_real, b_imag);
    let prod: Complex<f32> = Complex::new((a_real * b_real + a_imag * b_imag)/(b_real* b_real +
        b_imag * b_imag), (a_imag * b_real - a_real * b_imag)/(b_real * b_real + b_imag * b_imag));

    assert_eq!(prod, a / b);
}

#[test]
fn sub()
{
    let a : Complex<f32> = Complex::new(-3.5, 6.5);
    let b : Complex<f32> = Complex::new(2.0, -3.0);
    let diff: Complex<f32> = Complex::new(-5.5, 9.5);
    assert_eq!(diff, a - b);
}

#[test]
fn neg0()
{
    let a_real : f32 = 1.0;
    let a_imag : f32 = 2.0;
    let uut : Complex<f32> = Complex::new(a_real, a_imag);
    assert_eq!(Complex::new(-a_real, -a_imag), -uut);
}


#[test]
fn from_f64()
{
    let real = 3.14f64;
    let imag = 0.0f64;
    let a: Complex<f64> = Complex::from_f64(real);

    assert_eq!(real, a.re);
    assert_eq!(imag, a.im)
}