use crate::{
    algebra::{
        abstr::Real,
        linear::{matrix::Solve, Matrix, Vector},
    },
    optimization::{Optim, OptimResult},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::clone::Clone;


/// Newton's method
///
/// ```math
/// f \colon \mathbb{R}^n \to \mathbb{R}
/// ```
/// is a twice differentiable function.
///
/// Newton's method solves the minimization problem
///
/// ```math
///  f(x) \to min
/// ```
///
/// input: $ f \colon \mathbb{R}^{n} \to \mathbb{R} $ with initial
/// approximation $ x_{0} \in \mathbb{R}^{n} $
///
/// output: $ x_{k} $
///
/// 1. Initialization: Choose $ \sigma \in (0, 1) $
///
///     $ \rho > 0, k := 0 $
/// 2. Solve de equation system $ \nabla^{2} f(x_{k})d_{k} = -\nabla f(x_{k})
/// $ 3. If the euqation is not solvable, or the condition
/// $ \nabla f(x_{k})^{T}d_{k} \leq -\rho \lvert \lvert \nabla f(x_k) \rvert
/// \rvert_{2}^{2} $  is not fullfilled Than $ d_{k} := \nabla f(x_{k}) $
/// 4. $ \alpha_{k} := 1 $
/// 5. while  $ f(x_{k} + \alpha_{k}d_{k}) > f(x_{k}) + \sigma \alpha_{k}
/// \nabla f(x_{k})^{T}d_{k} $ set $ \alpha_{k} $ 6. $ x_{k + 1} := x_{k} +
/// d_{k} $ 7. $ k := k + 1 $  go to 2.
///
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug)]
pub struct Newton<T>
{
    iters: u64,
    sigma: T,
    rho: T,
}

impl<T> Newton<T>
{
    /// Creates an instance of newtons method
    ///
    /// # Arguments
    ///
    /// * 'iters': Number of iterations
    pub fn new(iters: u64, sigma: T, rho: T) -> Newton<T>
    {
        Newton { iters, sigma, rho }
    }
}

impl<T> Newton<T> where T: Real
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

        for _i in 0..self.iters
        {
            let hessian_x_n: Matrix<T> = func.hessian(&x_n);
            let grad_x_n: Vector<T> = func.jacobian(&x_n).get_row(0).transpose();
            let res_solve: Result<Vector<T>, ()> = hessian_x_n.solve(&-grad_x_n.clone());
            let d_k: Vector<T>;

            match res_solve
            {
                Ok(d_k_temp) =>
                {
                    let grad_x_n_abs: T = grad_x_n.dotp(&grad_x_n);
                    let grad_d_k_temp: T = grad_x_n.dotp(&d_k_temp);
                    if grad_d_k_temp <= -self.rho * grad_x_n_abs
                    {
                        d_k = d_k_temp;
                    }
                    else
                    {
                        d_k = -grad_x_n.clone();
                    }
                }
                Err(_e) =>
                {
                    d_k = -grad_x_n.clone();
                }
            }
            let mut alpha: T = T::one();
            //Backtracking line search
            //Armijo–Goldstein condition
            let mut r: Vector<T> = &x_n + &(&d_k * &alpha);
            let mut f_r: T = func.eval(&r)[0];
            let f_x_n: T = func.eval(&x_n)[0];
            let temp: T = grad_x_n.dotp(&d_k);
            while f_r > f_x_n + temp * (self.sigma * alpha)
            {
                alpha /= T::from_f64(2.0);
                r = &x_n + &(&d_k * &alpha);
                f_r = func.eval(&r)[0];
            }

            //Make step
            x_n += d_k * alpha;
        }

        OptimResult::new(x_n)
    }
}
