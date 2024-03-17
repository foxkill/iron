// !# Graph Generation
use plotters::prelude::*;
use slint::SharedPixelBuffer;

pub fn barchart(width: f32, height: f32, _changed: i32) -> slint::Image {
    // println!("w: {} - h: {}", width, height);
    let mut pixel_buffer = SharedPixelBuffer::new(width as u32 - 5, height as u32 - 5);
    let size = (pixel_buffer.width(), pixel_buffer.height());

    let backend = BitMapBackend::with_buffer(pixel_buffer.make_mut_bytes(), size);
    let root_area = backend.into_drawing_area();

    root_area.fill(&WHITE).expect("error filling drawing area");

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Bid To Cover", ("sans-serif", 20))
        .build_cartesian_2d(0..50, (0..10).into_segmented())
        .expect("Error building coordinate system");

    ctx.configure_mesh().draw().expect("error drawing axis");

    let data = [25, 37, 15, 32, 45, 33, 32, 10, 0, 21, 5];

    ctx.draw_series((0..).zip(data.iter()).map(|(y, x)| {
        let mut bar = Rectangle::new([
            (0, SegmentValue::Exact(y)), 
            (*x, SegmentValue::Exact(y + 1))
        ], GREEN.filled());
        bar.set_margin(5, 5, 0, 0);
        bar
    }))
    .expect("error drawing series");

    drop(ctx);
    drop(root_area);

    slint::Image::from_rgb8(pixel_buffer)
}