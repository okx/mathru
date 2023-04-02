# Change Log
All notable changes starting with the version 0.6.9 are documented here.

## 0.13.0
- Fix bug as reported in [Issue #8](https://gitlab.com/matthiaseiholzer/mathru/-/issues/8) and [Issue #12](https://gitlab.com/matthiaseiholzer/mathru/-/issues/12)
- Code refactoring

## [0.12.0]
- Implement convert-mint feature that can be enabled to convert from and to types of the mint crate.

## [0.11.3]
- Fix failing docs.rs build

## [0.11.2]
- Legendre polynomials
- Chebyshev polynomials first & second kind

## [0.11.1]

## [0.11.0]
- Code refactoring
- Performance improvements
- Implement Newton-Cotes and Gauss-Legendre as integration methods

## [0.10.1]
- Fixed Bug in LU decomposition [Issue #7](https://gitlab.com/matthiaseiholzer/mathru/-/issues/7)

## [0.10.0]
- Code refactoring
- Bug fix in QR decomposition algorithm

## [0.9.1]
- Update README.md and lib.rs to newest version

## [0.9.0]
- Implement additional ODE solvers
- Update dependencies
- Implement explicit ODE solvers with Butcher tableaus

## [0.8.4]
- Replace the out-of-place transpose algorithm with an in-place algorithm, therewith it matches to the documentation

## [0.8.3]
- Make serde dependency optional

## [0.8.2]
- Update dependencies
- Implement inverse of lower/upper regularized incomplete gamma function
- Improve accuracy of of the quantile function of the Chi-square distribution

## [0.8.1]
- README corrections

## [0.8.0]
- Implement polynomial

## [0.7.4]
- Improve documentation
- Update dependencies

## [0.7.3]
- Column/row iterators implemented
- Serde support

## [0.7.2]
- assert_eq! for floats replaced with assert_relative_eq! and assert_diff_abs_eq!
- CI pipeline for blas/lapack backends fixed
- Dev dependencies updated

## [0.7.1]
- Invalid URLs in README.md fixed

## [0.7.0]
- Using SemVer for release versioning
- log-normal distribution
- Different changes on beta, gamma and error functions

## [0.6.10]
- Native Rust code, Openblas, Netlib, Intel-Mkl and Accelerate are now usable as linear algebra libraries

## [0.6.9]
- Eigen decomposition is implemented
- Implicit Euler
- Backward differentiation formula
