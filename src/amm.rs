use crate::args::AmmCommand;
use crate::{contracts, address_list};
use ethers::prelude::*;
use eyre::Result;

/// Processing theh AMM Command
#[tokio::main]
pub async fn process(args: AmmCommand) -> Result<()> {
    let pools = address_list::get_pools().await;
    let pools_iter = pools.iter();

    if args.search_parameter == None && args.short == Some(false) {
        println!("");
        for pool in pools_iter.clone() {
            let contract = contracts::get_base_contract(pool.base_address.parse::<Address>()?);
            let quote_contract = contracts::get_quote_contract(pool.quote_address.parse::<Address>()?);
            // let pool_contract = contracts::get_pool_contract(pool.address.parse::<Address>()?);

            let index_price = contract
                .get_index_price(U256::zero())
                .call()
                .await?;
            let format_index_price = ethers::utils::format_units(index_price, 18)?;

            let price_feed = contract
               .get_price_feed()
               .call()
               .await?;

            let base_asset_reserve = contract
               .balance_of(pool.address.parse::<Address>()?)
               .call()
               .await?;

            let quote_asset_reserve = quote_contract
               .balance_of(pool.address.parse::<Address>()?)
               .call()
               .await?;

            // let market_price = pool_contract
            //     .slot_0()
            //     .call()
            //     .await?;

            // let price = ethers::utils::format_units(market_price.0.pow(U256::from(2)) / U256::from(2).pow(U256::from(192)), "ether")?.parse::<f64>()?;

            println!("========================");
            println!("=====  {}/{}  =====", pool.base_symbol, pool.quote_symbol);
            println!("========================");
            println!("- Pool Address: {}", pool.address);
            println!("- Index Price: {}", format_index_price);
            // println!("- Market Price: {}", price);
            // println!("- OpenInterestNotionalCap: {}", open_interest_notional_cap);
            // println!("- OpenInterestNotional: {}", open_interest_notional);
            // println!("- MaxHoldingBaseAsset: {}", max_holding_base_asset);
            println!("- {} Reserves: {}", pool.base_symbol, ethers::utils::format_units(base_asset_reserve, "ether")?);
            println!("- {} Reserves: {}", pool.quote_symbol, ethers::utils::format_units(quote_asset_reserve, "ether")?);
            println!("- Price Feed: {:?}", price_feed);
        }
        println!("");
    }

    match args.search_parameter {
        Some(value) => {
            println!("");
        for pool in pools_iter.clone() {
            if pool.address != value && pool.base_address != value && pool.base_symbol != value {continue;}
            let contract = contracts::get_base_contract(pool.base_address.parse::<Address>()?);
            let quote_contract = contracts::get_base_contract(pool.quote_address.parse::<Address>()?);
            // let pool_contract = contracts::get_pool_contract(pool.address.parse::<Address>()?);

            let index_price = contract
                .get_index_price(U256::zero())
                .call()
                .await?;
            let format_index_price = ethers::utils::format_units(index_price, 18)?;

            let price_feed = contract
               .get_price_feed()
               .call()
               .await?;

            let base_asset_reserve = contract
               .balance_of(pool.address.parse::<Address>()?)
               .call()
               .await?;

            let quote_asset_reserve = quote_contract
               .balance_of(pool.address.parse::<Address>()?)
               .call()
               .await?;

        //     let market_price = pool_contract
        //        .slot_0()
        //        .call()
        //        .await?;

        //    let price = ethers::utils::format_units(market_price.0.pow(U256::from(2)) / U256::from(2).pow(U256::from(192)), 6)?.parse::<f64>()?;

            println!("========================");
            println!("=====  {}/{}  =====", pool.base_symbol, pool.quote_symbol);
            println!("========================");
            println!("- Pool Address: {}", pool.address);
            println!("- Index Price: {}", format_index_price);
            // println!("- Market Price: {}", price);
            // println!("- OpenInterestNotionalCap: {}", open_interest_notional_cap);
            // println!("- OpenInterestNotional: {}", open_interest_notional);
            // println!("- MaxHoldingBaseAsset: {}", max_holding_base_asset);
            println!("- {} Reserves: {}", pool.base_symbol, ethers::utils::format_units(base_asset_reserve, "ether")?);
            println!("- {} Reserves: {}", pool.quote_symbol, ethers::utils::format_units(quote_asset_reserve, "ether")?);
            println!("- Price Feed: {:?}", price_feed);
            break;
        }
        println!("");
    },
        None => {},
    }

    match args.short {
        Some(short) => if short {
            println!("");
            for pool in pools_iter {
                println!("- {}/{}: {}", pool.base_symbol, pool.quote_symbol, pool.address);
            }
            println!("");
        },
        None => {},
    }
    Ok(())
}