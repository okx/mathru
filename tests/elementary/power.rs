use mathru::algebra::abstr::Complex;
use mathru::elementary::Power;

#[test]
fn pow0_complex_f64()
{
    let a: Complex<f64> = Complex::new(1.0_f64, 2.0_f64);
    let b: Complex<f64> = Complex::new(-2.0_f64, -1.0_f64);
    let refer: Complex<f64> = Complex::new(-0.6006033457684014, -0.07399065302898929);

    assert_eq!(refer, a.pow(b));
}

#[test]
fn pow0_complex_f32()
{
    let a: Complex<f32> = Complex::new(1.0_f32, 2.0_f32);
    let b: Complex<f32> = Complex::new(-2.0_f32, -1.0_f32);
    let refer: Complex<f32> = Complex::new(-0.6006034, -0.07399058);

    assert_eq!(refer, a.pow(b));
}

#[test]
fn pow0_f32()
{
    let a: f32 = 2.0_f32;
    let b: f32 = -2.0_f32;

    assert_eq!(0.25, a.pow(b));
}

#[test]
fn pow0_f64()
{
    let a: f64 = 2.0_f64;
    let b: f64 = -2.0_f64;

    assert_eq!(0.25, a.pow(b));
}

#[test]
fn pow1()
{
    let a: Complex<f64> = Complex::new(1.0_f64, 2.0_f64);
    let b: Complex<f64> = Complex::new(-2.0_f64, 1.0_f64);
    let refer: Complex<f64> = Complex::new(0.010610396101816041, -0.06524284357147048);

    assert_eq!(refer, a.pow(b));
}

#[test]
fn pow2()
{
    let a: Complex<f64> = Complex::new(1.0_f64, 2.0_f64);
    let b: Complex<f64> = Complex::new(-0.5_f64, -0.8_f64);
    let refer: Complex<f64> = Complex::new(0.591571342038212, -1.5097505998220102);

    assert_eq!(refer, a.pow(b));
}