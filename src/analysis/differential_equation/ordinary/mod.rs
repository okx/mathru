//! Ordinary differential equation
//!
//! Fore more information:<br>
//! <https://en.wikipedia.org/wiki/Ordinary_differential_equation>
//!
//! Because ODE higher order can always be reduced to a system of first order
//! ODEs,  the implemented algorithms only support to solve first order ODEs.
//!
//! ```math
//! \frac{dy}{dt}=f(t, y)
//! ```
//! 
//! 
//! Solves an ODE using the Runge-Kutta-Dormand-Prince algorithm.
//!
//!<https://en.wikipedia.org/wiki/Dormand-Prince_method>
//!
//! # Example
//!
//! For this example, we want to solve the following ordinary differential
//! equation:
//! ```math
//! \frac{dy}{dt} = ay = f(t, y)
//! ```
//! The initial condition is $y(0) = 0.5$ and we solve it in the interval
//! $\lbrack 0, 2\rbrack$ The following equation is the closed solution for
//! this ODE:
//! ```math
//! y(t) = C a e^{at}
//! ```
//! $C$ is a parameter and depends on the initial condition $y(t_{0})$
//! ```math
//! C = \frac{y(t_{0})}{ae^{at_{0}}}
//! ```
//!
//! In this example, we set $a=2$
//! ```
//! # #[macro_use]
//! # extern crate mathru;
//! # fn main()
//! # {
//! use mathru::{
//!     algebra::linear::Vector,
//!     analysis::differential_equation::ordinary::{solver::runge_kutta::{DormandPrince54, ProportionalControl}, ExplicitODE},
//! };
//!
//! pub struct ExplicitODE1
//! {
//!     time_span: (f64, f64),
//!     init_cond: Vector<f64>,
//! }
//!
//! impl Default for ExplicitODE1
//! {
//!     fn default() -> ExplicitODE1
//!     {
//!         ExplicitODE1 { time_span: (0.0, 2.0),
//!                        init_cond: vector![0.5] }
//!     }
//! }
//!
//! impl ExplicitODE<f64> for ExplicitODE1
//! {
//!     fn func(&self, _t: &f64, x: &Vector<f64>) -> Vector<f64>
//!     {
//!         return x * &2.0f64;
//!     }
//!
//!     fn time_span(&self) -> (f64, f64)
//!     {
//!         return self.time_span;
//!     }
//!
//!     fn init_cond(&self) -> Vector<f64>
//!     {
//!         return self.init_cond.clone();
//!     }
//! }
//!
//! let h_0: f64 = 0.1;
//! let fac: f64 = 0.9;
//! let fac_min: f64 = 0.01;
//! let fac_max: f64 = 2.0;
//! let n_max: u32 = 500;
//! let abs_tol: f64 = 10e-8;
//! let rel_tol: f64 = 10e-6;
//!
//! let solver: ProportionalControl<f64> = ProportionalControl::new(n_max, h_0, fac, fac_min, fac_max, abs_tol, rel_tol);
//! let problem: ExplicitODE1 = ExplicitODE1::default();
//!
//! // Solve the ODE
//! let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem, &DormandPrince54::default()).unwrap();
//!
//! # }
//! ```

pub mod solver;
pub mod problem;

mod explicit_ode;
mod implicit_ode;

pub use implicit_ode::ImplicitODE;
pub use explicit_ode::ExplicitODE;

