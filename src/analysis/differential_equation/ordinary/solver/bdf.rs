//! Solves an ODE using backward differentiation formula
use crate::{
    algebra::{abstr::Real, linear::vector::vector::Vector},
    analysis::differential_equation::ordinary::ImplicitODE,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::clone::Clone;

/// Backward differentiation formula
///
/// # Example
///
/// For this example, we want to solve the following stiff ordinary
/// differential equation:
/// ```math
/// 0 = -4(y(t) -2) - y(t)^{'} = f(t, y, y^{'})
/// ```
/// The initial condition is $y(0) = 1.0$ and we solve it in the interval
/// $\lbrack 0, 2\rbrack$. The following equation is the closed solution for
/// this ODE:
/// ```math
/// y(t) = 2 - e^{-t}
/// ```
///
/// ```
/// # #[macro_use]
/// # extern crate mathru;
/// # fn main()
/// # {
/// use mathru::{
///     algebra::linear::{Matrix, Vector},
///     analysis::differential_equation::ordinary::{ImplicitODE, solver::BDF},
/// };
///
/// pub struct ODEProblem
/// {
///     time_span: (f64, f64),
///     init_cond: Vector<f64>,
/// }
///
/// impl Default for ODEProblem
/// {
///     fn default() -> ODEProblem
///     {
///         return ODEProblem { time_span: (0.0, 2.0), init_cond: vector![1.0] };
///     }
/// }
///
/// impl ImplicitODE<f64> for ODEProblem
/// {
///     fn func(&self, _t: &f64, x: &Vector<f64>) -> Vector<f64>
///     {
///         let result = (x * &-4.0) + 8.0;
///         return result;
///     }
///
///     fn time_span(&self) -> (f64, f64)
///     {
///         return self.time_span;
///     }
///
///     fn init_cond(&self) -> Vector<f64>
///     {
///         return self.init_cond.clone();
///     }
///
///     fn jacobian(&self, _t: &f64, _input: &Vector<f64>) -> Matrix<f64>
///     {
///         let jacobian = matrix![-4.0];
///         return jacobian;
///     }
/// }
///
/// let step_size: f64 = 0.0001;
/// let solver: BDF<f64> = BDF::new(2, step_size);
///
/// let problem: ODEProblem = ODEProblem::default();
///
/// // Solve the ODE
/// let (t, x): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();
/// # }
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug)]
pub struct BDF<T>
{
    k: u8,
    step_size: T,
}

impl<T> BDF<T> where T: Real
{
    ///
    pub fn new(k: u8, step_size: T) -> BDF<T>
    {
        if k == 0 || k > 6
        {
            panic!();
        }
        if step_size <= T::zero()
        {
            panic!();
        }

        BDF { k, step_size }
    }
}

impl<T> BDF<T> where T: Real
{
    /// Solves `func` using BDF method.
    ///
    /// # Arguments
    ///
    /// * 'func' is an explict oridnary diffential equation
    /// * 'init' is the initial value at the time 't_start'
    /// * 't_span' Time span
    ///
    /// # Return
    ///
    /// The solver returns a vector and a matrix, containing the times used in
    /// each step of the algorithm and the respectful values for that time.
    ///
    /// # Panic
    ///
    /// if t_span.0 > t_span.1
    pub fn solve<F>(&self, prob: &F) -> Result<(Vec<T>, Vec<Vector<T>>), ()>
        where F: ImplicitODE<T>
    {
        let t_span: (T, T) = prob.time_span();
        let t_start: T = t_span.0;
        let t_stop: T = t_span.1;

        if t_start > t_stop
        {
            panic!();
        }

        let mut x_n: Vector<T> = prob.init_cond();
        let mut t_n: T = t_start;

        let limit = ((t_stop - t_start) / self.step_size).ceil() + T::one();
        let steps: usize = limit.to_u64() as usize;
        let mut t_vec: Vec<T> = Vec::with_capacity(steps);
        let mut res_vec: Vec<Vector<T>> = Vec::with_capacity(steps);

        t_vec.push(t_n);
        res_vec.push(x_n.clone());

        //calculate initial steps
        if self.k >= 2
        {
            let h: T = self.step_size.min(t_stop - t_n);
            let x_n = BDF::step_s1(prob, &t_vec, &res_vec, h);
            t_n += h;

            t_vec.push(t_n);
            res_vec.push(x_n);
        }

        if self.k >= 3
        {
            let h: T = self.step_size.min(t_stop - t_n);
            let x_n = BDF::step_s2(prob, &t_vec, &res_vec, h);
            t_n += h;

            t_vec.push(t_n);
            res_vec.push(x_n);
        }

        if self.k >= 4
        {
            let h: T = self.step_size.min(t_stop - t_n);
            let x_n = BDF::step_s3(prob, &t_vec, &res_vec, h);
            t_n += h;

            t_vec.push(t_n);
            res_vec.push(x_n);
        }

        if self.k >= 5
        {
            let h: T = self.step_size.min(t_stop - t_n);
            let x_n = BDF::step_s4(prob, &t_vec, &res_vec, h);
            t_n += h;

            t_vec.push(t_n);
            res_vec.push(x_n);
        }

        if self.k >= 6
        {
            let h: T = self.step_size.min(t_stop - t_n);
            let x_n = BDF::step_s5(prob, &t_vec, &res_vec, h);
            t_n += h;

            t_vec.push(t_n);
            res_vec.push(x_n);
        }

        let step = match self.k
        {
            1 => BDF::step_s1,
            2 => BDF::step_s2,
            3 => BDF::step_s3,
            4 => BDF::step_s4,
            5 => BDF::step_s5,
            6 => BDF::step_s6,
            _ => panic!(),
        };

        while (t_n - t_stop).abs() > T::from_f64(0.0000000001)
        {
            //Step size
            let h: T = self.step_size.min(t_stop - t_n);

            x_n = step(prob, &t_vec, &res_vec, h);
            t_n += h;

            t_vec.push(t_n);
            res_vec.push(x_n.clone());
        }

        Ok((t_vec, res_vec))
    }
}

impl<T> BDF<T> where T: Real
{
    fn step_s1<F>(prob: &F, t: &Vec<T>, x: &Vec<Vector<T>>, h: T) -> Vector<T>
        where F: ImplicitODE<T>
    {
        let n: usize = x.len() - 1;
        let x_n: &Vector<T> = &x[n];
        let t_n: T = t[n];
        x_n + &(&prob.func(&t_n, x_n) * &h)
    }

    fn step_s2<F>(prob: &F, t: &Vec<T>, x: &Vec<Vector<T>>, h: T) -> Vector<T>
        where F: ImplicitODE<T>
    {
        let n: usize = x.len() - 1;
        let x_n: &Vector<T> = &x[n];
        let t_n: T = t[n];
        let x_n1: &Vector<T> = &x[n - 1];
        let t_n1: T = t[n - 1];
        x_n + &((prob.func(&t_n, x_n) * T::from_f64(3.0 / 2.0)
                    + prob.func(&t_n1, x_n1) * T::from_f64(-0.5))
                   * h)
    }

    fn step_s3<F>(prob: &F, t: &Vec<T>, x: &Vec<Vector<T>>, h: T) -> Vector<T>
        where F: ImplicitODE<T>
    {
        let n: usize = x.len() - 1;
        let x_n: &Vector<T> = &x[n];
        let t_n: T = t[n];
        let x_n1: &Vector<T> = &x[n - 1];
        let t_n1: T = t[n - 1];
        let x_n2: &Vector<T> = &x[n - 2];
        let t_n2: T = t[n - 2];
        x_n + &((prob.func(&t_n, x_n) * T::from_f64(23.0 / 12.0)
                    + prob.func(&t_n1, x_n1) * T::from_f64(-16.0 / 12.0)
                    + prob.func(&t_n2, x_n2) * T::from_f64(5.0 / 12.0))
                   * h)
    }

    fn step_s4<F>(prob: &F, t: &Vec<T>, x: &Vec<Vector<T>>, h: T) -> Vector<T>
        where F: ImplicitODE<T>
    {
        let n: usize = x.len() - 1;
        let x_n: &Vector<T> = &x[n];
        let t_n: T = t[n];
        let x_n1: &Vector<T> = &x[n - 1];
        let t_n1: T = t[n - 1];
        let x_n2: &Vector<T> = &x[n - 2];
        let t_n2: T = t[n - 2];
        let x_n3: &Vector<T> = &x[n - 3];
        let t_n3: T = t[n - 3];
        x_n + &((prob.func(&t_n, x_n) * T::from_f64(55.0 / 24.0)
                    + prob.func(&t_n1, x_n1) * T::from_f64(-59.0 / 24.0)
                    + prob.func(&t_n2, x_n2) * T::from_f64(37.0 / 24.0)
                    + prob.func(&t_n3, x_n3) * T::from_f64(-9.0 / 24.0))
                   * h)
    }

    fn step_s5<F>(prob: &F, t: &Vec<T>, x: &Vec<Vector<T>>, h: T) -> Vector<T>
        where F: ImplicitODE<T>
    {
        let n: usize = x.len() - 1;
        let x_n: &Vector<T> = &x[n];
        let t_n: T = t[n];
        let x_n1: &Vector<T> = &x[n - 1];
        let t_n1: T = t[n - 1];
        let x_n2: &Vector<T> = &x[n - 2];
        let t_n2: T = t[n - 2];
        let x_n3: &Vector<T> = &x[n - 3];
        let t_n3: T = t[n - 3];
        let x_n4: &Vector<T> = &x[n - 4];
        let t_n4: T = t[n - 4];
        x_n + &((prob.func(&t_n, x_n) * T::from_f64(1901.0 / 720.0)
                    + prob.func(&t_n1, x_n1) * T::from_f64(-2774.0 / 720.0)
                    + prob.func(&t_n2, x_n2) * T::from_f64(2616.0 / 720.0)
                    + prob.func(&t_n3, x_n3) * T::from_f64(-1274.0 / 720.0)
                    + prob.func(&t_n4, x_n4) * T::from_f64(251.0 / 720.0))
                   * h)
    }

    fn step_s6<F>(prob: &F, t: &Vec<T>, x: &Vec<Vector<T>>, h: T) -> Vector<T>
        where F: ImplicitODE<T>
    {
        let n: usize = x.len() - 1;
        let x_n: &Vector<T> = &x[n];
        let t_n: T = t[n];
        let x_n1: &Vector<T> = &x[n - 1];
        let t_n1: T = t[n - 1];
        let x_n2: &Vector<T> = &x[n - 2];
        let t_n2: T = t[n - 2];
        let x_n3: &Vector<T> = &x[n - 3];
        let t_n3: T = t[n - 3];
        let x_n4: &Vector<T> = &x[n - 4];
        let t_n4: T = t[n - 4];
        x_n + &((prob.func(&t_n, x_n) * T::from_f64(1901.0 / 720.0)
                    + prob.func(&t_n1, x_n1) * T::from_f64(-2774.0 / 720.0)
                    + prob.func(&t_n2, x_n2) * T::from_f64(2616.0 / 720.0)
                    + prob.func(&t_n3, x_n3) * T::from_f64(-1274.0 / 720.0)
                    + prob.func(&t_n4, x_n4) * T::from_f64(251.0 / 720.0))
                   * h)
    }
}
