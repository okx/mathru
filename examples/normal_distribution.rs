use mathru::statistics::distrib::{Continuous, Normal};
use plotters::prelude::*;

fn main()
{
    let mean: f64 = 1.0;
    let variance: f64 = 1.0;
    let distrib: Normal<f64> = Normal::new(mean, variance);

    let x_start: f64 = -2.0;
    let x_end: f64 = 4.0;
    let length: usize = 1000;

    let mut graph_1: Vec<(f64, f64)> = Vec::with_capacity(length);
    for i in 0..length
    {
        let x: f64 = (x_end - x_start) / (length as f64) * (i as f64) + x_start;
        graph_1.push((x, distrib.pdf(x)));
    }

    let root_area =
        BitMapBackend::new("./figures/pdf_normal_distribution.png", (1200, 800)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .margin(20)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption(
            "Probability distribution function of Normal distribution",
            ("Arial", 40),
        )
        .build_cartesian_2d(x_start..x_end, 0.0f64..0.45f64)
        .unwrap();

    ctx.configure_mesh()
       .x_desc("x")
       .axis_desc_style(("sans-serif", 25).into_font())
       .draw()
       .unwrap();

    ctx.draw_series(LineSeries::new(graph_1, &BLUE)).unwrap();
}
