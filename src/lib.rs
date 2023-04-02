//! # mathru
//!
//! A crate that provides  mathematics functions implemented entirely in Rust.
//!
//! ## Usage
//!
//! The library usage is described well in the API documentation - including
//! example code.
//!
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! mathru = "0.13"
//! ```
//!
//! Then it is ready to be used:
//!
//!``` rust
//! # #[macro_use]
//! # extern crate mathru;
//! # fn main()
//! # {
//! use mathru::algebra::linear::{Vector, Matrix};
//! use mathru::algebra::linear::matrix::{Substitute};
//!
//! // Compute the LU decomposition of a 2x2 matrix
//! let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 2.0, -3.0, -7.0]);
//! let b: Vector<f64> = vector![1.0; 3.0];
//!
//! let (l, u, p): (Matrix<f64>, Matrix<f64>, Matrix<f64>) = a.dec_lu().unwrap().lup();
//!
//! let b_hat = &p * &b;
//!
//! let y = u.substitute_backward(b_hat).unwrap();
//!
//! let x = p * l.substitute_forward(y).unwrap();
//!
//! println!("{}", x);
//! # }
//! ```
//!
//! ```
//! use mathru::vector;
//! use mathru::algebra::linear::{Vector, Matrix};
//! use mathru::statistics::distrib::{Distribution, Normal};
//! use mathru::optimization::{Optim, LevenbergMarquardt};
//!
//!
//! //y = a + b * exp(c * t) = f(t)
//! pub struct Example
//! {
//!    x: Vector<f64>,
//!    y: Vector<f64>
//! }
//!
//! impl Example
//! {
//!     pub fn new(x: Vector<f64>, y: Vector<f64>) -> Example
//!     {
//!         Example
//!         {
//!             x,
//!             y
//!         }
//!     }
//!
//!     pub fn function(x: f64, beta: &Vector<f64>) -> f64
//!     {
//!         let beta_0: f64 = beta[0];
//!         let beta_1: f64 = beta[1];
//!         let beta_2: f64 = beta[2];
//!         let f_x: f64 = beta_0 + beta_1 * (beta_2 * x).exp();
//!
//!         return f_x;
//!     }
//! }
//!
//! impl Optim<f64> for Example
//! {
//!     // y(x_i) - f(x_i)
//!     fn eval(&self, beta: &Vector<f64>) -> Vector<f64>
//!     {
//!         let f_x = self.x.clone().apply(&|x: &f64| Example::function(*x, beta));
//!         let r: Vector<f64> = &self.y - &f_x;
//!         return vector![r.dotp(&r)];
//!     }
//!
//!     fn jacobian(&self, beta: &Vector<f64>) -> Matrix<f64>
//!     {
//!         let (x_m, _x_n) = self.x.dim();
//!         let (beta_m, _beta_n) = beta.dim();
//!
//!         let mut jacobian_f: Matrix<f64> = Matrix::zero(x_m, beta_m);
//!
//!         let f_x = self.x.clone().apply(&|x: &f64| Example::function(*x, beta));
//!         let residual: Vector<f64> = &self.y - &f_x;
//!
//!         for i in 0..x_m
//!         {
//!             let beta_1: f64 = beta[1];
//!             let beta_2: f64 = beta[2];
//!
//!             let x_i: f64 = self.x[i];
//!
//!             jacobian_f[[i, 0]] = 1.0;
//!             jacobian_f[[i, 1]] = (beta_2 * x_i).exp();
//!             jacobian_f[[i, 2]] = beta_1 * x_i * (beta_2 * x_i).exp();
//!            }
//!
//!            let jacobian: Matrix<f64> = (residual.transpose() * jacobian_f * -2.0).into();
//!         return jacobian;
//!     }
//! }
//!
//!
//! fn main()
//! {
//!     let num_samples: usize = 100;
//!
//!     let noise: Normal<f64> = Normal::new(0.0, 0.05);
//!
//!     let mut t_vec: Vec<f64> = Vec::with_capacity(num_samples);
//!
//!     // Start time
//!     let t_0 = 0.0f64;
//!     // End time
//!     let t_1 = 5.0f64;
//!
//!     let mut y_vec: Vec<f64> = Vec::with_capacity(num_samples);
//!
//!     // True function parameters
//!     let beta: Vector<f64> = vector![0.5; 5.0; -1.0];
//!
//!     for i in 0..num_samples
//!     {
//!         let t_i: f64 = (t_1 - t_0) / (num_samples as f64) * (i as f64);
//!
//!         //Add some noise
//!         y_vec.push(Example::function(t_i, &beta) + noise.random());
//!
//!         t_vec.push(t_i);
//!     }
//!
//!     let t: Vector<f64> = Vector::new_column(t_vec.clone());
//!     let y: Vector<f64> = Vector::new_column(y_vec.clone());
//!
//!     let example_function = Example::new(t, y);
//!
//!     let optim: LevenbergMarquardt<f64> = LevenbergMarquardt::new(100, 0.3, 0.95);
//!
//!     let beta_0: Vector<f64> = vector![-1.5; 1.0; -2.0];
//!     let beta_opt: Vector<f64> = optim.minimize(&example_function, &beta_0).unwrap().arg();
//!
//!     println!("{}", beta_opt);
//! }
//! ```

#[cfg(feature = "lapack")]
extern crate blas;
#[cfg(feature = "lapack")]
extern crate blas_src;
#[cfg(feature = "lapack")]
extern crate lapack;

#[macro_use]
pub mod algebra;
pub mod analysis;
pub mod elementary;
pub mod optimization;
pub mod special;
pub mod statistics;
