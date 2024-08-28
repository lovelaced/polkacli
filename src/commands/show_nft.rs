use crate::error::Result;
use colored::*;
use spinners::{Spinner, Spinners};
use subxt::utils::AccountId32;
use crate::commands::assethub;
use std::time::Duration;
use tokio::time::sleep;
use crate::client::get_client;

// Function to convert AccountId32 to SS58 format
fn format_account_ss58(account_id: &AccountId32) -> String {
    account_id.to_string() // If you want to convert it to SS58, use the appropriate method here
}

// Function to convert a BoundedVec of bytes to a human-readable string
fn parse_metadata_data(data: &assethub::runtime_types::bounded_collections::bounded_vec::BoundedVec<u8>) -> String {
    match String::from_utf8(data.0.to_vec()) {
        Ok(parsed_string) => parsed_string,
        Err(_) => format!("{:?}", data), // Fall back to showing the raw bytes if parsing fails
    }
}

#[cfg(feature = "nft")]
pub async fn show_nft(collection_id: u32, nft_id: u32) -> Result<()> {
    let api = get_client().await?;
    println!("{}", "🔍 Fetching NFT information...".green().bold());

    // Start the spinner
    let mut sp = Spinner::new(Spinners::Dots12, "⏳ Retrieving NFT data...".yellow().bold().to_string());
    sleep(Duration::from_secs(1)).await;

    // Use the API to query the NFT data based on `collection_id` and `nft_id`
    let storage_query = assethub::storage().nfts().item(collection_id, nft_id);
    let nft_info = api.storage().at_latest().await?.fetch(&storage_query).await?;

    // Fetch the metadata associated with the NFT
    let metadata_query = assethub::storage().nfts().item_metadata_of(collection_id, nft_id);
    let metadata_info = api.storage().at_latest().await?.fetch(&metadata_query).await?;

    // Stop the spinner with a final message
    sp.stop_and_persist("✅", "NFT data retrieved!".green().bold().to_string());

    // Display the NFT information
    if let Some(info) = nft_info {
        println!("\n{}\n", "🎨 NFT Information".blue().bold());
        println!("{}: {}", "Collection ID".cyan().bold(), collection_id.to_string().bright_white());
        println!("{}: {}", "NFT ID".cyan().bold(), nft_id.to_string().bright_white());
        println!("{}: {}", "Owner".cyan().bold(), format_account_ss58(&info.owner));
        println!("{}: {:?}", "Approvals".cyan().bold(), info.approvals);
        println!("{}: {:?}", "Deposit".cyan().bold(), info.deposit);
    } else {
        println!("{}", "❌ NFT not found.".red().bold());
    }

    // Display the metadata information
    if let Some(metadata) = metadata_info {
        println!("\n{}\n", "📝 NFT Metadata".blue().bold());
        println!("{}: {}", "Data".cyan().bold(), parse_metadata_data(&metadata.data));
        println!("{}: {:?}", "Deposit".cyan().bold(), metadata.deposit);
    } else {
        println!("{}", "❌ Metadata not found.".red().bold());
    }

    Ok(())
}

