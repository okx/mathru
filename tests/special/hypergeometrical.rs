use mathru::special;

#[test]
fn hypergeometrical_f21_0()
{
    let a: f64 = 0.5_f64;
    let b: f64 = 0.3_f64;
    let c: f64 = 0.7_f64;
    let z: f64 = -5.8_f64;

    let h: f64 = special::hypergeometric::f21(a, b, c, z);

    assert_relative_eq!(0.6463419208795624, h);
}

#[test]
fn hypergeometrical_f21_1()
{
    let a: f64 = 0.5_f64;
    let b: f64 = 0.3_f64;
    let c: f64 = 0.7_f64;
    let z: f64 = -0.5_f64;

    let h: f64 = special::hypergeometric::f21(a, b, c, z);

    assert_relative_eq!(0.9157005646955769, h);
}

#[test]
fn hypergeometrical_f21_2()
{
    let a: f64 = 0.5_f64;
    let b: f64 = 0.3_f64;
    let c: f64 = 0.7_f64;
    let z: f64 = 0.5_f64;

    let h: f64 = special::hypergeometric::f21(a, b, c, z);

    assert_relative_eq!(1.1561182056693702, h);
}

#[test]
fn hypergeometrical_f21_3()
{
    let a: f64 = 0.5_f64;
    let b: f64 = 0.3_f64;
    let c: f64 = 0.7_f64;
    let z: f64 = 0.8_f64;

    let h: f64 = special::hypergeometric::f21(a, b, c, z);

    assert_relative_eq!(1.3858902990044464, h);
}

#[test]
fn hypergeometrical_f21_4()
{
    let a: f64 = 0.5_f64;
    let b: f64 = 0.3_f64;
    let c: f64 = 0.7_f64;
    let z: f64 = 0.9_f64;

    let h: f64 = special::hypergeometric::f21(a, b, c, z);

    assert_relative_eq!(1.577586297244991, h);
}

//    #[test]
//    fn hypergeometrical_f21_5()
//    {
//        let a: f64 = 0.5_f64;
//        let b: f64 = 0.3_f64;
//        let c: f64 = 0.7_f64;
//        let z: f64 = 3.9_f64;
//
//        let h: f64 = special::hypergeometrical::f21(a, b, c, z);
//
//        assert_relative_eq!(1.3858900032709065, h);
//    }

//    #[test]
//    fn hypergeometrical_f21_6()
//    {
//        let a: f64 = 0.5_f64;
//        let b: f64 = 1.5_f64;
//        let c: f64 = 1.5_f64;
//        let z: f64 = -2.0_f64;
//
//        let h: f64 = special::hypergeometrical::f21(a, b, c, z);
//
//        assert_relative_eq!(1.577586290185282, h);
//    }

#[test]
fn hypergeometrical_f21_7()
{
    let a: f64 = 0.5_f64;
    let b: f64 = 0.3_f64;
    let c: f64 = 0.7_f64;
    let z: f64 = -2.0_f64;

    let h: f64 = special::hypergeometric::f21(a, b, c, z);

    assert_relative_eq!(0.7834881981920094, h);
}

#[test]
fn hypergeometrical_f21_8()
{
    let a: f64 = 0.1_f64;
    let b: f64 = 0.2_f64;
    let c: f64 = 0.3_f64;
    let z: f64 = 0.5_f64;

    let h: f64 = special::hypergeometric::f21(a, b, c, z);

    assert_relative_eq!(1.0464328112173522, h);
}
