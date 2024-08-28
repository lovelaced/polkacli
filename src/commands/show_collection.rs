use crate::error::Result;
use colored::*;
use spinners::{Spinner, Spinners};
use subxt::{
    utils::AccountId32,
};
use crate::commands::assethub;
use std::time::Duration;
use tokio::time::sleep;
use crate::client::get_client;

// Function to convert AccountId32 to SS58 format
fn format_account_ss58(account_id: &AccountId32) -> String {
    account_id.to_string() // Convert to SS58 if required by your application
}

#[cfg(feature = "nft")]
pub async fn show_collection(collection_id: u32) -> Result<()> {
    let api = get_client().await?;
    println!("{}", "🔍 Fetching Collection information...".green().bold());

    // Start the spinner
    let mut sp = Spinner::new(Spinners::Dots12, "⏳ Retrieving Collection data...".yellow().bold().to_string());
    sleep(Duration::from_secs(1)).await;

    // Fetch the collection details
    let collection_query = assethub::storage().nfts().collection(collection_id);
    let collection_info = match api.storage().at_latest().await?.fetch(&collection_query).await {
        Ok(Some(info)) => Some(info),
        Ok(None) => {
            sp.stop_and_persist("❌", "Collection not found.".red().bold().to_string());
            return Ok(());
        }
        Err(e) => {
            sp.stop_and_persist("❌", format!("Error fetching collection data: {}", e).red().bold().to_string());
            return Err(Box::new(e));
        }
    };

    // Fetch the collection metadata
    let metadata_query = assethub::storage().nfts().collection_metadata_of(collection_id);
    let metadata_info = match api.storage().at_latest().await?.fetch(&metadata_query).await {
        Ok(Some(metadata)) => Some(metadata),
        Ok(None) => {
            println!("{}", "❌ Metadata not found.".red().bold());
            None
        }
        Err(e) => {
            sp.stop_and_persist("❌", format!("Error fetching metadata: {}", e).red().bold().to_string());
            return Err(Box::new(e));
        }
    };

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
    }

    // Display the metadata information
    if let Some(metadata) = metadata_info {
        println!("\n{}\n", "📝 Collection Metadata".blue().bold());
        println!("{}: {:?}", "Data".cyan().bold(), metadata.data);
        println!("{}: {:?}", "Deposit".cyan().bold(), metadata.deposit);
    }

    Ok(())
}

