use mathru::{algebra::linear::Matrix, matrix};

fn main()
{
    let a: Matrix<f64> = matrix![   1.0, -2.0, 3.0;
                                        2.0, -5.0, 12.0;
                                        0.0, 2.0, -10.0];

    let (_l, _u, _p): (Matrix<f64>, Matrix<f64>, Matrix<f64>) = a.dec_lu().unwrap().lup();
}
