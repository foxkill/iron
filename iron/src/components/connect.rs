// !# Connects widgets to handlers.


use auctionresult::validate_cusip;
use slint::ComponentHandle;
use crate::{AppState, AppWindow, SecuritiesTableAdapter};

use super::{barchart, SlMapModel};

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

        let tabindex = this.get_tab_index();
        match tabindex {
            0 => {
                let rows = SlMapModel::details(cusip.as_str());
                this.global::<SecuritiesTableAdapter>().set_row_data(rows.clone().into());
                0i32
            }
            _ => 0i32
        };
    });
}

pub fn connect_close(app: &AppWindow) {
    let weak = app.as_weak();
    app.on_close(move || weak.unwrap().window().hide().unwrap());
}

pub fn connect_barchart(app: &AppWindow) {
    let ui = app.as_weak();

    app.global::<AppState>().on_render_plot(move |w, h, changed| {
        let this = ui.upgrade().unwrap();
        println!("Im retrieving the cusip: {:?} -> chg: {}", this.global::<AppState>().get_cusip(), changed);
        
        let data = api_many_items();
        barchart(w, h, changed)
    });

}