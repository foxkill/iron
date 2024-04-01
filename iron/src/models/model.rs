// !# Contains all relevant models.
#![allow(dead_code, unused_imports)]
#![allow(clippy::vec_init_then_push)]

use std::rc::Rc;

use auctionresult::treasury::TreasuryAccess;
use auctionresult::{treasury::Treasury, Get};

use slint::{ModelRc, SharedString, StandardListViewItem, VecModel};

use crate::{add, rows, SlMap};

enum TabState {
    Details,
}

pub trait SlintModelTrait {
    fn to_slint_model(&self, counter: usize) -> Rc<VecModel<StandardListViewItem>>;
}

impl SlintModelTrait for Treasury {
    fn to_slint_model(&self, counter: usize) -> Rc<VecModel<StandardListViewItem>> {
        // let row_data: Rc<VecModel<slint::ModelRc<StandardListViewItem>>> = Rc::new(VecModel::default());

        let col = Rc::new(VecModel::default());
        col.push(slint::format!("{}", counter + 1).into());
        col.push(self.get_original_security_term().into());
        col.push(self.cusip().into());
        col.push((if self.is_reopening() { "Yes" } else { "No" }).into());
        col.push(slint::format!("{}", self.get_security_type()).into());
        col.push(slint::format!("{:.2}%", self.get_bid_to_cover_ratio()).into());
        col.push(
            slint::format!(
                "{:.2}%",
                self.get_percentage_debt_purchased_by_dealers()
            )
            .into(),
        );
        col.push(
            slint::format!(
                "{:.2}%",
                self.get_percentage_debt_purchased_by_directs()
            )
            .into(),
        );
        col.push(
            slint::format!(
                "{:.2}%",
                self.get_percentage_debt_purchased_by_indirects()
            )
            .into(),
        );
        col.push(slint::format!("{:.3}%", self.get_high_yield()).into());
        col.push(slint::format!("{:.3}%", self.get_interest_rate()).into());

        col
    }
}