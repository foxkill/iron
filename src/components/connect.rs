// !# Connects widgets to handlers.

// use slint::{ModelRc, SharedString, VecModel};

use slint::ComponentHandle;
use crate::{AppWindow, AppState};

pub fn connect_cusip_handler(app: &AppWindow) {
    let ui = app.as_weak();
    app.global::<AppState>().on_set_cusip(move |cusip| {
        let _app = ui.upgrade().unwrap();
        
        println!("{:?}", cusip);
    });
}