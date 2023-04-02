//! Elementary functions
//!
//! Fore more information:
//! <https://en.wikipedia.org/wiki/Elementary_function>

pub mod power;
pub mod exponential;
pub mod trigonometry;
pub mod hyperbolic;

pub use self::{
    exponential::Exponential, hyperbolic::Hyperbolic, power::Power, trigonometry::Trigonometry,
};
