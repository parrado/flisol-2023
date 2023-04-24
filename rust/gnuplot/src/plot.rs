use gnuplot::{Figure, Caption, Color};
use std::{thread, time};
use rand::Rng;
use plotters::prelude::*;
use plotters::style::*;


fn main()->Result<(), Box<dyn std::error::Error>>{

const N:usize=10;


let mut t=vec![0.0;N];
let mut u=vec![0.0;N];

for i in 0..u.len(){
    t[i]=i as f64;
}


let mut rng = rand::thread_rng();



    let value:f64= rng.gen();
    u.pop();
    u.push(value);
  

    thread::sleep(time::Duration::from_secs(1));

    let root = BitMapBackend::new("plotters-doc-data/0.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
            &RED,
        ))?
        .label("y = x^2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE)
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
    


}