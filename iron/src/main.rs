// !# Auction Result Desktop App
// #![allow(unused_imports)]
pub mod components;


use components::connect_cusip_handler;
use components::connect_close;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui: AppWindow = AppWindow::new()?; 

    connect_cusip_handler(&ui);
    connect_close(&ui);

    ui.run()
}
