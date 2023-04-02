use mathru::statistics::combins;

#[test]
fn binom0()
{
    let n: u32 = 5;
    let k: u32 = 3;
    let c: u32 = 10;

    assert_eq!(c, combins::binom(n, k))
}

#[test]
fn binom1()
{
    let n: u32 = 6;
    let k: u32 = 3;
    let c: u32 = 20;

    assert_eq!(c, combins::binom(n, k))
}

#[test]
fn binom2()
{
    let n: u32 = 5;
    let k: u32 = 2;
    let c: u32 = 10;

    assert_eq!(c, combins::binom(n, k))
}

#[test]
fn factorial()
{
    let n: u32 = 5;
    assert_eq!(120, combins::factorial(n));
}
