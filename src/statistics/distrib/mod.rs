//! Distributions
//!
//! Each univariate distribution implements the trait Continuous or Discrete for
//! discrete distributions.
//!
//! Fore more information:
//! <https://en.wikipedia.org/wiki/Probability_distribution>
//!
mod bernoulli;
mod beta;
mod binomial;
mod chisquare;
mod distrib;
mod exponential;
mod gamma;
mod normal;
mod poisson;
mod t;
mod log_normal;
mod raisedcosine;
mod uniform;

pub use self::{
    bernoulli::Bernoulli,
    beta::Beta,
    binomial::Binomial,
    chisquare::ChiSquare,
    distrib::{Continuous, Discrete, Distribution},
    exponential::Exponential,
    gamma::Gamma,
    normal::Normal,
    poisson::Poisson,
    t::T,
    log_normal::LogNormal,
    raisedcosine::RaisedCosine,
    uniform::Uniform,
};
