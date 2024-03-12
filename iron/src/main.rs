// !# Auction Result Desktop App
// #![allow(unused_imports)]
pub mod components;

use components::barchart;
use components::connect_close;
use components::connect_cusip_handler;
use components::connect_validate_cusip;
// use components::render_plot;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui: AppWindow = AppWindow::new()?; 

    connect_validate_cusip(&ui);
    connect_cusip_handler(&ui);
    connect_close(&ui);

    ui.global::<AppState>().on_render_plot(|w, h| {
        barchart(w, h)
    });
    ui.run()
}