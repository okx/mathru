//! Butcher tableau
use crate::analysis::differential_equation::ordinary::ExplicitODE;
use crate::algebra::linear::Vector;
use crate::algebra::abstr::Real;
use std::fmt::Debug;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use super::{explicit_method::ExplicitEmbeddedMethod};


/// ```math
/// y_{n+1} = y_n + h \sum_{i = 1}^{s}b_i k_i
/// ```
/// ```math
/// k_i = f(t_n + c_ih, y_n + h \sum_{j=1}^{s}a_{ij}k_j), \quad i = 1, \dots , s
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct ButcherFixedStepSize<T>
{
    a: Vec<T>,
    b: Vec<T>,
    c: Vec<T>,
}

impl<T> ButcherFixedStepSize<T>
    where T: Real
{
    pub fn new(a: Vec<T>, b: Vec<T>, c: Vec<T>) -> ButcherFixedStepSize<T>
    {
        ButcherFixedStepSize
        {
            a,
            b,
            c,
        }
    }
}

impl<T> ButcherFixedStepSize<T>
    where T: Real
{
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
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct ExplicitButcherAdaptiveStepSize<T>
{
    a: Vec<T>,
    b: Vec<T>,
    b_order: u8,
    b_s: Vec<T>,
    b_s_order: u8,
    c: Vec<T>,
}

pub trait ButcherTableauEmbedded<T>
{
    fn tableau<'a>(&'a self) -> &'a ExplicitButcherAdaptiveStepSize<T>;
}

///
/// ```math
/// y_{n+1}^* = y_n 1 h \sum_{i=1}^{s} b_{i}^{s}$k_{i}
/// ```
impl<T: Debug> ExplicitButcherAdaptiveStepSize<T>
{
    pub fn new(a: Vec<T>, b: Vec<T>, b_order: u8, b_s: Vec<T>, b_s_order: u8, c: Vec<T>) -> ExplicitButcherAdaptiveStepSize<T>
    {
        ExplicitButcherAdaptiveStepSize
        {
            a,
            b,
            b_order,
            b_s,
            b_s_order,
            c
        }
    }
}

impl<T> ExplicitEmbeddedMethod<T> for ExplicitButcherAdaptiveStepSize<T>
    where T: Real + Debug
{
    fn do_step<F>(&self, prob: &F, t_n: &T, x_n: &Vector<T>, h: &T) -> (Vector<T>, Vector<T>)
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

    fn order(&self) -> (u8, u8)
    {
        (self.b_order, self.b_s_order)
    }
}