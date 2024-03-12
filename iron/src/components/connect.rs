// !# Connects widgets to handlers.


use auctionresult::validate_cusip;
use slint::ComponentHandle;
use crate::{AppState, AppWindow, SecuritiesTableAdapter};

use super::SlMapModel;

pub fn connect_validate_cusip(app: &AppWindow) {
    let ui = app.as_weak();
    app.global::<AppState>().on_validate_cusip({ move |cusip| {
        let this = ui.upgrade().unwrap();

        let is_cusip = validate_cusip(cusip);

        this.global::<AppState>().set_valid_cusip(is_cusip);

        is_cusip
    }});
}

pub fn connect_cusip_handler(app: &AppWindow) {
    let ui = app.as_weak();
    app.global::<AppState>().on_set_cusip(move |cusip| {
        let this = ui.upgrade().unwrap();
        let rows = SlMapModel::details(cusip.as_str());

        println!("Cur Index: {}", this.get_tab_index());
        this.global::<SecuritiesTableAdapter>().set_row_data(rows.clone().into());
        // this.set_rows(ModelRc::new(VecModel::from(rows)));
    });
}

pub fn connect_close(app: &AppWindow) {
    let weak = app.as_weak();
    app.on_close(move || weak.unwrap().window().hide().unwrap());
}