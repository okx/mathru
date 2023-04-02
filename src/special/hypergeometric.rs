//! Hypergeometric functions

use crate::{algebra::abstr::Real, special::gamma};
use crate::elementary::Power;
use crate::special::gamma::Gamma;

pub trait Hypergeometric
{
    fn f21(self, b: Self, c: Self, z: Self) -> Self;
}

macro_rules! impl_hypergeometric
{
    ($t: ty, $norm_name: ident, $tolerance: expr, $infinity: expr) =>
    {

             fn $norm_name(a: $t, b: $t, c: $t, z: $t) -> $t
        {
            let mut c_i: $t = 1.0;
            let mut s_i: $t = c_i;
            let mut s_i_p: $t = s_i; //s_{i-1}
            let mut j: $t = 0.0;

            while c_i.abs() / s_i_p.abs() > $tolerance
            {
                let k: $t = (a + j) * (b + j) / (c + j);
                let l: $t = z / (j + 1.0);
                c_i = c_i * k * l;
                s_i_p = s_i;
                s_i += c_i;

                j += 1.0;
            }
            s_i
        }


        impl Hypergeometric for $t
        {
            fn f21(self, b: Self, c: Self, z: Self) -> Self
            {
                if self <= 0.0 || b <= 0.0 || c <= 0.0 || z > 1.0
                {
                    panic!();
                }

                if z == 1.0
                {
                    if c - self - b < 0.0
                    {
                        return c.gamma() * (self + b - c).gamma() / (self.gamma() * b.gamma());
                    }
                    else
                    {
                        if c - self - b == 0.0
                        {
                            return gamma::gamma(c) / (gamma::gamma(self) * gamma::gamma(b));
                        }
                        else
                        {
                            return gamma::gamma(c) * gamma::gamma(c - self - b)
                                   / (gamma::gamma(c - self) * gamma::gamma(c - b));
                        }
                    }
                }

                let f: Self;
                if -$infinity < z && z < -1.0
                {
                    let l1: Self = (1.0 - z).pow(-self) * gamma::gamma(c) * gamma::gamma(b - self)
                                / (gamma::gamma(b) * gamma::gamma(c - self));

                    let l2: Self = (1.0 - z).pow(-b) * gamma::gamma(c) * gamma::gamma(self - b)
                                / (gamma::gamma(self) * gamma::gamma(c - b));

                    let f1: Self = $norm_name(self, c - b, self - b + 1.0, 1.0 / (1.0 - z));
                    let f2: Self = $norm_name(b, c - self, b - self + 1.0, 1.0 / (1.0 - z));

                    f = l1 * f1 + l2 * f2;
                }
                else
                {
                    if (-1.0..0.0).contains(&z)
                    {
                        f = $norm_name(self, c - b, c, z / (z - 1.0)) * (1.0 - z).pow(-self);
                    }
                    else
                    {
                        if (0.0..=0.5).contains(&z)
                        {
                            f = $norm_name(self, b, c, z);
                        }
                        else
                        {
                            if (0.5..=1.0).contains(&z)
                            {
                                let l1: Self = gamma::gamma(c) * gamma::gamma(c - self - b)
                                            / (gamma::gamma(c - self) * gamma::gamma(c - b));
                                let l2: Self = (1.0 - z).pow(c - self - b)
                                            * gamma::gamma(c)
                                            * gamma::gamma(self + b - c)
                                            / (gamma::gamma(self) * gamma::gamma(b));
                                let f1: Self = $norm_name(self, b, self + b - c + 1.0, 1.0 - z);
                                let f2: Self = $norm_name(c - self, c - b, c - self - b + 1.0, 1.0 - z);

                                f = l1 * f1 + l2 * f2;
                            }
                            else
                            {
                                if 1.0 < z && z <= 2.0
                                {
                                    //complex numbers are not supported
                                    //						let l1: f64 = gamma::gamma(c) * gamma::gamma(c - a - b) / (gamma::gamma(c
                                    // - a) * gamma::gamma 						(c - b)) * z.powf(-a); println!("{}", l1); let l2:
                                    //   f64 = (1.0 - z).powf(c - a - b) * z.powf(a - c) * gamma::gamma(c)
                                    // * gamma::gamma(a + 						b - c) / (gamma::gamma(a) *
                                    // gamma::gamma(b)); 						println!("{}", l2);
                                    //						let f1: f64 = f21_norm(a, a - c + 1.0, a + b - c + 1.0, 1.0 - 1.0 / z);
                                    //						println!("{}", f1);
                                    //						let f2: f64 = f21_norm(c - a, 1.0 - a, c - a - b + 1.0, 1.0 - 1.0 / z);
                                    //						println!("{}", f2);
                                    //
                                    //						f = l1 * f1 + l2 * f2;
                                    unimplemented!();
                                //f = 0.0;
                                }
                                // 2.0 < z && z < std::f64::INFINITY
                                else
                                {
                                    //complex numbers are not supported
                                    //						let l1: f64 = gamma::gamma(c) * gamma::gamma(b - a) / (gamma::gamma(b) *
                                    // gamma::gamma 						(c - b)) * (-a * z.ln()).exp();
                                    //						println!("l1: {}", l1);
                                    //						let l2: f64 = gamma::gamma(c) * gamma::gamma(a - b) / (gamma::gamma(a) *
                                    // gamma::gamma(c - b))
                                    // 						 * (-b * z.ln()).exp();
                                    //						 println!("l2: {}", l2);
                                    //						let f1: f64 = f21_norm(a, 1.0 - c + a, 1.0 - b + a, 1.0 / z);
                                    //						let f2: f64 = f21_norm(b, 1.0 - c + b, 1.0 - a + b, 1.0 / z);
                                    //
                                    //						f = l1 * f1 + l2 * f2;
                                    unimplemented!();
                                    //f = 0.0;
                                }
                            }
                        }
                    }
                }

                return f;
            }

        }
    };
}

impl_hypergeometric!(f64, f21_f64_norm, 0.0000000000000002, std::f64::INFINITY);
impl_hypergeometric!(f32, f21_f32_norm, 0.0000000000000002, std::f32::INFINITY);


/// Hypergeometric function
///
/// # Arguments
///
/// * `a` > 0.0
/// * `b` > 0.0
/// * `c` > 0.0
/// * `z` < 1.0
///
/// # Panics
///
/// if the argument conditions are not fulfilled
///
/// Fore more information:
/// <https://en.wikipedia.org/wiki/Hypergeometric_function>
///
/// The implementation follows the suggested algorithm in the follow:
/// Numerical methods for the computation of the confluent and Gauss
/// hypergeometric functions Pearson et al. 20
/// Taylor series, Method a)
/// <https://arxiv.org/abs/1407.7786>
///
/// <https://cran.r-project.org/web/packages/hypergeo/vignettes/hypergeometric.pdf>
/// <http://people.maths.ox.ac.uk/porterm/research/pearson_final.pdf>
pub fn f21<T>(a: T, b: T, c: T, z: T) -> T
    where T: Real + Hypergeometric
{
    a.f21(b, c, z)
}
