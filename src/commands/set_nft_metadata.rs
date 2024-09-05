use crate::error::Result;
use crate::utils::{ipfs_utils, json_utils};
use colored::*;
use spinners::{Spinner, Spinners};
use subxt::utils::AccountId32;
use crate::commands::assethub;
use crate::client::get_client;
use std::path::Path;
use serde_json::Value;
use std::path::PathBuf;

pub async fn set_nft_metadata(collection_id: u32, nft_id: u32, json_path: Option<&str>, image_path: Option<&str>) -> Result<()> {
    let api = get_client().await?;
    println!("{}", "🚀 Connection with parachain established.".green().bold());

    let account_signer = crate::config::load_account_from_config()?;

    let mut json_data: Option<Value> = None;
    let mut image_link = String::new();

    // Step 1: Handle the JSON and image
    if let Some(json_path_str) = json_path {
        let json_path = Path::new(json_path_str);
        json_data = Some(json_utils::load_json_from_file(json_path)?);

        let mut sp = Spinner::new(Spinners::Dots12, "🖼️ Processing image...".yellow().bold().to_string());

        // Check if the JSON already contains an image link
        if let Some(image) = json_data.as_ref().unwrap().get("image").and_then(Value::as_str) {
            if !image.is_empty() {
                image_link = image.to_string();
                sp.stop_and_persist("🔗", "Using existing image link in JSON.".green().bold().to_string());
            }
        }

        // If no image link, pin the provided or inferred image to IPFS
        if image_link.is_empty() {
            let image_path_buf = if let Some(image_path_str) = image_path {
                PathBuf::from(image_path_str)
            } else {
                json_utils::find_image_for_json(json_path)?
            };

            let image_bytes = std::fs::read(&image_path_buf)?;
            image_link = ipfs_utils::pin_to_ipfs(&image_bytes).await?;
            json_data.as_mut().unwrap()["image"] = Value::String(image_link.clone());

            sp.stop_and_persist("✅", "Image pinned to IPFS and link added to JSON.".green().bold().to_string());
        }
    } else if image_path.is_some() {
        return Err("Error: --json must be provided when using --image.".into());
    }

    // Step 2: Pin JSON metadata to IPFS
    let ipfs_json_link = if let Some(json_data) = json_data {
        let mut sp = Spinner::new(Spinners::Dots12, "📦 Pinning JSON metadata to IPFS...".yellow().bold().to_string());
        let json_bytes = serde_json::to_vec(&json_data)?;
        let link = ipfs_utils::pin_to_ipfs(&json_bytes).await?;
        sp.stop_and_persist("✅", "JSON metadata pinned to IPFS.".green().bold().to_string());
        Some(link)
    } else {
        None
    };

    if let Some(ref ipfs_json_link) = ipfs_json_link {
        println!("📄 Pinned JSON to IPFS: {}", ipfs_json_link);

        // Step 3: Set the metadata on-chain for the NFT
        let mut sp = Spinner::new(Spinners::Dots12, "📜 Setting NFT metadata on-chain...".yellow().bold().to_string());
        let metadata_link: assethub::runtime_types::bounded_collections::bounded_vec::BoundedVec<u8> =
            assethub::runtime_types::bounded_collections::bounded_vec::BoundedVec(ipfs_json_link.clone().into_bytes());

        let nft_metadata_tx = assethub::tx().nfts().set_metadata(collection_id, nft_id, metadata_link);

        let extrinsic_result = api
            .tx()
            .sign_and_submit_then_watch_default(&nft_metadata_tx, &account_signer)
            .await?;

        let extrinsic_result = extrinsic_result.wait_for_finalized_success().await?;
        sp.stop_and_persist("✅", "Metadata set successfully.".green().bold().to_string());

        let extrinsic_hash = extrinsic_result.extrinsic_hash();
        println!(
            "{}: {}",
            "🔗 Extrinsic Hash".cyan().bold(),
            format!("{:?}", extrinsic_hash).bright_white()
        );
    } else {
        println!("❌ No valid JSON file found for setting metadata.");
    }

    Ok(())
}

