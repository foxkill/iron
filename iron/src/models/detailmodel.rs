// !# The Detail Model

use auctionresult::{treasury::{AuctionResultError, Treasuries, TreasuryAccess}, Get};

#[derive(Default)]
pub struct DetailModel; 

// Rc<VecModel<slint::ModelRc<StandardListViewItem>>> 
impl DetailModel {
    pub fn get(cusip: &str) -> Result<Treasuries, AuctionResultError> {
        // let row_data: Rc<VecModel<slint::ModelRc<StandardListViewItem>>> =
        //     Rc::new(VecModel::default());

        let get_command = Get::new(cusip.to_owned());
        let treasuries = get_command.get()?;

        Ok(treasuries)

        // for (counter, treasury) in treasuries.into_iter().enumerate() {
        //     let col = Rc::new(VecModel::default());
        //     col.push(slint::format!("{}", counter + 1).into());
        //     col.push(treasury.get_original_security_term().into());
        //     col.push(treasury.cusip().into());
        //     col.push((if treasury.is_reopening() { "Yes" } else { "No" }).into());
        //     col.push(slint::format!("{}", treasury.get_security_type()).into());
        //     col.push(slint::format!("{:.2}%", treasury.get_bid_to_cover_ratio()).into());
        //     col.push(
        //         slint::format!(
        //             "{:.2}%",
        //             treasury.get_percentage_debt_purchased_by_dealers()
        //         )
        //         .into(),
        //     );
        //     col.push(
        //         slint::format!(
        //             "{:.2}%",
        //             treasury.get_percentage_debt_purchased_by_directs()
        //         )
        //         .into(),
        //     );
        //     col.push(
        //         slint::format!(
        //             "{:.2}%",
        //             treasury.get_percentage_debt_purchased_by_indirects()
        //         )
        //         .into(),
        //     );
        //     col.push(slint::format!("{:.3}%", treasury.get_high_yield()).into());
        //     col.push(slint::format!("{:.3}%", treasury.get_interest_rate()).into());
        //     row_data.push(col.into());
        // }

        // row_data
    }
}