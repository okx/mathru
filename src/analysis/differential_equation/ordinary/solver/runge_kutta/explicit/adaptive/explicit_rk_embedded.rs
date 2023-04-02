use crate::algebra::abstr::Real;
use crate::algebra::linear::Vector;
use crate::analysis::differential_equation::ordinary::ExplicitODE;
use std::clone::Clone;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct ExplicitRKEmbedded<T>
{
    a: Vec<T>,
    b: Vec<T>,
    b_order: u8,
    b_s: Vec<T>,
    b_s_order: u8,
    c: Vec<T>
}


impl<T> ExplicitRKEmbedded<T>
    where T: Real
{
    pub fn new( a: Vec<T>, b: Vec<T>, b_order: u8, b_s: Vec<T>, b_s_order: u8, c: Vec<T>) -> ExplicitRKEmbedded<T>
    {
        ExplicitRKEmbedded
        {
            a,
            b,
            b_order,
            b_s,
            b_s_order,
            c
        }
    }

    pub fn do_step<F>(&self, prob: &F, t_n: &T, x_n: &Vector<T>, h: &T) -> (Vector<T>, Vector<T>)
        where F: ExplicitODE<T>,
    {
        let mut k: Vec<Vector<T>> = Vec::with_capacity(self.b.len());
        let (rows, _columns): (usize, usize) = x_n.dim();

        k.push(prob.func(t_n, x_n));

        for j in 1..self.b.len()
        {
            let i_b = (j - 1) * j / 2;
            let i_e = i_b + j;

            let sum= self.a[i_b..i_e].iter().zip(k.iter()).map(|(a_jl, k_l)| k_l * a_jl).fold(Vector::zero(rows),  | a, b| a + b);

            let k_i = prob.func(&(*t_n + self.c[j - 1] * *h), &(x_n + &(&sum * h))) ;

            k.push(k_i);
        }

        let sum: Vector<T> = self.b.iter().zip(k.iter()).map(|(b, k_j)| k_j * b).fold(Vector::zero(rows), |a, b| a + b);
        let x_n_1 = x_n + &(&sum * h);

        let sum_s: Vector<T> = self.b_s.iter().zip(k.iter()).map(|(b, k_j)| k_j * b).fold(Vector::zero(rows), |a, b| a + b);
        let x_s_n_1 = x_n + &(&sum_s * h);

        (x_n_1, x_s_n_1)
    }

    pub fn order(&self) -> (u8, u8)
    {
        (self.b_order, self.b_s_order)
    }
}


pub trait ExplicitRKEmbeddedMethod<T>
{
    fn tableau<'a>(&'a self) -> &'a ExplicitRKEmbedded<T>;
}