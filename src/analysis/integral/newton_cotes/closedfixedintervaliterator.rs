use crate::algebra::abstr::Real;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct ClosedFixedIntervalIterator<T>
    where T: Real
{
    upper: T,
    next: T,
    h: T
}

impl<T> ClosedFixedIntervalIterator<T>
    where T: Real
{
    pub fn new(lower: T, upper: T, num_intervals: u32) -> ClosedFixedIntervalIterator<T>
    {
        ClosedFixedIntervalIterator{
            upper,
            next: lower,
            h: (upper - lower) / T::from_u32(num_intervals)
        }
    }
}

impl<T> Iterator for ClosedFixedIntervalIterator<T>
    where T: Real
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {

        let ret_val: T = self.next;

        self.next = ret_val + self.h;

        if ret_val > self.upper {
            None
        } else {
            Some(ret_val)
        }
    }
}