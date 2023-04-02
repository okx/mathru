use mathru::analysis::integral::gauss_legendre::RootWeight;

#[test]
fn root_weight_1()
{
    let mut roots = RootWeight::<f64>::new(1);

    assert_eq!(roots.next(), Some((0.0, 2.0)));
    assert_eq!(roots.next(), None);
}

#[test]
fn root_weight_2()
{
    let mut roots = RootWeight::<f64>::new(2);

    assert_eq!(roots.next(), Some((-0.577350269189626, 1.0)));
    assert_eq!(roots.next(), Some((0.577350269189626, 1.0)));
    assert_eq!(roots.next(), None);
}

#[test]
fn root_weight_3()
{
    let mut roots = RootWeight::<f64>::new(3);

    assert_eq!(roots.next(), Some((-0.774596669241483, 0.555555555555556)));
    assert_eq!(roots.next(), Some((0.0, 0.888888888888889)));
    assert_eq!(roots.next(), Some((0.774596669241483, 0.555555555555556)));
    assert_eq!(roots.next(), None);
}

#[test]
fn root_weight_4()
{
    let mut roots = RootWeight::<f64>::new(4);

    assert_eq!(roots.next(), Some((-0.861136311594053, 0.347854845137454)));
    assert_eq!(roots.next(), Some((-0.339981043584856, 0.652145154862546)));
    assert_eq!(roots.next(), Some((0.339981043584856, 0.652145154862546)));
    assert_eq!(roots.next(), Some((0.861136311594053, 0.347854845137454)));
    assert_eq!(roots.next(), None);
}

