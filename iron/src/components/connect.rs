// !# Connects widgets to handlers.
use crate::{
    components::barchart::{draw_barchart, empty_image}, models::{model::SlintModelTrait, DetailModel, QualityModel}, AppState, AppWindow
};
use arboard::Clipboard;
use auctionresult::validate_cusip;
use slint::{ComponentHandle, Model, StandardListViewItem, VecModel};
use std::rc::Rc;

use crate::DetailsTableAdapter;
use crate::QualityTableAdapter;

// The index into ModelRc<ModelRc<VecModel<StandardListViewItem>>>
const CUSIP_COLUMN: usize = 2;

pub fn connect_validate_cusip(app: &AppWindow) {
    let ui = app.as_weak();
    app.global::<AppState>().on_validate_cusip({
        move |cusip| {
            let this = ui.upgrade().unwrap();

            let is_cusip = validate_cusip(cusip);

            this.global::<AppState>().set_valid_cusip(is_cusip);

            is_cusip
        }
    });
}

pub fn connect_cusip_handler(app: &AppWindow) {
    let ui = app.as_weak();
    app.global::<AppState>().on_set_cusip(move |cusip| {
        let Some(this) = ui.upgrade() else {
            return 0i32;
        };

        let tabindex = this.get_tab_index();

        println!("Me working on the tabindex: {}", tabindex);

        let Ok(rows) = DetailModel::get(cusip.as_str()) else {
            return 0;
        };

        let row_data: Rc<VecModel<slint::ModelRc<StandardListViewItem>>> =
            Rc::new(VecModel::default());

        for (counter, treasury) in rows.iter().enumerate() {
            let col: Rc<VecModel<StandardListViewItem>> = treasury.to_slint_model(counter);
            row_data.push(col.into());
        }

        this.global::<DetailsTableAdapter>()
            .set_row_data(row_data.clone().into());

        row_data.row_count() as i32
    });
}

pub fn connect_close(app: &AppWindow) {
    let weak = app.as_weak();
    app.on_close(move || weak.unwrap().window().hide().unwrap());
}

pub fn connect_barchart(app: &AppWindow) {
    let ui = app.as_weak();

    app.global::<AppState>()
        .on_render_plot(move |w, h, changed| {
            let this = ui.upgrade().unwrap();
            println!(
                "Im retrieving the cusip: {:?} -> chg: {}",
                this.global::<AppState>().get_cusip(),
                changed
            );

            let takedown = this.global::<AppState>().get_takedown();
            let lookback = this.global::<AppState>().get_lookback();
            let auction_type = this.global::<AppState>().get_auction_type();

            println!(
                "takedown: {:?}, lookback: {:?}, auction_type: {:?}, days: {}",
                takedown,
                lookback,
                auction_type,
                QualityModel::to_days(lookback.parse::<usize>().unwrap())
            );

            let Ok(qm) = QualityModel::query(auction_type, lookback, takedown.into()) else {
                return empty_image();
            };

            let row_data: Rc<VecModel<slint::ModelRc<StandardListViewItem>>> =
                Rc::new(VecModel::default());

            for (counter, treasury) in qm.iter().enumerate() {
                let col: Rc<VecModel<StandardListViewItem>> = treasury.to_slint_model(counter);
                row_data.push(col.into());
            }

            this.global::<QualityTableAdapter>()
                .set_row_data(row_data.clone().into());
            // this.global::<QualityTableAdapter>().set_column_data();

            let datavec = QualityModel::process_treasuries(&qm, &takedown.into());

            // empty_image()
            draw_barchart(
                w,
                h,
                datavec,
                this.global::<AppState>().get_auction_type(),
                takedown.into(),
            )
        });
}

pub fn connect_clipboard_handler(app: &AppWindow) {
    let ui = app.as_weak();
    app.global::<QualityTableAdapter>().on_copy_cusip_to_clipboard(move |index: i32| {
        let Some(this) = ui.upgrade() else {
            eprintln!("Could not upgrade window");
            return;
        };

        let model = this.global::<QualityTableAdapter>().get_row_data();

        let Some(the_row) = model.row_data(index as usize) else {
            eprintln!("Could not get QualityTableAdapter.row_data");
            return;
        };

        let Some(the_model) = slint::Model::as_any(&the_row).downcast_ref::<VecModel<StandardListViewItem>>() else {
            eprintln!("Could not downcast to ModelRc<StandardListviewItem>");
            return;
        };

        let Some(cusip) = the_model.iter().nth(CUSIP_COLUMN) else {
            eprintln!("Could not unpack column data (cusip).");
            return;
        };

        let Ok(mut clipboard) = Clipboard::new() else {
            eprintln!("Could not access clipboard, sorry");
            return;
        };

        let c = cusip.text.as_str();
        let Ok(_) = clipboard.set_text(c) else {
            eprintln!("Couldnt copy cusip to clipboard.");
            return;
        };
    });
}