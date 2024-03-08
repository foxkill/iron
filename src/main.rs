// !# Auction Result Desktop App


pub mod components;
use components::connect_cusip_handler;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui: AppWindow = AppWindow::new()?;

    connect_cusip_handler(&ui);

    ui.run()
}
