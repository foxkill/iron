// !# Connects widgets to handlers.

// use slint::{ModelRc, SharedString, VecModel};
#![allow(clippy::vec_init_then_push)]

use slint::ComponentHandle;
use crate::{AppState, AppWindow, SecuritiesTableAdapter};

use super::SlMapModel;

pub fn connect_cusip_handler(app: &AppWindow) {
    let ui = app.as_weak();
    app.global::<AppState>().on_set_cusip(move |cusip| {
        let this = ui.upgrade().unwrap();
        let rows = SlMapModel::details(cusip.as_str());

        this.global::<SecuritiesTableAdapter>().set_row_data(rows.clone().into());
        // this.set_rows(ModelRc::new(VecModel::from(rows)));
    });
}

pub fn connect_close(app: &AppWindow) {
    let weak = app.as_weak();
    app.on_close(move || weak.unwrap().window().hide().unwrap());
}