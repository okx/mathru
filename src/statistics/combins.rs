/// Factorial
///
/// <https://en.wikipedia.org/wiki/Factorial>
pub fn factorial(f: u32) -> u64
{
    let mut n: u64 = f as u64;
    let mut result: u64 = 1;
    while n > 0
    {
        result *= n;
        n -= 1;
    }
    result
}

/// Binomial coefficient
///
/// <https://en.wikipedia.org/wiki/Binomial_coefficient>
pub fn binom(n: u32, k: u32) -> u32
{
    (factorial(n) / (factorial(k) * factorial(n - k))) as u32
}

pub fn perm(_n: u32, _r: u32) -> u32
{
    //	let n_fact : u32 = n.factorial();
    //	let r_fact : u32 = r_factorial();
    //	let diff_fact : u32 = (n - r).factorial();
    unimplemented!()
}

pub fn comb<'a>(_n: &'a u32, _r: &'a u32) -> u32
{
    unimplemented!()
}
