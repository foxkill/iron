// !# Delivers the quality tab data.

use std::str::FromStr;

use auctionresult::{tenor::Tenor, treasury::{AuctionResultError, Treasury, TreasuryAccess}, Latest, SecurityType};
use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct DataPair {
    x_axis: f64,
    y_axis: NaiveDateTime,
}

pub enum TakeDown {
    BidToCover,
    PrimaryDealers,
    Indirects,
    Directs,
}

impl FromStr for TakeDown {
    type Err = AuctionResultError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let this = match s {
            "Bid To Cover" => TakeDown::BidToCover,
            "Primary Dealers" => TakeDown::BidToCover,
            "Indirects" => TakeDown::BidToCover,
            "Directs" => TakeDown::BidToCover,
            _ => return Err(AuctionResultError::OutOfBounds),
        };

        Ok(this)
    }
}

pub struct QualityModel;

impl QualityModel {
    pub fn query(security_type: SecurityType, lookback: usize, takedown: TakeDown) -> Result<Vec<DataPair>, AuctionResultError> {
        let tenor = Tenor::default();
        let latest_command = Latest::new(security_type, lookback, tenor);

        let result = latest_command.get();

        let Ok(treasuries) = result else {  
            return Err(result.unwrap_err());
        };

        Ok(vec![])

        // let fptr = match takedown {
        //     TakeDown::BidToCover => |value: &Treasury| { 
        //         value.get_bid_to_cover_ratio()
        //     }
        //     TakeDown::PrimaryDealers => |value: &Treasury| value.get_bid_to_cover_ratio(),
        //     TakeDown::Indirects => |value: &Treasury| value.get_bid_to_cover_ratio(),
        //     Takedown::Directs => |value: &Treasury| value.get_bid_to_cover_ratio(),
        // }

        // let result = treasuries.iter().map(fptr).collect::<Vec<_>>();

    }
}

#[cfg(test)]
mod tests {
    use super::DataPair;
    use auctionresult::treasury::Treasury;

    #[test]
    fn it_should_return_usable_results() {
        let json = include_str!("dd.json");
        let result: Vec<Treasury> =
            serde_json::from_str(json).unwrap_or_else(|_| vec![Treasury::default()]);
        println!("{:#?}", result);
        let btc = result
            .iter()
            .map(|value| {
                DataPair {
                    x_axis: value.get_bid_to_cover_ratio(),
                    y_axis: value.get_issue_date(),
                }
            })
            .collect::<Vec<_>>();
        println!("{:?}", btc);
    }
}
