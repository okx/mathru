//! Statistical hypothesis tests
//!
//! Fore more information:
//! <https://en.wikipedia.org/wiki/Statistical_hypothesis_testing>

mod chisquare;
mod g;
mod t;

#[allow(clippy::module_inception)]
mod test;

pub use self::{chisquare::ChiSquare, g::G, t::T, test::Test};
