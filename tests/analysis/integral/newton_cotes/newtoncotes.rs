use mathru::analysis::integral::newton_cotes::{NewtonCotes};


#[test]
fn newton_cotes_1()
{
    let nc = NewtonCotes::new(1);
    let f = | x | {x};

    let integral = nc.integrate(f, 2.0, 4.0, 4);

    assert_relative_eq!(integral, 6.0);
}

#[test]
fn newton_cotes_2()
{
    let nc = NewtonCotes::new(2);
    let f = | x | {x};

    let integral = nc.integrate(f, 2.0, 4.0, 1);

    assert_relative_eq!(integral, 6.0);
}

#[test]
fn newton_cotes_3()
{
    let nc = NewtonCotes::new(3);
    let f = | x | {x};

    let integral = nc.integrate(f, 2.0, 4.0, 1);

    assert_relative_eq!(integral, 6.0);
}

#[test]
fn newton_cotes_4()
{
    let nc = NewtonCotes::new(4);
    let f = | x | {x};

    let integral = nc.integrate(f, 2.0, 4.0, 1);

    assert_relative_eq!(integral, 6.0);
}

#[test]
fn newton_cotes_5()
{
    let nc = NewtonCotes::new(5);
    let f = | x | {x};

    let integral = nc.integrate(f, 2.0, 4.0, 1);

    assert_relative_eq!(integral, 6.0);
}

