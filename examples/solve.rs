use mathru::{
    algebra::linear::{
        matrix::{LUDec, Solve},
        Matrix, Vector,
    },
    matrix, vector,
};

/// Solves a system of linear equations
fn main()
{
    let a: Matrix<f64> = matrix![6.0, 2.0, -1.0; -3.0, 5.0, 3.0; -2.0, 1.0, 3.0];
    let b: Vector<f64> = vector![48.0; 49.0; 24.0];

    // Decompose a into a lower and upper matrix
    let lu_dec: LUDec<f64> = a.dec_lu().unwrap();

    // Solve the system of linear equations with the decompose matrix
    let _x1: Vector<f64> = lu_dec.solve(&b).unwrap();

    // Solve it directly
    let _x2: Vector<f64> = a.solve(&b).unwrap();
}
