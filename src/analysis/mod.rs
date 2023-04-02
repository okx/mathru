//! Analysis
//!
//! Fore more information: <br>
//! <https://en.wikipedia.org/wiki/Analysis>

//pub mod interpolation;
#[macro_use]
mod function;
mod hessian;
mod jacobian;

mod newton_raphson;

pub mod differential_equation;
pub mod integral;

pub use function::Function;
pub use hessian::Hessian;
pub use jacobian::Jacobian;
pub use newton_raphson::NewtonRaphson;
