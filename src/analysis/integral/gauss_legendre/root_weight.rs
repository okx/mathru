use crate::algebra::abstr::Real;
use std::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct RootWeight<T>
{
    roots: Vec<T>,
    weights: Vec<T>,
    n: i8,
    i: i8,
    k: i8,
}

impl<T> RootWeight<T>
    where T: Real
{
    
    pub fn new(n: u8) -> RootWeight<T>
    {
        let roots = match n
        {
            1 => vec![T::from_f64(0.0)],
            2 => vec![T::from_f64(0.577350269189626)],
            3 => vec![T::from_f64(0.0), T::from_f64(0.774596669241483)],
            4 => vec![T::from_f64(0.339981043584856), T::from_f64(0.861136311594053)],
            5 => vec![T::from_f64(0.0), T::from_f64(0.538469310105683), T::from_f64(0.906179845938664)],
            6 => vec![T::from_f64(0.238619186083197), T::from_f64(0.661209386466265), T::from_f64(0.932469514203152)],
            7 => vec![T::from_f64(0.0), T::from_f64(0.405845151377397), T::from_f64(0.741531185599394), T::from_f64(0.949107912342759)],
            8 => vec![T::from_f64(0.183434642495650), T::from_f64(0.525532409916329), T::from_f64(0.796666477413627), T::from_f64(0.960289856497536)],
            9 => vec![T::from_f64(0.0), T::from_f64(0.324253423403809), T::from_f64(0.613371432700590), T::from_f64(0.836031107326636), T::from_f64(0.968160239507626)],
            _ => panic!("")
        };

        let weights = match n
        {
            1 => vec![T::from_f64(2.0)],
            2 => vec![T::from_f64(1.0)],
            3 => vec![T::from_f64(0.888888888888889), T::from_f64(0.555555555555556)],
            4 => vec![T::from_f64(0.652145154862546), T::from_f64(0.347854845137454)],
            5 => vec![T::from_f64(0.568888888888889), T::from_f64(0.478628670499366), T::from_f64(0.236926885056189)],
            6 => vec![T::from_f64(0.467913934572691), T::from_f64(0.360761573048139), T::from_f64(0.171324492379170)],
            7 => vec![T::from_f64(0.417959183673469), T::from_f64(0.381830050505119), T::from_f64(0.279705391489277), T::from_f64(0.129484966168870)],
            8 => vec![T::from_f64(0.362683783378362), T::from_f64(0.313706645877887), T::from_f64(0.222381034453374), T::from_f64(0.101228536290376)],
            9 => vec![T::from_f64(0.330239355001260), T::from_f64(0.312347077040003), T::from_f64(0.260610696402935), T::from_f64(0.180648160694857), T::from_f64(0.081274388361574)],
            _ => panic!("")
        };

        let k: i8 = if n % 2 == 0
        {
            n as i8 / 2
        }
        else
        {
            n as i8 / 2 + 1
        };

        RootWeight
        {
            roots,
            weights,
            n: n as i8,
            i: 1,
            k
        }
    }

    fn idx_n_even(it: &Self) -> usize
    {
        if it.i <= it.k
        {
            (it.i % it.k) as usize
        }
        else
        {
            (it.i - (it.k + 1)) as usize
        }
    }

    fn idx_n_odd(it: &Self) -> usize
    {
        (it.i - it.k).abs() as usize
    }
}

impl<T> Iterator for RootWeight<T>
    where T: Real
{
    type Item = (T, T);


    fn next(&mut self) -> Option<Self::Item>
    {
        let f_idx = if self.n % 2 == 0
        {
            RootWeight::idx_n_even
        }
        else
        {
            RootWeight::idx_n_odd
        };

        if self.i <= self.n
        {

            let idx = f_idx(self);
            let root = self.roots[idx];
            let weight = self.weights[idx];
            let pair = if self.i <= self.k
            {
                Some((-root, weight))
            }
            else
            {
                Some((root, weight))
            };
            self.i += 1;
            pair
        }
        else
        {
            None
        }
    }
}