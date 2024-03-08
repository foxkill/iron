// !# Contains all relevant models.
#![allow(dead_code)]
use std::rc::Rc;

use slint::{ModelRc, VecModel};

use crate::SlMap;

type Model = ModelRc<Vec::<SlMap>>;

#[derive(Default)]
pub struct SlMapModel { 
    inner: Model,
}


impl SlMapModel {
    pub fn new() -> Self {
        let rows: Vec<SlMap> = vec![];
        // rows.push(SlMap { key: SharedString::from("CUSIP"), value: SharedString::from(cu)});
        // let vm = VecModel::from(rows);

        // let mrc = ModelRc::from(Rc::new(vm));
        // ui.set_rows(mrc);
        Self {
            // inner: ModelRc::from(Rc::new(VecModel::from(Vec::<SlMap>::new())))
            inner: ModelRc::default()
        }
    }

    // pub fn get(&self) -> &Vec<SlMap> {
    //     self.inner
    // }
}