// !# Auction Result Desktop App

use std::rc::Rc;

use slint::{MapModel, ModelRc, SharedString, VecModel};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let mut rows: Vec<SlMap> = Vec::<SlMap>::new();

    rows.push(SlMap { key: SharedString::from("CUSIP"), value: SharedString::from("91282CJZ5")});

    let vm = VecModel::from(rows);

    let ui = AppWindow::new()?;
    let mrc = ModelRc::from(Rc::new(vm));
    ui.set_rows(mrc);
    ui.run()
}
