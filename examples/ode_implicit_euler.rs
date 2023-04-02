use mathru::{
    algebra::linear::Vector,
    analysis::differential_equation::ordinary::{problem::Euler, solver::runge_kutta::implicit::ImplicitEuler, ImplicitODE},
};
use plotters::prelude::*;

fn main()
{
    // Create an ODE instance
    let problem: Euler<f64> = Euler::default();
    let solver: ImplicitEuler<f64> = ImplicitEuler::new(0.0001);

    let (x, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();

    let (x_start, x_end) = problem.time_span();

    //Create chart
    let mut graph_x1: Vec<(f64, f64)> = Vec::with_capacity(x.len());
    let mut graph_x2: Vec<(f64, f64)> = Vec::with_capacity(x.len());
    let mut graph_x3: Vec<(f64, f64)> = Vec::with_capacity(x.len());

    for i in 0..y.len()
    {
        let x_i = x[i];

        graph_x1.push((x_i, y[i][0]));
        graph_x2.push((x_i, y[i][1]));
        graph_x3.push((x_i, y[i][2]));
    }

    let root_area =
        BitMapBackend::new("./figures/ode_implicit_euler.png", (1200, 800)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx =
        ChartBuilder::on(&root_area).margin(20)
                                    .set_label_area_size(LabelAreaPosition::Left, 40)
                                    .set_label_area_size(LabelAreaPosition::Bottom, 40)
                                    .caption("ODE solved with implicit Euler", ("Arial", 40))
                                    .build_cartesian_2d(x_start..x_end, -1.0f64..1.5f64)
                                    .unwrap();

    ctx.configure_mesh()
       .x_desc("Time t")
       .axis_desc_style(("sans-serif", 25).into_font())
       .draw()
       .unwrap();

    ctx.draw_series(LineSeries::new(graph_x1, &BLACK)).unwrap();

    ctx.draw_series(LineSeries::new(graph_x2, &RED)).unwrap();

    ctx.draw_series(LineSeries::new(graph_x3, &BLUE)).unwrap();
}
