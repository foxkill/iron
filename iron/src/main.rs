// !# Auction Result Desktop App
// #![allow(unused_imports)]
pub mod components;
pub mod models;

use components::connect_clipboard_handler;
use components::connect_validate_cusip;
use components::connect_cusip_handler;
use components::connect_barchart;
use components::connect_close;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui: AppWindow = AppWindow::new()?; 

    connect_validate_cusip(&ui);
    connect_cusip_handler(&ui);
    connect_close(&ui);
    connect_barchart(&ui);
    connect_clipboard_handler(&ui);

    ui.run()
}