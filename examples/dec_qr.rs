use mathru::{algebra::linear::Matrix, matrix};

fn main()
{
    let a: Matrix<f64> = matrix![  6.0, 5.0, 0.0;
                                    5.0, 1.0, 4.0;
                                    0.0, 4.0, 3.0];

    let (_q, _r): (Matrix<f64>, Matrix<f64>) = a.dec_qr().unwrap().qr();
}
