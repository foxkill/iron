use chrono::NaiveDateTime;
// !# Graph Generation
use plotters::prelude::*;
use slint::SharedPixelBuffer;

use super::qualitymodel::{DataPair, TakeDown};
/*evcxr_figure((640, 480), |root| {
    let mut chart = ChartBuilder::on(&root)
        .caption("Histogram", ("Arial", 20).into_font())
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(0u32..100u32, 0f64..0.5f64)?;

    chart.configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .y_labels(5)
        .x_label_formatter(&|x| format!("{:.1}", *x as f64 / 100.0))
        .y_label_formatter(&|y| format!("{}%", (*y * 100.0) as u32))
        .draw()?;

    let hist = Histogram::vertical(&chart)
        .style(RED.filled())
        .margin(0)
        .data(random_samples.iter().map(|(x,_)| ((x*100.0) as u32, 0.01)));

    let _ = chart.draw_series(hist);

    Ok(())
}).style("width:100%")*/

pub fn empty_image() -> slint::Image {
    let mut pixel_buffer = SharedPixelBuffer::new(1, 1);
    let size = (pixel_buffer.width(), pixel_buffer.height());

    let backend = BitMapBackend::with_buffer(pixel_buffer.make_mut_bytes(), size);
    let root_area = backend.into_drawing_area();

    root_area.fill(&WHITE).expect("error filling drawing area");

    drop(root_area);

    slint::Image::from_rgb8(pixel_buffer)
}

pub fn draw_barchart(
    width: f32,
    height: f32,
    datavec: Vec<DataPair>,
    takedown: TakeDown,
) -> slint::Image {
    // println!("w: {} - h: {}", width, height);
    let mut pixel_buffer = SharedPixelBuffer::new(width as u32, height as u32);
    let size = (pixel_buffer.width(), pixel_buffer.height());

    let backend = BitMapBackend::with_buffer(pixel_buffer.make_mut_bytes(), size);
    let root_area = backend.into_drawing_area();

    root_area.margin(5, 5, 5, 5);
    root_area.fill(&WHITE).expect("error filling drawing area");

    let maxopt = datavec
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
        .value
        .ceil();

    let mut ctx = ChartBuilder::on(&root_area)
        .margin_left(20)
        .margin_bottom(5)
        .margin_top(5)
        .margin_right(5)
        .set_label_area_size(LabelAreaPosition::Left, 20)
        .set_label_area_size(LabelAreaPosition::Bottom, 20)
        .caption(takedown.to_string(), ("sans-serif", 15))
        // .build_cartesian_2d((0..datavec.len() - 1).into_segmented(), 0..maxopt.unwrap().x_axis as i32)
        .build_cartesian_2d((0..datavec.len() as i32 - 1).into_segmented(), 0.0..maxopt)
        .expect("Error building coordinate system");

    ctx.configure_mesh()
        .x_label_formatter(&|x| {
            let index = match x {
                SegmentValue::Exact(coord) => *coord,
                SegmentValue::CenterOf(coord) => *coord,
                SegmentValue::Last => 0i32
            };
            let date = datavec.get(index as usize).unwrap();
            date.date.format("%b-%y").to_string()
        })
        .y_label_formatter(&|y| match takedown {
            TakeDown::BidToCover => format!("{:.2}", *y),
            _ => format!("{:.1}%", *y),
        })
        .disable_x_mesh()
        // .disable_mesh()
        .draw()
        .expect("error drawing axis");
    
    let values = datavec.iter().map(|elem| elem.value).collect::<Vec<_>>();
    ctx.draw_series((0..).zip(values.iter()).map(|(x, y)| {
        let x0 = SegmentValue::Exact(x);
        let x1 = SegmentValue::Exact(x + 1);
        let mut bar = Rectangle::new([(x0, 0.0), (x1, *y)], RGBColor(23, 161, 206).filled());
        bar.set_margin(0, 0, 20, 20);
        bar
    }))
    .expect("Error drawing series data");

    drop(ctx);
    drop(root_area);

    slint::Image::from_rgb8(pixel_buffer)
}
