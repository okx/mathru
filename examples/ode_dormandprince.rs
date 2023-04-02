use mathru::
{
    algebra::linear::Vector,
    analysis::differential_equation::ordinary::
    {
        problem::Euler, 
        ExplicitODE,
        solver::runge_kutta::explicit::adaptive::DormandPrince54,
        solver::runge_kutta::ProportionalControl
    },
};
use plotters::prelude::*;

fn main()
{
    // Create an ODE instance
    let problem: Euler<f64> = Euler::default();

    let (x_start, x_end) = problem.time_span();

    // Create a ODE solver instance
    let h_0: f64 = 0.0001;
    let fac: f64 = 0.9;
    let fac_min: f64 = 0.01;
    let fac_max: f64 = 2.0;
    let n_max: u32 = 800;
    let abs_tol: f64 = 10e-7;
    let rel_tol: f64 = 10e-6;

    let solver: ProportionalControl<f64> = ProportionalControl::new(n_max, h_0, fac, fac_min, fac_max, abs_tol, rel_tol);

    // Solve ODE
    let (x, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem, &DormandPrince54::default()).unwrap();

    //Create chart
    let mut graph_x1: Vec<(f64, f64)> = Vec::with_capacity(x.len());
    let mut graph_x2: Vec<(f64, f64)> = Vec::with_capacity(x.len());
    let mut graph_x3: Vec<(f64, f64)> = Vec::with_capacity(x.len());

    for i in 0..x.len()
    {
        let x_i = x[i];
        graph_x1.push((x_i, y[i][0]));
        graph_x2.push((x_i, y[i][1]));
        graph_x3.push((x_i, y[i][2]));
    }

    let root_area =
        BitMapBackend::new("./figures/ode_dormandprince.png", (1200, 800)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx =
        ChartBuilder::on(&root_area).margin(20)
                                    .set_label_area_size(LabelAreaPosition::Left, 40)
                                    .set_label_area_size(LabelAreaPosition::Bottom, 40)
                                    .caption("ODE solved with Dormand-Prince", ("Arial", 40))
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
