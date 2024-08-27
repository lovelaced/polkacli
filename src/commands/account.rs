use crate::error::Result;
use subxt::{OnlineClient, PolkadotConfig, utils::AccountId32};
use std::str::FromStr;
use crate::commands::assethub;


pub async fn account_info(public_key: String) -> Result<()> {
    let api = OnlineClient::<PolkadotConfig>::from_url("wss://asset-hub-paseo-rpc.dwellir.com").await?;
    println!("Connection with parachain established.");

    let account: AccountId32 = AccountId32::from_str(&public_key)?;

    let storage_query = assethub::storage().system().account(account.clone());

    let result: Option<
        assethub::runtime_types::frame_system::AccountInfo<
            u32,
            assethub::runtime_types::pallet_balances::types::AccountData<u128>,
        >
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;

    if let Some(account_info) = result {
        println!("Free balance: {}", account_info.data.free);
        println!("Nonce: {}", account_info.nonce);
        println!("Consumers: {}", account_info.consumers);
        println!("Providers: {}", account_info.providers);
        println!("Sufficients: {}", account_info.sufficients);
    } else {
        println!("No account data found for account {}", public_key);
    }

    Ok(())
}

