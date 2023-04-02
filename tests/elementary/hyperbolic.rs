use mathru::algebra::abstr::{Complex, One};
use mathru::elementary::{Exponential, Hyperbolic, Power, Trigonometry};

#[test]
fn sinh_complex_f64()
{
    let a: f64 = 1.0;
    let b: f64 = 2.0;
    let z: Complex<f64> = Complex::new(a, b);

    let uut: Complex<f64> = z.sinh();

    let refer: Complex<f64> = Complex::new(0.0_f64, -1.0_f64) * Complex::new(-2.0_f64, 1.0_f64).sin();
    assert_eq!(refer, uut);
}

#[test]
fn sinh_complex_f32()
{
    let a: f32 = 1.0;
    let b: f32 = 2.0;
    let z: Complex<f32> = Complex::new(a, b);

    let uut: Complex<f32> = z.sinh();

    let refer: Complex<f32> = Complex::new(0.0_f32, -1.0_f32) * Complex::new(-2.0_f32, 1.0_f32).sin();
    assert_eq!(refer, uut);
}

#[test]
fn sinh_f32()
{
  assert_eq!(3.6268604, 2.0f32.sinh());
}

#[test]
fn sinh_f64()
{
    assert_eq!(3.626860407847019, 2.0f64.sinh());
}


#[test]
fn cosh()
{
    let a: f64 = 1.0;
    let b: f64 = 2.0;
    let z: Complex<f64> = Complex::new(a, b);

    let uut: Complex<f64> = z.cosh();

    let refer: Complex<f64> = Complex::new(-2.0_f64, 1.0_f64).cos();

    assert_eq!(refer, uut);
}

#[test]
fn tanh()
{
    let a: f64 = 1.0;
    let b: f64 = 2.0;
    let z: Complex<f64> = Complex::new(a, b);
    let refer: Complex<f64> = z.sinh() / z.cosh();

    let uut: Complex<f64> = z.tanh();
    assert_eq!(refer, uut);
}


#[test]
fn coth()
{
    let a: Complex<f64> = Complex::new(1.0_f64, 2.0_f64);
    let refer: Complex<f64> = a.cosh() / a.sinh();

    assert_eq!(refer, a.coth());
}

#[test]
fn sech()
{
    let a: Complex<f64> = Complex::new(1.0_f64, 2.0_f64);
    let refer: Complex<f64> = Complex::new(-2.0_f64, 1.0_f64).sec();

    assert_eq!(refer, a.sech());
}

#[test]
fn csch()
{
    let a: Complex<f64> = Complex::new(1.0_f64, 2.0_f64);
    let refer: Complex<f64> = Complex::new(0.0_f64, -1.0_f64) * Complex::new(-2.0_f64, 1.0_f64).csc();

    assert_eq!(refer, a.csch());
}

#[test]
fn artanh()
{
    let a: Complex<f64> = Complex::new(0.5_f64, -0.4_f64);
    let f: Complex<f64> = Complex::new(0.5_f64, 0.0_f64);
    let refer: Complex<f64> = ((Complex::one() + a) / (Complex::one() - a)).ln() * f;

    assert_eq!(refer, a.artanh());
}

#[test]
fn arcoth()
{
    let a: Complex<f64> = Complex::new(1.0_f64, 2.0_f64);
    let f: Complex<f64> = Complex::new(0.5_f64, 0.0_f64);
    let refer: Complex<f64> = ((a + Complex::one()) / (a - Complex::one())).ln() * f;

    assert_eq!(refer, a.arcoth());
}

#[test]
fn arsinh()
{
    let a: Complex<f64> = Complex::new(1.0_f64, 2.0_f64);
    let pow: Complex<f64> = Complex::new(0.5_f64, 0.0_f64);

    let refer: Complex<f64> = (a + (a * a + Complex::one()).pow(pow)).ln();

    assert_eq!(refer, a.arsinh());
}

#[test]
fn arcosh()
{
    let a: Complex<f64> = Complex::new(1.0_f64, 2.0_f64);
    let pow: Complex<f64> = Complex::new(0.5_f64, 0.0_f64);

    let refer: Complex<f64> = (a + (a * a - Complex::one()).pow(pow)).ln();

    assert_eq!(refer, a.arcosh());
}

#[test]
fn arcsech()
{
    let a: Complex<f64> = Complex::new(1.0_f64, 2.0_f64);
    let refer: Complex<f64> = (Complex::one() / a).arcosh();

    assert_eq!(refer, a.arsech());
}

#[test]
fn arccsch()
{
    let a: Complex<f64> = Complex::new(1.0_f64, 2.0_f64);
    let refer: Complex<f64> = (Complex::one() / a).arsinh();

    assert_eq!(refer, a.arcsch());
}