// !# Graph Generation
use plotters::{prelude::*, style::text_anchor::{HPos, Pos, VPos}};
use slint::SharedPixelBuffer;

use crate::models::qualitymodel::{DataPair, TakeDown};


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
    auction_type: impl Into<String>,
    takedown: TakeDown,
) -> slint::Image {
    let area_color = RGBColor(0, 30, 89);
    // println!("w: {} - h: {}", width, height);
    let mut pixel_buffer = SharedPixelBuffer::new(width as u32, height as u32);
    let size = (pixel_buffer.width(), pixel_buffer.height());

    let backend = BitMapBackend::with_buffer(pixel_buffer.make_mut_bytes(), size);
    let root_area = backend.into_drawing_area();

    root_area.margin(5, 5, 5, 5);
    root_area.fill(&area_color).expect("error filling drawing area");
    let mut maxopt = datavec
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
        .value
        .ceil();

    maxopt *= 1.1;

    let mut ctx = ChartBuilder::on(&root_area)
        .margin_left(20)
        .margin_bottom(5)
        .margin_top(10)
        .margin_right(5)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption(format!("{} {}", auction_type.into(), takedown), ("sans-serif", 15).with_color(WHITE))
        // .build_cartesian_2d((0..datavec.len() - 1).into_segmented(), 0..maxopt.unwrap().x_axis as i32)
        .build_cartesian_2d((0..datavec.len() as i32 - 1).into_segmented(), 0.0..maxopt)
        .expect("Error building coordinate system");

    // ctx.configure_axes().light_grid_style(BLACK.mix(0.15)).draw().unwrap();
    // ctx.configure_series_labels().border_style(WHITE).draw().unwrap();

    ctx.configure_mesh()
        .bold_line_style(RGBColor(12, 53, 121))
        .axis_style(WHITE)
        .label_style(("sans-serif", 18).into_font().color(&WHITE))
        .light_line_style(area_color)
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
        .y_labels(6)
        .disable_x_mesh()
        // .disable_mesh()
        .draw()
        .expect("error drawing axis");
    
    // Blue: 23, 161, 206
    let values = datavec.iter().map(|elem| elem.value).collect::<Vec<_>>();
    ctx.draw_series((0..).zip(values.iter()).map(|(x, y)| {
        let x0 = SegmentValue::Exact(x);
        let x1 = SegmentValue::Exact(x + 1);
        let mut bar = Rectangle::new([(x0, 0.0), (x1, *y)], RGBColor(238, 176, 0).filled());
        bar.set_margin(0, 0, 20, 20);
        bar
    }))
    .expect("Error drawing series data");

    ctx.draw_series((0i32..).zip(values.iter()).map(|(x, y)| {
        let pos = Pos::new(HPos::Center, VPos::Top);
        let style = ("Arial", 16).into_font().color(&WHITE).pos(pos);
        let x0 = SegmentValue::CenterOf(x);
        let y0 = *y;

        let value = match takedown {
            TakeDown::BidToCover => format!("{:.2}", y0),
            _ => format!("{:.2}%", y0),
        };

        EmptyElement::at((x0, y0))
            + Text::new(value, (0, -18), style)
        
    })).expect("Error drawing text values");


    drop(ctx);
    drop(root_area);

    slint::Image::from_rgb8(pixel_buffer)
}
