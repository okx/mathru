#[macro_use]
pub mod matrix;
//mod matrixcolumniterator;
//mod matrixcolumniteratormut;
mod matrixintoiterator;
mod matrixiterator;
mod matrixiteratormut;
//mod matrixrowiterator;
//mod matrixrowiteratormut;
mod matrixcolumnintoiterator;
mod matrixrowintoiterator;
mod eigen;
pub use self::eigen::EigenDec;

mod hessenberg;
pub use self::hessenberg::HessenbergDec;

mod lu;
pub use self::lu::LUDec;

mod qr;
pub use self::qr::QRDec;

mod add;
mod add_assign;
mod sub;
mod sub_assign;
mod div;
mod inverse;
mod mul;
mod mul_assign;

mod det;
mod singular;
mod cholesky;
pub use self::cholesky::CholeskyDec;
mod index;

mod solve;
mod substitute;
mod transpose;

#[cfg(feature = "convert-mint")]
mod mint;

pub use self::{
    inverse::Inverse,
    matrix::Matrix,
    //matrixcolumniterator::MatrixColumnIterator,
    //matrixcolumniteratormut::MatrixColumnIteratorMut,
    matrixcolumnintoiterator::MatrixColumnIntoIterator,
    matrixintoiterator::MatrixIntoIterator,
    matrixiterator::MatrixIterator,
    matrixiteratormut::MatrixIteratorMut,
    //matrixrowiterator::MatrixRowIterator,
    //matrixrowiteratormut::MatrixRowIteratorMut,
    matrixrowintoiterator::MatrixRowIntoIterator,
    solve::Solve,
    substitute::Substitute, transpose::Transpose
};
