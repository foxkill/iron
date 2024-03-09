// !# Contains all relevant models.
#![allow(dead_code, unused_imports)]
#![allow(clippy::vec_init_then_push)]

use std::rc::Rc;

use auctionresult::Get;
use auctionresult::treasury::TreasuryAccess;

use slint::{ModelRc, VecModel, SharedString};

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
}