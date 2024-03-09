// !# Contains all relevant models.
#![allow(dead_code, unused_imports)]
#![allow(clippy::vec_init_then_push)]

use std::rc::Rc;

use auctionresult::Get;
use auctionresult::treasury::TreasuryAccess;

use slint::{ModelRc, SharedString, StandardListViewItem, VecModel};

use crate::{rows, SlMap};

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
        
        for treasury in treasuries {

            let items = Rc::new(VecModel::default());
            items.push("Treasury ---------".into());
            items.push("".into());
            row_data.push(items.into());

            let items = Rc::new(VecModel::default());
            items.push("Security Term:".into());
            items.push(treasury.get_security_term().into());
            row_data.push(items.into());

            let items = Rc::new(VecModel::default());
            items.push("CUSIP".into());
            items.push(treasury.cusip().into());

            row_data.push(items.into());
        }

        row_data
    }
}