use mathru::{algebra::linear::Matrix, matrix};

fn main()
{
    let a: Matrix<f64> = matrix![   1.0, 5.0, 3.0;
                                    1.0, 0.0, -7.0;
                                    3.0, 8.0, 9.0];

    let (_q, _h): (Matrix<f64>, Matrix<f64>) = a.dec_hessenberg().qh();
}
