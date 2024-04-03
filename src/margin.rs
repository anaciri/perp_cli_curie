use ethers::prelude::*;
use crate::{contracts, utils};
use eyre::Result;

pub async fn process_margin_command(trader_address: Option<String>) -> Result<()> {
    let clearing_house = contracts::get_clearing_house().await?;
    let vault_contract = contracts::get_vault().await?;
    let mut trader = utils::get_wallet()?.address();

    if let Some(trader_add) = trader_address {
        trader = trader_add.parse::<Address>()?;
    }

    let total_account_value: I256 = clearing_house
        .get_account_value(trader)
        .call()
        .await?;

    let free_collateral_value: U256 = vault_contract
        .get_free_collateral(trader)
        .call()
        .await?;

    let margin_ratio = if total_account_value != I256::zero() && free_collateral_value != U256::zero() {
        let total_account_value_f64 = ethers::utils::format_units(total_account_value, "ether")?.parse::<f64>()?;
        let free_collateral_value_f64 = ethers::utils::format_units(free_collateral_value, 6)?.parse::<f64>()?;
        free_collateral_value_f64 / total_account_value_f64
    } else {
        0.0
    };

    println!("Trader Address: {:?}", trader);
    println!("Margin Ratio: {:.2}%", margin_ratio * 100.0);

    Ok(())
}
