use crate::{
    algebra::linear::Vector,
    optimization::{Optim, OptimResult},
};
extern crate rand;
use crate::algebra::abstr::Real;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::clone::Clone;

/// Gradient method
///
/// It is assumed that $f \colon D \in \mathbb{R}^n \to \mathbb{R}$
/// The idea is, that in every iteration a step is made in direction of
/// the anti gradient.
///
/// ```math
/// x_{k + 1} := x_{k} - \alpha_{k} \nabla f(x_{k})
/// ```
/// in order that $ f(x_{k + 1}) < f(x_{k}) $.
///
/// input: Function $ f: \mathbb{R}^n \to \mathbb{R} $, and initial
/// approximation $ x_{0} \in \mathbb{R}^{n} $
///
/// output: $ x_{k} $
///
/// 1. Initialization: choose $\sigma \in (0, 1) $
///
///     set  $k := 0 $
/// 2. calculate antigradient $d_{k} := -\nabla f(x_{k}) $
///
///     set $ \alpha_{k} := 1 $
/// 3. while $f(x_{k} + \alpha_{k} d_{k}) > f(x_k) + \sigma \alpha_{k} \lvert
/// \lvert d_{k} \rvert \rvert_{2}^{2} $     set  $ \alpha_{k} := \alpha_{k}
/// /2 $ 4. $ x_{k + 1} := x_{k} + \alpha_{k} d_{k} $
/// 5. $ k := k + 1 $ go to 2.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug)]
pub struct Gradient<T>
{
    /// Learning rate
    sigma: T,
    /// The number of iterations to run.
    iters: usize,
}

impl<T> Gradient<T> where T: Real
{
    /// Construct an instance of gradient algorithm.
    ///
    /// # Parameters
    ///
    /// sigma: learning rate > 0.0
    ///
    /// # Examples
    ///
    /// ```
    /// use mathru::optimization::Gradient;
    ///
    /// let gd = Gradient::new(0.3, 10000);
    /// ```
    pub fn new(sigma: T, iters: usize) -> Gradient<T>
    {
        assert!(sigma <= T::one() && sigma > T::zero());
        assert!(iters > 0);
        Gradient { sigma, iters }
    }
}

impl<T> Gradient<T> where T: Real
{
    pub fn minimize<F>(&self, func: &F, x_0: &Vector<T>) -> OptimResult<Vector<T>>
        where F: Optim<T>
    {
        let mut x_k: Vector<T> = x_0.clone();

        for _k in 0..self.iters
        {
            let mut alpha_k: T = T::one();
            let anti_grad: Vector<T> = -func.jacobian(&x_k).get_row(0).transpose();

            //Backtracking line search
            //Armijo–Goldstein condition
            loop
            {
                let anti_grad_alpha: Vector<T> = &anti_grad * &alpha_k;
                let r: Vector<T> = &x_k + &anti_grad_alpha;
                let f_r: T = func.eval(&r)[0];
                let b: T = self.sigma * anti_grad_alpha.dotp(&anti_grad);

                let f_x_k: T = func.eval(&x_k)[0];

                if f_r <= f_x_k - b
                {
                    break;
                }
                alpha_k /= T::from_f64(2.0);
            }
            //Make step
            x_k += anti_grad * alpha_k;
        }
        OptimResult::new(x_k)
    }
}
