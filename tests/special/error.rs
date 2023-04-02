use mathru::special::error;

#[test]
fn erf_0()
{
    let x: f64 = 0.0;

    let error: f64 = error::erf(x);

    assert_relative_eq!(0.0, error, epsilon=3.0 * f64::EPSILON);
}

#[test]
fn erf_1()
{
    let x: f64 = -0.5;

    let error: f64 = error::erf(x);

    assert_relative_eq!(-0.5204998778130465, error, epsilon=3.0 * f64::EPSILON);
}

#[test]
fn erf_2()
{
    let x: f64 = 0.5;

    let error: f64 = error::erf(x);

    assert_relative_eq!(0.5204998778130465, error, epsilon=3.0 * f64::EPSILON);
}

#[test]
fn erf_3()
{
    let x: f64 = 3.5;

    let error: f64 = error::erf(x);

    assert_relative_eq!(0.9999992569016276, error, epsilon=3.0 * f64::EPSILON);
}

#[test]
fn erfc_0()
{
    let x: f64 = 0.0;

    let error: f64 = error::erfc(x);

    assert_relative_eq!(1.0, error, epsilon=3.0 * f64::EPSILON);
}

#[test]
fn erfc_1()
{
    let x: f64 = 0.5;

    let error: f64 = error::erfc(x);

    assert_relative_eq!(1.0 - error::erf(x), error, epsilon=3.0 * f64::EPSILON);
}


#[test]
fn erfc_2()
{
    let x: f64 = -0.5;

    let error: f64 = error::erfc(x);

    assert_relative_eq!(1.0 - error::erf(x), error, epsilon=3.0 * f64::EPSILON);
}

#[test]
fn erfc_3()
{
    let x: f64 = 3.5;

    let error: f64 = error::erfc(x);

    assert_relative_eq!(0.00000074309837239106, error, epsilon=3.0 * f64::EPSILON);
}

#[test]
fn erfc_4()
{
    let x: f64 = -3.5;

    let error: f64 = error::erfc(x);

    assert_relative_eq!(1.0 - error::erf(x), error, epsilon=3.0 * f64::EPSILON);
}

#[test]
fn erfinv_0()
{
    let x: f64 = 0.0;

    let error: f64 = error::erfinv(x);

    assert_relative_eq!(0.0, error, epsilon=0.0 * f64::EPSILON);
}

#[test]
fn erfinv_1()
{
    let x: f64 = 0.1;

    let error: f64 = error::erfinv(x);

    assert_relative_eq!(0.08885599049425768701574, error, epsilon=0.0 * f64::EPSILON);
}

#[test]
fn erfinv_2()
{
    let x: f64 = -0.1;

    let error: f64 = error::erfinv(x);

    assert_relative_eq!(-0.08885599049425768701574, error, epsilon=0.0 * f64::EPSILON);
}

#[test]
fn erfinv_3()
{
    let x: f64 = 0.75;

    let error: f64 = error::erfinv(x);

    assert_relative_eq!(0.8134198475976185416903, error, epsilon=0.0 * f64::EPSILON);
}

#[test]
fn erfinv_4()
{
    let x: f64 = 0.9375;

    let error: f64 = error::erfinv(x);

    assert_relative_eq!(1.317150334986130748884, error, epsilon=0.0 * f64::EPSILON);
}

#[test]
fn erfinv_5()
{
    let x: f64 = 0.98;

    let error: f64 = error::erfinv(x);

    assert_relative_eq!(1.644976357133187050177, error, epsilon=15.0 * f64::EPSILON);
}

#[test]
fn erfcinv_0()
{
    let x: f64 = 0.0;

    let error: f64 = error::erfcinv(x);

    assert_relative_eq!(f64::INFINITY, error, epsilon=0.0 * f64::EPSILON);
}

#[test]
fn erfcinv_1()
{
    let x: f64 = 0.1;

    let error: f64 = error::erfcinv(x);

    assert_relative_eq!(1.163087153676674086726, error, epsilon=0.0 * f64::EPSILON);
}


#[test]
fn erfcinv_2()
{
    let x: f64 = 0.75;

    let error: f64 = error::erfcinv(x);

    assert_relative_eq!(0.225312055012178104725, error, epsilon=5.0 * f64::EPSILON);
}

#[test]
fn erfcinv_3()
{
    let x: f64 = 0.9375;

    let error: f64 = error::erfcinv(x);

    assert_relative_eq!(0.05544594877278202029899, error, epsilon=2.0 * f64::EPSILON);
}

#[test]
fn erfcinv_4()
{
    let x: f64 = 0.98;

    let error: f64 = error::erfcinv(x);

    assert_relative_eq!(0.0177263950266780184822, error, epsilon=15.0 * f64::EPSILON);
}

#[test]
fn erfcinv_5()
{
    let x: f64 = 1.98;

    let error: f64 = error::erfcinv(x);

    assert_relative_eq!(-1.64497635713319, error, epsilon=15.0 * f64::EPSILON);
}

#[test]
fn erfcinv_6()
{
    let x: f64 = 2.0;

    let error: f64 = error::erfcinv(x);

    assert_relative_eq!(f64::NEG_INFINITY, error, epsilon=15.0 * f64::EPSILON);
}

#[test]
fn erfcinv_7()
{
    let x: f64 = 0.0;

    let error: f64 = error::erfcinv(x);

    assert_relative_eq!(f64::INFINITY, error, epsilon=15.0 * f64::EPSILON);
}
