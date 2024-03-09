// !# Auction Result Desktop App


pub mod components;
use std::rc::Rc;

use components::connect_cusip_handler;
use slint::VecModel;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui: AppWindow = AppWindow::new()?; 

    ui.set_rows(Rc::new(VecModel::default()).into());
    connect_cusip_handler(&ui);

    let weak = ui.as_weak();
    ui.on_close(move || weak.unwrap().window().hide().unwrap());
    ui.run()
}
