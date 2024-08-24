use crate::error::Result;
use subxt::{OnlineClient, PolkadotConfig, utils::AccountId32};
use crate::commands::statemint;
use std::str::FromStr;

pub async fn balance(address: Option<String>) -> Result<()> {
    let api = OnlineClient::<PolkadotConfig>::from_url("wss://asset-hub-paseo-rpc.dwellir.com").await?;
    println!("Connection with parachain established.");

    // Determine the account to use based on the presence of an address argument
    let account: AccountId32 = if let Some(addr) = address {
        AccountId32::from_str(&addr)?
    } else {
        let account_signer = crate::config::load_account_from_config()?;
        account_signer.public_key().into()
    };

    let storage_query = statemint::storage().system().account(account.clone());

    let result: Option<
        statemint::runtime_types::frame_system::AccountInfo<
            u32,
            statemint::runtime_types::pallet_balances::types::AccountData<u128>,
        >
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;

    if let Some(account_info) = result {
        let free_balance = account_info.data.free;
        let unit = 10u128.pow(10); // 1 PAS = 10^10 Plancks
        let human_readable_balance = free_balance as f64 / unit as f64;

        println!("Free balance of account {}: {:.10} PAS", account, human_readable_balance);
    } else {
        println!("No account data found for account {}", account);
    }

    Ok(())
}

