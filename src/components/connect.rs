// !# Connects widgets to handlers.

// use slint::{ModelRc, SharedString, VecModel};
#![allow(clippy::vec_init_then_push)]

use slint::{ComponentHandle, ModelRc, VecModel};
use crate::{AppState, AppWindow};

use super::SlMapModel;

pub fn connect_cusip_handler(app: &AppWindow) {
    let ui = app.as_weak();
    app.global::<AppState>().on_set_cusip(move |cusip| {
        let this = ui.upgrade().unwrap();
        let rows = SlMapModel::create(cusip.as_str());
        this.set_rows(ModelRc::new(VecModel::from(rows)));
    });
}