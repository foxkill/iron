// !# Delivers the quality tab data.

use std::{fmt::Display, str::FromStr};

use auctionresult::{
    tenor::Tenor,
    treasury::{security_type, AuctionResultError, Treasury, TreasuryAccess},
    Latest, SecurityType,
};
use chrono::{Months, NaiveDateTime, Utc};

#[derive(Debug, PartialEq, PartialOrd, Default)]
pub struct DataPair {
    pub x_axis: f64,
    pub y_axis: NaiveDateTime,
}

// impl Default for DataPair {
//     fn default() -> Self {
//         Self { x_axis: Default::default(), y_axis: Default::default() }
//     }
// }

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

impl QualityModel {
    pub fn query(
        auction_type: impl Into<String>,
        lookback: impl Into<String>,
        takedown: TakeDown,
    ) -> Result<Vec<DataPair>, AuctionResultError> {
        let timerange = lookback.into().parse::<u32>().unwrap();

        let now = Utc::now();

        let at_that_time = now.checked_sub_months(Months::new(timerange));
        let diff = now.with_timezone(&Utc) - at_that_time.unwrap().with_timezone(&Utc);
        let days = diff.num_days() as usize;

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

        let fptr = match takedown {
            TakeDown::BidToCover => |value: &Treasury| DataPair {
                x_axis: value.get_bid_to_cover_ratio(),
                y_axis: value.get_issue_date(),
            },
            TakeDown::PrimaryDealers => |value: &Treasury| DataPair {
                x_axis: value.get_percentage_debt_purchased_by_dealers(),
                y_axis: value.get_issue_date(),
            },
            TakeDown::Indirects => |value: &Treasury| DataPair {
                x_axis: value.get_percentage_debt_purchased_by_indirects(),
                y_axis: value.get_issue_date(),
            },
            TakeDown::Directs => |value: &Treasury| DataPair {
                x_axis: value.get_percentage_debt_purchased_by_directs(),
                y_axis: value.get_issue_date(),
            },
            TakeDown::None => |value: &Treasury| DataPair {
                x_axis: 0.,
                y_axis: value.get_issue_date(),
            },
        };

        let result = treasuries.iter().map(fptr).collect::<Vec<_>>();

        Ok(result)
    }

    fn to_security_type(security_type: impl Into<String>) -> Result<SecurityType, AuctionResultError> {
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
        let json = include_str!("dd.json");
        let result: Vec<Treasury> =
            serde_json::from_str(json).unwrap_or_else(|_| vec![Treasury::default()]);
        println!("{:#?}", result);
        let btc = result
            .iter()
            .map(|value| DataPair {
                x_axis: value.get_bid_to_cover_ratio(),
                y_axis: value.get_issue_date(),
            })
            .collect::<Vec<_>>();
        println!("{:?}", btc);
    }
}
