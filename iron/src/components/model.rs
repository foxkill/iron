// !# Contains all relevant models.
#![allow(dead_code, unused_imports)]
#![allow(clippy::vec_init_then_push)]

use std::rc::Rc;

use auctionresult::Get;
use auctionresult::treasury::TreasuryAccess;

use slint::{ModelRc, SharedString, StandardListViewItem, VecModel};

use crate::{rows, add, SlMap};

enum TabState {
    Details,
}

#[derive(Default)]
pub struct SlMapModel { }


impl SlMapModel {
    pub fn create(cusip: &str) -> Vec<SlMap> {
        let get_command = Get::new(cusip.to_owned());
        let treasuries = get_command.get().unwrap();
        let treasury = treasuries.first().unwrap();

        let dealers = format!("{:.2}%", treasury.get_percentage_debt_purchased_by_dealers());
        let bid_to_cover = format!("{:.2}", treasury.get_bid_to_cover_ratio());
        println!("{:?}", treasury);

        rows![
            "Security Term:" => treasury.get_security_term(),
            "CUSIP:" => treasury.cusip(),
            "Reopening:" => if treasury.is_reopening() { "Yes" } else { "No" },
            "Security Type:" => treasury.get_security_type().to_string(),
            "Bid To Cover:" => bid_to_cover,
            "Dealers %:" => dealers
        ]
    }

    pub fn details(cusip: &str) -> Rc<VecModel<slint::ModelRc<StandardListViewItem>>> {
        let row_data: Rc<VecModel<slint::ModelRc<StandardListViewItem>>> = Rc::new(VecModel::default());

        let get_command = Get::new(cusip.to_owned());
        let treasuries = get_command.get().unwrap();

        for (counter, treasury) in treasuries.into_iter().enumerate() {
            let col = Rc::new(VecModel::default());
            col.push(slint::format!("{}", counter + 1).into());
            col.push(treasury.get_original_security_term().into());
            col.push(treasury.cusip().into());
            col.push((if treasury.is_reopening() {"Yes"} else {"No"}).into());
            col.push(slint::format!("{}", treasury.get_security_type()).into());
            col.push(slint::format!("{:.2}%", treasury.get_bid_to_cover_ratio()).into());
            col.push(slint::format!("{:.2}%", treasury.get_percentage_debt_purchased_by_dealers()).into());
            col.push(slint::format!("{:.2}%", treasury.get_percentage_debt_purchased_by_directs()).into());
            col.push(slint::format!("{:.2}%", treasury.get_percentage_debt_purchased_by_indirects()).into());
            col.push(slint::format!("{:.3}%", treasury.get_high_yield()).into());
            col.push(slint::format!("{:.3}%", treasury.get_interest_rate()).into());
            row_data.push(col.into());
        }

        row_data
    }
}