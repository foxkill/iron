// !# Delivers the quality tab data.

use std::{fmt::Display, str::FromStr};

use auctionresult::{
    tenor::Tenor,
    treasury::{AuctionResultError, Treasuries, Treasury, TreasuryAccess},
    Latest, SecurityType,
};
use chrono::{Months, NaiveDateTime, Utc};

#[derive(Debug, PartialEq, PartialOrd, Default)]
pub struct DataPair {
    pub value: f64,
    pub date: NaiveDateTime,
}

pub enum TakeDown {
    BidToCover,
    PrimaryDealers,
    Indirects,
    Directs,
    None,
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

impl From<i32> for TakeDown {
    fn from(value: i32) -> Self {
        match value {
            0 => TakeDown::BidToCover,
            1 => TakeDown::PrimaryDealers,
            2 => TakeDown::Indirects,
            3 => TakeDown::Directs,
            _ => TakeDown::None,
        }
    }
}

impl Display for TakeDown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_repr = match self {
            TakeDown::BidToCover => "Bid To Cover",
            TakeDown::PrimaryDealers => "Primary Dealers",
            TakeDown::Indirects => "Indirects",
            TakeDown::Directs => "Directs",
            TakeDown::None => "None",
        };

        write!(f, "{}", str_repr)
    }
}
pub struct QualityModel;
trait DataPairTrait {
    fn get_data_pair(&self, takedown: &TakeDown) -> DataPair;
}

impl DataPairTrait for Treasury {
    fn get_data_pair(&self, takedown: &TakeDown) -> DataPair {
        match takedown {
            TakeDown::BidToCover => DataPair {
                value: self.get_bid_to_cover_ratio(),
                date: self.get_issue_date(),
            },
            TakeDown::PrimaryDealers => DataPair {
                value: self.get_percentage_debt_purchased_by_dealers(),
                date: self.get_issue_date(),
            },
            TakeDown::Indirects => DataPair {
                value: self.get_percentage_debt_purchased_by_indirects(),
                date: self.get_issue_date(),
            },
            TakeDown::Directs => DataPair {
                value: self.get_percentage_debt_purchased_by_directs(),
                date: self.get_issue_date(),
            },
            TakeDown::None => DataPair { value: 0.0, date: self.get_issue_date() },
        }
    }
}
impl QualityModel {
    pub fn query(
        auction_type: impl Into<String>,
        lookback: impl Into<String>,
        _takedown: TakeDown,
    ) -> Result<Treasuries, AuctionResultError> {
        // TODO: map err to AuctionResultError.
        let lookback_auctions = lookback.into().parse::<usize>().unwrap();

        let days = QualityModel::to_days(lookback_auctions);

        let stype = auction_type.into();

        // Get the tenor from the auction type.
        let tenor = Tenor::parse(stype.to_owned())?;

        // Get the security type from the auction type.
        let st = QualityModel::to_security_type(stype)?;

        let latest_command = Latest::new(st, days, tenor);

        let result = latest_command.get();

        let Ok(treasuries) = result else {
            return Err(result.unwrap_err());
        };

        Ok(treasuries)
    }

    /// Use the number of auction to look back to get the equivalent number of days.
    pub fn to_days(lookback_auctions: usize) -> usize {
        let now = Utc::now();

        let at_that_time = now.checked_sub_months(Months::new(lookback_auctions as u32));
        let diff = now.with_timezone(&Utc) - at_that_time.unwrap().with_timezone(&Utc);

        diff.num_days() as usize
    }

    pub fn process_treasuries(treasuries: &[Treasury], takedown: &TakeDown) -> Vec<DataPair> {
        treasuries.iter().map(|t| t.get_data_pair(takedown)).collect()
    }

    fn to_security_type(
        security_type: impl Into<String>,
    ) -> Result<SecurityType, AuctionResultError> {
        let tenor = Tenor::parse(security_type)?;
        let security = match (tenor.security(), tenor.shortcut()) {
            (20..=30, "y") => SecurityType::Bond,
            (2..=10, "y") => SecurityType::Note,
            _ => return Err(AuctionResultError::NoTreasury),
        };

        Ok(security)
    }
}



#[cfg(test)]
mod tests {
    use super::DataPair;
    use auctionresult::treasury::Treasury;

    #[test]
    fn it_should_return_usable_results() {
        let json = include_str!("../assets/dd.json");
        let result: Vec<Treasury> =
            serde_json::from_str(json).unwrap_or_else(|_| vec![Treasury::default()]);
        println!("{:#?}", result);
        let btc = result
            .iter()
            .map(|value| DataPair {
                value: value.get_bid_to_cover_ratio(),
                date: value.get_issue_date(),
            })
            .collect::<Vec<_>>();
        println!("{:?}", btc);
    }
}
