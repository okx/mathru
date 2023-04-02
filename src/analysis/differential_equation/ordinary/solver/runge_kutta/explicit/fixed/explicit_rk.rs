use crate::algebra::abstr::Real;
use crate::algebra::linear::Vector;
use crate::analysis::differential_equation::ordinary::ExplicitODE;
use std::clone::Clone;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct ExplicitRK<T>
{
    a: Vec<T>,
    b: Vec<T>,
    b_order: u8,
    c: Vec<T>,
}


impl<T> ExplicitRK<T>
    where T: Real
{
    pub fn new( a: Vec<T>, b: Vec<T>, b_order: u8, c: Vec<T>) -> ExplicitRK<T>
    {
        ExplicitRK
        {
            a,
            b,
            b_order,
            c
        }
    }

    pub fn do_step<F>(&self, prob: &F, t_n: &T, x_n: &Vector<T>, h: &T) -> Vector<T>
        where F: ExplicitODE<T>,
    {
        let mut k: Vec<Vector<T>> = Vec::with_capacity(self.b.len());

        k.push(prob.func(t_n, x_n));

        for j in 1..self.b.len()
        {
            let i_b = (j - 1) * j / 2;
            let i_e = i_b + j;

            let sum= self.a[i_b..i_e].iter().zip(k.iter()).map(|(a_jl, k_l)| k_l * a_jl * *h).fold(x_n.clone(),  | a, b| a + b);

            k.push(prob.func(&(*t_n + self.c[j - 1] * *h), &sum));
        }

        self.b.iter().zip(k.iter()).map(|(b, k_j)|  k_j * &(*b * *h)).fold(x_n.clone(), |a, b| a + b)
    }

    pub fn order(&self) -> u8
    {
        self.b_order
    }
}

pub trait ExplicitRKMethod<T>
{
    fn tableau<'a>(&'a self) -> &'a ExplicitRK<T>;
}
