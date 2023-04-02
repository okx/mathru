//use crate::algebra::linear::{Vector, Matrix};
//use crate::optimization::{OptimResult, Jacobian};
//use crate::algebra::abstr::Real;
//
//pub struct NonlinearConjugateGradient<T>
//{
//    iters: u64,
//    epsilon: T
//}
//
///// The nonlinear conjugate gradient is a generalized method of the conjugate
///// gradient method for nonlinear optimization problems. In this module, the
///// method of Polak and Ribiere is implemented.
/////
///// input: $ f \colon \mathbb{R}^{n} \to \mathbb{R} $ with initial
///// approximation $ x_{0} \in \mathbb{R}^{n} $
/////
///// output: $ x_{k} $
/////
///// 1. Initialization: $ \sigma \in (0, 1), 0 < \underline{\gamma} < 1 <
///// \overline{\gamma} $, set $ d_{0} := - \nabla f (x_{0}), k := 0 $
///// 2. $ \alpha_{k} := \frac{\lvert \nabla f(x_k)^{T}d_{k} \rvert}{\lvert
///// \lvert d_{k} \rvert \rvert_{2}^{2}} $ 3. $ x_{k+1} := x_k + \alpha_{k}
///// d_k $ <br>     $ \beta_{k} := \frac{\nabla f(x_{k+1})^T(\nabla
///// f(x_{k+1}) - \nabla f(x_k))}{\lvert \lvert \nabla f(x_k) \rvert
///// \rvert_{2}^{2}} $ <br>
/////     $ d_{k+1} := - \nabla f(x_{k+1}) + \beta_k d_k $
///// 4. If one of the following condition is not hold, halve $ \alpha_k $ and
///// go to 3 <br>     $f(x_{k+1}) \leq f(x_k) - \sigma \alpha_k^2 \lvert
///// \lvert d_k \rvert \rvert_2^2 $ <br>     $-\overline{\gamma}\lvert \lvert
///// \nabla f(x_{k+1}) \rvert \rvert_{2}^{2} \leq \nabla f(x_{k+1})^Td_{k+1}
///// \leq -\underline{\gamma}\lvert \lvert \nabla f(x_{k+1}) \rvert
///// \rvert_{2}^{2} $ 5. if $ \nabla f(x_{k+1}) \neq 0 $ increase $ k:= k +
///// 1 $ and go to 2 # Example
/////
/////
//impl<T> NonlinearConjugateGradient<T>
//{
//    pub fn new(iters: u64, epsilon: T) -> NonlinearConjugateGradient<T>
//    {
//        NonlinearConjugateGradient
//        {
//            iters: iters,
//            epsilon: epsilon,
//        }
//    }
//}
//
//impl<T> NonlinearConjugateGradient<T>
//    where T: Real
//{
//    /// Minimize function func
//    ///
//    /// f:Rn→R
//    /// # Arguments
//    ///
//    /// * 'func0': Function to be minimized
//    /// * 'x_0': Initial guess for the minimum
//    ///
//    /// # Return
//    ///
//    /// local minimum
//    pub fn minimize<F: Jacobian<T>>(&self, func: &F, x_0: &Vector<T>) ->
// OptimResult<Vector<T>>    {
//        let mut x_n: Vector<T> = x_0.clone();
//        let jacobian_n: Matrix<T> = func.jacobian(&x_n);
//
//        let mut d_n: Vector<T> = -jacobian_n;
//
//        for _i in 0..self.iters
//        {
//            let jacobian: Matrix<T> = func.jacobian(&x_n);
//
//            let temp: Vector<T> = (d_n.clone().transpose() *
// jacobian.clone());            let d_n_norm_squared =
//
//            let alpha_n: T = jacobian_n.dotp(&d_n) / d_n_norm_squared;
//
//            x_n = x_n + d_n.clone() * alpha_n;
//
//            let r_n_1: Vector<T> = r_n.clone()  - jacobian * d_n.clone() *
// alpha_n;
//
//            let beta_n: T = r_n_1.dotp(&r_n_1) / r_n.dotp(&r_n);
//
//            d_n = r_n_1.clone() + d_n * beta_n;
//
//            if r_n_1.dotp(&r_n_1).pow(&T::from_f64(0.5).unwrap()) <
// self.epsilon            {
//                break;
//            }
//
//            r_n = r_n_1
//        }
//
//        return OptimResult::new(x_n);
//    }
//}
