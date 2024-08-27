use crate::error::Result;
use colored::*;
use spinners::{Spinner, Spinners};
use subxt::{
    OnlineClient, PolkadotConfig,
    utils::{AccountId32},
};
use crate::commands::assethub;
use std::time::Duration;
use tokio::time::sleep;

// Function to convert AccountId32 to SS58 format
fn format_account_ss58(account_id: &AccountId32) -> String {
    account_id.to_string() // Convert to SS58 if required by your application
}

#[cfg(feature = "nft")]
pub async fn show_collection(collection_id: u32) -> Result<()> {
    let api = OnlineClient::<PolkadotConfig>::from_url("wss://asset-hub-paseo-rpc.dwellir.com").await?;
    println!("{}", "🔍 Fetching Collection information...".green().bold());

    // Start the spinner
    let mut sp = Spinner::new(Spinners::Dots12, "⏳ Retrieving Collection data...".yellow().bold().to_string());
    sleep(Duration::from_secs(1)).await;

    // Fetch the collection details
    let collection_query = assethub::storage().nfts().collection(collection_id);
    let collection_info = api.storage().at_latest().await?.fetch(&collection_query).await?;

    // Fetch the collection metadata
    let metadata_query = assethub::storage().nfts().collection_metadata_of(collection_id);
    let metadata_info = api.storage().at_latest().await?.fetch(&metadata_query).await?;

    // Stop the spinner with a final message
    sp.stop_and_persist("✅", "Collection data retrieved!".green().bold().to_string());

    // Display the Collection information
    if let Some(info) = collection_info {
        println!("\n{}\n", "🏛️ Collection Information".blue().bold());
        println!("{}: {}", "Collection ID".cyan().bold(), collection_id.to_string().bright_white());
        println!("{}: {}", "Owner".cyan().bold(), format_account_ss58(&info.owner));
        println!("{}: {}", "Owner Deposit".cyan().bold(), info.owner_deposit.to_string().bright_white());
        println!("{}: {}", "Items Count".cyan().bold(), info.items.to_string().bright_white());
        println!("{}: {}", "Item Metadata Count".cyan().bold(), info.item_metadatas.to_string().bright_white());
        println!("{}: {}", "Item Config Count".cyan().bold(), info.item_configs.to_string().bright_white());
        println!("{}: {:?}", "Attributes".cyan().bold(), info.attributes);
    } else {
        println!("{}", "❌ Collection not found.".red().bold());
    }

    // Display the metadata information
    if let Some(metadata) = metadata_info {
        println!("\n{}\n", "📝 Collection Metadata".blue().bold());
        println!("{}: {:?}", "Data".cyan().bold(), metadata.data);
        println!("{}: {:?}", "Deposit".cyan().bold(), metadata.deposit);
    } else {
        println!("{}", "❌ Metadata not found.".red().bold());
    }

    Ok(())
}

