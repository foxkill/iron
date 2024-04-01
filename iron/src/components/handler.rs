// !# Contain all event handlers

// use slint::{ModelRc, SharedString, VecModel};

slint::include_modules!();

/// Handle a new given cusip.
pub fn handle_cusip(cusip: impl Into<String>) {
    let cu = cusip.into();
    println!("{:?}", cu);

    // let mut rows: Vec<SlMap> = Vec::<SlMap>::new();
    // rows.push(SlMap { key: SharedString::from("CUSIP"), value: SharedString::from(cu)});
    // let vm = VecModel::from(rows);

    // let mrc = ModelRc::from(Rc::new(vm));
    // ui.set_rows(mrc);
}

