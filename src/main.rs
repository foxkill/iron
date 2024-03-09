// !# Auction Result Desktop App
#![allow(unused_imports)]
pub mod components;

use std::rc::Rc;

use components::connect_cusip_handler;
use components::connect_close;

use slint::VecModel;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui: AppWindow = AppWindow::new()?; 

    // ui.set_rows(Rc::new(VecModel::default()).into());
    connect_cusip_handler(&ui);
    connect_close(&ui);

    ui.run()
}
