use mathru::{
    algebra::linear::{Matrix, Vector},
    optimization::{LevenbergMarquardt, Optim},
    statistics::distrib::{Distribution, Normal},
    *,
};
use plotters::prelude::*;

///y = a + b * exp(c * t) = f(t)
pub struct Example
{
    x: Vector<f64>,
    y: Vector<f64>,
}

impl Example
{
    pub fn new(x: Vector<f64>, y: Vector<f64>) -> Example
    {
        Example { x, y }
    }

    pub fn function(x: f64, beta: &Vector<f64>) -> f64
    {
        let beta_0: f64 = beta[0];
        let beta_1: f64 = beta[1];
        let beta_2: f64 = beta[2];
        let f_x: f64 = beta_0 + beta_1 * (beta_2 * x).exp();

        return f_x;
    }
}

impl Optim<f64> for Example
{
    // y(x_i) - f(x_i)
    fn eval(&self, beta: &Vector<f64>) -> Vector<f64>
    {
        let f_x = self.x.clone().apply(&|x: &f64| Example::function(*x, beta));
        let r: Vector<f64> = &self.y - &f_x;
        return vector![r.dotp(&r)];
    }

    fn jacobian(&self, beta: &Vector<f64>) -> Matrix<f64>
    {
        let (x_m, _x_n) = self.x.dim();
        let (beta_m, _beta_n) = beta.dim();

        let mut jacobian_f: Matrix<f64> = Matrix::zero(x_m, beta_m);

        let f_x = self.x.clone().apply(&|x: &f64| Example::function(*x, beta));
        let residual: Vector<f64> = &self.y - &f_x;

        for i in 0..x_m
        {
            let beta_1: f64 = beta[1];
            let beta_2: f64 = beta[2];

            let x_i: f64 = self.x[i];

            jacobian_f[[i, 0]] = 1.0;
            jacobian_f[[i, 1]] = (beta_2 * x_i).exp();
            jacobian_f[[i, 2]] = beta_1 * x_i * (beta_2 * x_i).exp();
        }

        let jacobian: Matrix<f64> = (residual.transpose() * jacobian_f * -2.0).into();
        return jacobian;
    }
}

fn main()
{
    let num_samples: usize = 100;

    let noise: Normal<f64> = Normal::new(0.0, 0.05);

    let mut t_vec: Vec<f64> = Vec::with_capacity(num_samples);
    // Start time
    let t_0 = 0.0f64;
    // End time
    let t_1 = 5.0f64;

    let mut x_vec: Vec<f64> = Vec::with_capacity(num_samples);

    // True function parameters
    let beta: Vector<f64> = vector![0.5; 5.0; -1.0];

    for i in 0..num_samples
    {
        let t_i: f64 = (t_1 - t_0) / (num_samples as f64) * (i as f64);

        //Add some noise
        x_vec.push(Example::function(t_i, &beta) + noise.random());
        t_vec.push(t_i);
    }

    let t: Vector<f64> = Vector::new_column(t_vec.clone());
    let x: Vector<f64> = Vector::new_column(x_vec.clone());

    let example_function = Example::new(t, x);

    let optim: LevenbergMarquardt<f64> = LevenbergMarquardt::new(100, 0.3, 0.95);

    // Fit parameter
    let beta_0: Vector<f64> = vector![-1.5; 1.0; -2.0];
    let beta_opt: Vector<f64> = optim.minimize(&example_function, &beta_0).unwrap().arg();

    //Create chart
    let mut graph_x: Vec<(f64, f64)> = Vec::with_capacity(x_vec.len());
    let mut graph_x_hat: Vec<(f64, f64)> = Vec::with_capacity(x_vec.len());

    for i in 0..x_vec.len()
    {
        let t_i = t_vec[i];
        graph_x.push((t_i, x_vec[i]));

        let x_hat = Example::function(t_i, &beta_opt);
        graph_x_hat.push((t_i, x_hat));
    }

    let root_area = BitMapBackend::new("./figures/fit_lm.png", (1200, 800)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area).margin(20)
                                              .set_label_area_size(LabelAreaPosition::Left, 40)
                                              .set_label_area_size(LabelAreaPosition::Bottom, 40)
                                              .caption("Parameter fitting with Levenberg Marquardt",
                                                       ("Arial", 40))
                                              .build_cartesian_2d(t_0..t_1, -0.5f64..6.0f64)
                                              .unwrap();

    ctx.configure_mesh()
       .x_desc("Time t")
       .axis_desc_style(("sans-serif", 25).into_font())
       .draw()
       .unwrap();

    ctx.draw_series(LineSeries::new(graph_x, &BLACK)).unwrap();

    ctx.draw_series(LineSeries::new(graph_x_hat, &RED)).unwrap();
}
