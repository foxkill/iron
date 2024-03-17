// !# Delivers the quality tab data.

use auctionresult::{tenor::Tenor, Latest, SecurityType};

pub struct QualityModel;

impl QualityModel {
    pub fn query(security_type: SecurityType, lookback: usize, takedown: u32) {
        let tenor = Tenor::default();
        let latest_command = Latest::new(security_type, lookback, tenor);
    }
}