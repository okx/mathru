use mathru::statistics::{
    distrib::{Continuous, Normal, T},
    test::{Test, T as T_Test},
};

/// One sample, two tailed
fn main()
{
    let mean: f64 = 0.0;

    let alpha: f64 = 0.05;

    let x: Vec<f64> = vec![0.3, -0.4, 0.2, -0.4, 0.5];

    let sample_size: u32 = x.len() as u32;

    let normal: Normal<f64> = Normal::from_data(&x);

    println!("x_bar: {}", normal.mean());
    println!("standard deviation: {}", normal.variance().sqrt());

    let test: T_Test<f64> = T_Test::one_sample(&x, mean);

    let t: f64 = T::new((sample_size - 1) as f64).pdf(1.0 - alpha);

    println!("test: {}", test.value());
    println!("t(1.0 - alpha, n - 1) = {}", t);

    if -t <= test.value() && test.value() <= t
    {
        println!("H0 is accepted");
    }
    else
    {
        println!("H0 is rejected and H1 is accepted");
    }
}
