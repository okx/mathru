#[cfg(feature = "lapack")]
pub mod lapack;
#[cfg(feature = "native")]
pub mod native;

pub mod qrdec;
pub use self::qrdec::QRDec;
