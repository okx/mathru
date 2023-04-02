//#![feature(test)]
#![allow(unused_macros)]
extern crate mathru;

#[macro_use]
extern crate criterion;

mod algebra;
mod analysis;


criterion_main!(
    algebra::linear::vector::add::vector_add,
    algebra::linear::vector::add_assign::vector_add_assign,
    algebra::linear::vector::sub::vector_sub,
    algebra::linear::vector::sub_assign::vector_sub_assign,
    algebra::linear::vector::mul::vector_mul,

    algebra::linear::matrix::add::add,
    algebra::linear::matrix::sub::sub,
    algebra::linear::matrix::mul::mul,

    // analysis::vector_bench::euler,

    analysis::ode::dormandprince::dormandprince,
    algebra::linear::matrix::matrix,
    analysis::ode::ode,
);
