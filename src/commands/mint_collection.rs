use crate::error::Result;
use colored::*;
use spinners::{Spinner, Spinners};
use subxt::utils::{AccountId32, MultiAddress};
use crate::commands::assethub;
use crate::client::get_client;
use pallet_nfts::{ItemSettings, CollectionSettings};
use std::marker::PhantomData;
use reqwest::Client;
use serde_json::Value;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::ffi::OsStr;

// Function to convert CollectionSettings to the required BitFlags1 type
fn to_collection_bitflags(
    settings: CollectionSettings,
) -> assethub::runtime_types::pallet_nfts::types::BitFlags1<
    assethub::runtime_types::pallet_nfts::types::CollectionSetting,
> {
    let bits = settings.0.bits();
    assethub::runtime_types::pallet_nfts::types::BitFlags1(bits, PhantomData)
}

// Function to pin data to IPFS
async fn pin_to_ipfs(data: &[u8]) -> Result<String> {
    let client = Client::new();
    let response = client
        .post("https://ipfs.io/ipfs")
        .body(data.to_vec())
        .send()
        .await
        .map_err(|e| format!("Failed to pin to IPFS: {}", e))?;

    let ipfs_hash = response.text().await.map_err(|e| format!("Failed to read IPFS response: {}", e))?;
    Ok(format!("ipfs://{}", ipfs_hash))
}

// Function to load JSON from a file
fn load_json_from_file(path: &Path) -> Result<Value> {
    let file = File::open(path).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    serde_json::from_reader(file).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

// Function to match JSON files with images based on the "image" field or filename
fn link_json_with_images(json_folder: &Path, image_folder: &Path) -> Result<HashMap<String, (Value, PathBuf)>> {
    let mut linked_data = HashMap::new();

    for entry in json_folder.read_dir().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)? {
        let entry = entry.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        if entry.path().is_file() {
            let json_data = load_json_from_file(&entry.path())?;
            let image_name = json_data.get("image")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| {
                    // If "image" field is not present, match by filename
                    let binding = entry.path();
                    let json_filename = binding.file_stem().unwrap_or_else(|| OsStr::new("")).to_string_lossy();
                    format!("{}.jpg", json_filename)
                });

            let image_path = image_folder.join(&image_name);
            if !image_path.exists() {
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, format!("Image file not found: {:?}", image_path))));
            }

            linked_data.insert(image_name, (json_data, image_path));
        }
    }

    Ok(linked_data)
}

#[cfg(feature = "nft")]
pub async fn mint_collection(json_folder: Option<&str>, image_folder: Option<&str>) -> Result<()> {
    let api = get_client().await?;
    println!("{}", "🚀 Connection with parachain established.".green().bold());

    let account_signer = crate::config::load_account_from_config()?;
    let admin: MultiAddress<AccountId32, ()> = account_signer.public_key().into();

    // Handle optional JSON and image folders
    if let (Some(json_folder_str), Some(image_folder_str)) = (json_folder, image_folder) {
        let json_folder_path = Path::new(json_folder_str);
        let image_folder_path = Path::new(image_folder_str);

        // Link JSON files with their corresponding images
        let linked_data = link_json_with_images(json_folder_path, image_folder_path)?;

        let mut pinned_data = Vec::new();

        for (image_name, (mut json_data, image_path)) in linked_data {
            // Pin image to IPFS
            let image_bytes = std::fs::read(&image_path).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            let ipfs_image_link = pin_to_ipfs(&image_bytes).await?;
            println!("Pinned image to IPFS: {}", ipfs_image_link);

            // Update JSON with IPFS image link if not already present
            json_data["image"] = Value::String(ipfs_image_link.clone());

            // Pin updated JSON to IPFS
            let json_bytes = serde_json::to_vec(&json_data).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            let ipfs_json_link = pin_to_ipfs(&json_bytes).await?;
            println!("Pinned JSON to IPFS: {}", ipfs_json_link);

            pinned_data.push((image_name, ipfs_json_link));
        }
    }

    let config = assethub::runtime_types::pallet_nfts::types::CollectionConfig {
        settings: to_collection_bitflags(CollectionSettings::all_enabled()),
        max_supply: None,
        mint_settings: assethub::runtime_types::pallet_nfts::types::MintSettings {
            mint_type: assethub::runtime_types::pallet_nfts::types::MintType::Issuer,
            price: None,
            start_block: None,
            end_block: None,
            default_item_settings: crate::commands::mint_nft::to_item_bitflags(
                ItemSettings::all_enabled(),
            ),
            __ignore: Default::default(),
        },
        __ignore: Default::default(),
    };

    let payload = assethub::tx().nfts().create(admin, config);

    let mut sp = Spinner::new(Spinners::Dots12, "⏳ Preparing transaction...".yellow().bold().to_string());
    sp.stop_and_persist("🚀", "Sending transaction to the network...".yellow().bold().to_string());

    let extrinsic_result = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &account_signer)
        .await?;

    let mut sp = Spinner::new(Spinners::Dots12, "⏳ Finalizing transaction...".yellow().bold().to_string());

    let extrinsic_result = extrinsic_result.wait_for_finalized_success().await?;

    sp.stop_and_persist("✅", "Collection creation finalized!".green().bold().to_string());

    let extrinsic_hash = extrinsic_result.extrinsic_hash();

    let created_event = extrinsic_result.find_first::<assethub::nfts::events::Created>()?;

    if let Some(event) = created_event {
        if let assethub::nfts::events::Created { collection, .. } = event {
            println!("\n{}\n", "🎉 Collection Created Successfully!".blue().bold());
            println!(
                "{}: {}",
                "📦 Collection ID".cyan().bold(),
                collection.to_string().bright_white()
            );
        }
    } else {
        println!("{}", "❌ Collection ID not found in events.".red().bold());
    }

    println!(
        "{}: {}",
        "🔗 Extrinsic Hash".cyan().bold(),
        format!("{:?}", extrinsic_hash).bright_white()
    );

    Ok(())
}
