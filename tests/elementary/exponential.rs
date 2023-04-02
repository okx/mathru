use mathru::algebra::abstr::Complex;
use mathru::elementary::{Exponential, Trigonometry};

#[test]
fn exp_complex_f64()
{
    let z: Complex<f64> = Complex::new(1.0, 2.0);
    let uut: Complex<f64> = z.exp();

    let refer: Complex<f64> = Complex::new(1.0.exp() * 2.0.cos(), 1.0.exp() * 2.0.sin());

    assert_eq!(refer, uut);
}

#[test]
fn exp_complex_f32()
{
    let z: Complex<f32> = Complex::new(1.0, 2.0);
    let uut: Complex<f32> = z.exp();

    let refer: Complex<f32> = Complex::new(1.0f32.exp() * 2.0.cos(), 1.0f32.exp() * 2.0.sin());

    assert_eq!(refer, uut);
}

#[test]
fn exp_f32()
{
    assert_eq!(f32::e(), 1.0f32.exp());
}

#[test]
fn exp_f64()
{
    assert_eq!(f64::e(), 1.0f64.exp());
}

#[test]
fn ln()
{
    let a: Complex<f64> = Complex::new(1.0_f64, 2.0_f64);
    let refer: Complex<f64> = Complex::new(5.0_f64.powf(0.5_f64).ln(), 2.0_f64.arctan2(1.0_f64));

    assert_eq!(refer, a.ln());
}
