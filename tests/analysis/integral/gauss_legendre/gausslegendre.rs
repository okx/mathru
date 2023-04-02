use mathru::analysis::integral::gauss_legendre::{GaussLegendre};


#[test]
fn gauss_legendre_1()
{
    let gl: GaussLegendre<f64> = GaussLegendre::new(1);
    let f = | x | {x};

    let integral: f64 = gl.integrate(f, 2.0, 4.0);

    assert_relative_eq!(integral, 6.0);
}

#[test]
fn gauss_legendre_2()
{
    let gl: GaussLegendre<f64> = GaussLegendre::new(2);
    let f = | x | {x};

    let integral: f64 = gl.integrate(f, 2.0, 4.0);

    assert_relative_eq!(integral, 6.0);
}

#[test]
fn gauss_legendre_3()
{
    let gl: GaussLegendre<f64> = GaussLegendre::new(3);
    let f = | x | {x};

    let integral: f64 = gl.integrate(f, 2.0, 4.0);

    assert_relative_eq!(integral, 6.0, epsilon=0.000000001);
}

#[test]
fn gauss_legendre_4()
{
    let gl: GaussLegendre<f64> = GaussLegendre::new(4);
    let f = | x | {x};

    let integral: f64 = gl.integrate(f, 2.0, 4.0);

    assert_relative_eq!(integral, 6.0, epsilon=0.000000001);
}

#[test]
fn gauss_legendre_5()
{
    let gl: GaussLegendre<f64> = GaussLegendre::<f64>::new(5);
    let f = | x | {x};

    let integral: f64 = gl.integrate(f, 2.0, 4.0);

    assert_relative_eq!(integral, 6.0, epsilon=0.000000001);
}

#[test]
fn gauss_legendre_6()
{
    let gl: GaussLegendre<f64> = GaussLegendre::new(6);
    let f = | x | {x};

    let integral: f64 = gl.integrate(f, 2.0, 4.0);

    assert_relative_eq!(integral, 6.0, epsilon=0.000000001);
}

#[test]
fn gauss_legendre_7()
{
    let gl: GaussLegendre<f64> = GaussLegendre::new(7);
    let f = | x | {x};

    let integral: f64 = gl.integrate(f, 2.0, 4.0);

    assert_relative_eq!(integral, 6.0, epsilon=0.000000001);
}

#[test]
fn gauss_legendre_8()
{
    let gl: GaussLegendre<f64> = GaussLegendre::new(8);
    let f: fn(f64) -> f64 = | x | {x};

    let integral: f64 = gl.integrate(f, 2.0, 4.0);

    assert_relative_eq!(integral, 6.0, epsilon=0.000000001);
}

