use crate::{
    algebra::{
        abstr::Real,
        linear::{Matrix, Vector},
    },
    optimization::{Optim, OptimResult},
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::clone::Clone;

/// Conjugate Gradient method
///
/// The conjugate gradient method is a solver for systems of linear equations
/// with a symmetric and positive-definite matrix. Ax = b where A is a symmetric
/// and positive-definite matrix
///
/// input: $A \in  \mathbb{R}^{n \times n}$ and $ b \in \mathbb{R}^{n} $ and
/// initial approximation $x_{0} \in \mathbb{R}^{n} $
///
/// output: $ x_k $
///
/// 1. $ d_{0} = r_{0} := b - Ax_{0} $ and set $ k := 0 $
/// 2. $ \alpha_{k} := \frac{\lvert \lvert r_{k} \rvert
/// \rvert_{2}^{2}}{d_{k}^{T}Ad_{k}} $ <br>     $ x_{k+1} := x_{k} +
/// \alpha_{j}d_{k} $ <br>     $ r_{k+1} := r_{k} - \alpha_{k}Ad_{k} $ <br>
///     $ \beta_{k} := \frac{\lvert \lvert r_{k+1} \rvert
/// \rvert_{2}^{2}}{\lvert \lvert r_{k} \rvert \rvert_{2}^{2}} $ <br>
///     $ d_{k+1} := r_{k+1} + \beta_{k}d_{k} $ <br>
/// 3. if $ \lvert \lvert r_{k+ 1} \rvert \rvert_{2} > \epsilon $ increase
/// $k:= k + 1$ and goto 2.
///
/// # Example
///
/// ```
/// use mathru::*;
/// use mathru::algebra::linear::{Vector, Matrix};
/// use mathru::optimization::{Optim, ConjugateGradient};
///
/// struct LinearEquation
/// { ///
///     a: Matrix<f64>,
///     b: Vector<f64>,
/// }
///
/// //Ax = b
/// impl LinearEquation
/// {
///     pub fn new() -> LinearEquation
///     {
///         LinearEquation
///         {
///             a: matrix![1.0, 3.0; 3.0, 5.0],
///             b: vector![-7.0; 7.0]
///         }
///     }
/// }
///
/// impl Optim<f64> for LinearEquation
/// {
///     // A
///		fn jacobian(&self, _input: &Vector<f64>) -> Matrix<f64>
///		{
///			return self.a.clone();
///		}///
///
///     // f = b-Ax
///		fn eval(&self, x: &Vector<f64>) -> Vector<f64>
///		{
///			return self.b.clone() - self.a.clone() * x.clone()
///		}
///
///     //Computes the Hessian at the given value x
///     fn hessian(&self, _x: &Vector<f64>) -> Matrix<f64>
///     {
///         unimplemented!();
///     }
/// }
///
/// //create optimizer instance
/// let optim: ConjugateGradient<f64> = ConjugateGradient::new(10, 0.01);
///
/// let leq: LinearEquation = LinearEquation::new();
///
/// // Initial approximation
/// let x_0: Vector<f64> = vector![1.0; 1.0];
///
/// // Minimize function
/// let x_min: Vector<f64> = optim.minimize(&leq, &x_0).arg();
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug)]
pub struct ConjugateGradient<T>
{
    iters: u64,
    epsilon: T,
}

impl<T> ConjugateGradient<T>
{
    /// Creates an instance of ConjugateGradient method
    ///
    /// # Arguments
    ///
    /// * 'iters': Number of iterations
    /// * 'epsilon':
    pub fn new(iters: u64, epsilon: T) -> ConjugateGradient<T>
    {
        ConjugateGradient { iters, epsilon }
    }
}

impl<T> ConjugateGradient<T> where T: Real
{
    /// Minimize function func
    ///
    /// # Arguments
    ///
    /// * 'func0': Function to be minimized
    /// * 'x_0': Initial guess for the minimum
    ///
    /// # Return
    ///
    /// local minimum
    pub fn minimize<F>(&self, func: &F, x_0: &Vector<T>) -> OptimResult<Vector<T>>
        where F: Optim<T>
    {
        let mut x_n: Vector<T> = x_0.clone();
        let mut d_n: Vector<T> = func.eval(&x_n);
        let mut r_n: Vector<T> = d_n.clone();

        for _i in 0..self.iters
        {
            let jacobian: Matrix<T> = func.jacobian(&x_n);

            let temp: Vector<T> = d_n.clone().transpose() * jacobian.clone();

            let alpha_n: T = r_n.dotp(&r_n) / temp.transpose().dotp(&d_n);

            x_n += d_n.clone() * alpha_n;

            let r_n_1: Vector<T> = r_n.clone() - jacobian * d_n.clone() * alpha_n;

            let beta_n: T = r_n_1.dotp(&r_n_1) / r_n.dotp(&r_n);

            d_n = r_n_1.clone() + d_n * beta_n;

            if r_n_1.dotp(&r_n_1).pow(T::from_f64(0.5)) < self.epsilon
            {
                break;
            }

            r_n = r_n_1
        }

        OptimResult::new(x_n)
    }
}
