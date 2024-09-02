use crate::error::Result;
use colored::*;
use spinners::{Spinner, Spinners};
use subxt::utils::AccountId32;
use crate::commands::assethub;
use std::time::Duration;
use tokio::time::sleep;
use crate::client::get_client;
use viuer::{print_from_file, Config};
use reqwest::get;
use std::path::Path;
use serde_json::Value;


// Function to convert Plancks to PAS
fn plancks_to_pas(amount: u128) -> f64 {
    (amount as f64) / 10f64.powi(10)
}

fn format_account_ss58(account_id: &AccountId32) -> String {
    account_id.to_string() // Modify if you need SS58 formatting
}

fn format_account_option(account_id: &Option<AccountId32>) -> String {
    match account_id {
        Some(id) => format_account_ss58(id),
        None => "No account".to_string(),
    }
}

fn format_item_deposit(deposit: &assethub::runtime_types::pallet_nfts::types::ItemDeposit<u128, AccountId32>) -> String {
    let account = format_account_ss58(&deposit.account);
    let pas_amount = plancks_to_pas(deposit.amount); // Accessing DepositBalance (u128)
    let formatted_amount = format!("{:.4} PAS", pas_amount); // Format to 4 decimal places
    format!("{}: {} | {}: {}", "Account".cyan().bold(), account.bright_white(), "Amount".cyan().bold(), formatted_amount.bright_white())
}

fn format_metadata_deposit(deposit: &assethub::runtime_types::pallet_nfts::types::ItemMetadataDeposit<u128, AccountId32>) -> String {
    let account = format_account_option(&deposit.account);
    let pas_amount = plancks_to_pas(deposit.amount); // Accessing DepositBalance (u128)
    let formatted_amount = format!("{:.4} PAS", pas_amount); // Format to 4 decimal places
    format!("{}: {} | {}: {}", "Account".cyan().bold(), account.bright_white(), "Amount".cyan().bold(), formatted_amount.bright_white())
}

// Convert a BoundedVec of bytes to a human-readable string
fn parse_metadata_data(data: &assethub::runtime_types::bounded_collections::bounded_vec::BoundedVec<u8>) -> String {
    match String::from_utf8(data.0.to_vec()) {
        Ok(parsed_string) => parsed_string,
        Err(_) => format!("{:?}", data),
    }
}

// Check if the data is an IPFS link
fn is_ipfs_link(data: &str) -> bool {
    data.starts_with("ipfs://")
}

// Convert an IPFS link to an HTTP URL
fn ipfs_to_http_url(ipfs_link: &str) -> String {
    ipfs_link.replace("ipfs://", "https://ipfs.io/ipfs/")
}

// Download content from a URL
async fn download_content(url: &str) -> Result<String> {
    let response = get(url).await?;
    let content = response.text().await?;
    Ok(content)
}

// Download an image from a URL and return the file path
async fn download_image(url: &str) -> Result<String> {
    let response = get(url).await?;
    let image_path = "/tmp/ipfs_image.png";
    let mut file = std::fs::File::create(image_path)?;
    let content = response.bytes().await?;
    std::io::copy(&mut content.as_ref(), &mut file)?;
    Ok(image_path.to_string())
}

// Handle and display JSON data from IPFS
async fn handle_ipfs_json(json_content: String) -> Result<()> {
    let parsed_json: Value = serde_json::from_str(&json_content)?;

    println!("\n{}\n", "📄 Parsed JSON Metadata".blue().bold());
    println!("{}", serde_json::to_string_pretty(&parsed_json)?.cyan());

    Ok(())
}

// Function to handle displaying the image from IPFS
async fn handle_ipfs_image(image_link: &str) -> Result<()> {
    let image_url = ipfs_to_http_url(image_link);
    println!("📦 Image URL: {}", image_url);

    match download_image(&image_url).await {
        Ok(image_path) => {
            let config = Config::default();
            print_from_file(Path::new(&image_path), &config).expect("Image display failed");
            std::fs::remove_file(image_path)?;
        }
        Err(err) => println!("{}", format!("❌ Failed to download image: {}", err).red().bold()),
    }

    Ok(())
}

pub async fn show_nft(collection_id: u32, nft_id: u32, show_json: bool, show_image: bool) -> Result<()> {
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
        //println!("{}: {:?}", "Approvals".cyan().bold(), info.approvals);
        println!("{}: {}", "Deposit".cyan().bold(), format_item_deposit(&info.deposit));
    } else {
        println!("{}", "❌ NFT not found.".red().bold());
    }

    // Display the metadata information
    if let Some(metadata) = metadata_info {
        let metadata_str = parse_metadata_data(&metadata.data);
        println!("\n{}\n", "📝 NFT Metadata".blue().bold());
        println!("{}: {}", "Data".cyan().bold(), metadata_str);
        println!("{}: {}", "Deposit".cyan().bold(), format_metadata_deposit(&metadata.deposit));

        if is_ipfs_link(&metadata_str) {
            let metadata_url = ipfs_to_http_url(&metadata_str);
            println!("📦 Metadata URL: {}", metadata_url);

            if show_json {
                match download_content(&metadata_url).await {
                    Ok(content) => {
                        if let Ok(_) = serde_json::from_str::<Value>(&content) {
                            handle_ipfs_json(content).await?;
                        } else {
                            println!("Raw Content: {}", content.cyan());
                        }
                    }
                    Err(err) => println!("{}", format!("❌ Failed to download metadata: {}", err).red().bold()),
                }
            }

            if show_image {
                match download_content(&metadata_url).await {
                    Ok(content) => {
                        if let Ok(parsed_json) = serde_json::from_str::<Value>(&content) {
                            if let Some(image_link) = parsed_json.get("image").and_then(Value::as_str) {
                                handle_ipfs_image(image_link).await?;
                            }
                        }
                    }
                    Err(err) => println!("{}", format!("❌ Failed to download image: {}", err).red().bold()),
                }
            }
        }
    } else {
        println!("{}", "❌ Metadata not found.".red().bold());
    }

    Ok(())
}

