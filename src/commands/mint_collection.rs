use crate::error::Result;
use crate::utils::{json_utils};
use colored::*;
use spinners::{Spinner, Spinners};
use subxt::utils::{AccountId32, MultiAddress};
use crate::commands::assethub;
use crate::commands::mint_nft;
use crate::client::get_client;
use pallet_nfts::{ItemSettings, CollectionSettings};
use std::marker::PhantomData;
use std::path::{Path};

// Function to convert CollectionSettings to the required BitFlags1 type
fn to_collection_bitflags(
    settings: CollectionSettings,
) -> assethub::runtime_types::pallet_nfts::types::BitFlags1<
    assethub::runtime_types::pallet_nfts::types::CollectionSetting,
> {
    let bits = settings.0.bits();
    assethub::runtime_types::pallet_nfts::types::BitFlags1(bits, PhantomData)
}

pub async fn mint_collection(json_folder: Option<&str>, image_folder: Option<&str>) -> Result<()> {
    let api = get_client().await?;
    println!("{}", "🚀 Connection with parachain established.".green().bold());

    let account_signer = crate::config::load_account_from_config()?;
    let admin: MultiAddress<AccountId32, ()> = account_signer.public_key().into();

    let config = assethub::runtime_types::pallet_nfts::types::CollectionConfig {
        settings: to_collection_bitflags(CollectionSettings::all_enabled()),
        max_supply: None,
        mint_settings: assethub::runtime_types::pallet_nfts::types::MintSettings {
            mint_type: assethub::runtime_types::pallet_nfts::types::MintType::Issuer,
            price: None,
            start_block: None,
            end_block: None,
            default_item_settings: mint_nft::to_item_bitflags(ItemSettings::all_enabled()),
            __ignore: Default::default(),
        },
        __ignore: Default::default(),
    };

    let payload = assethub::tx().nfts().create(admin.clone(), config);

    let mut sp = Spinner::new(Spinners::Dots12, "⏳ Preparing transaction...".yellow().bold().to_string());
    sp.stop_and_persist("🚀", "Sending transaction to the network...".yellow().bold().to_string());

    let extrinsic_result = api.tx().sign_and_submit_then_watch_default(&payload, &account_signer).await?;

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

            // Step 2: Mint NFTs into the collection if JSON and image paths were provided
            if let Some(json_folder_str) = json_folder {
                let json_folder_path = Path::new(json_folder_str);
                let image_folder_path = image_folder.map(Path::new);

                let entries: Vec<_> = json_folder_path
                    .read_dir()?
                    .collect::<std::result::Result<Vec<_>, _>>()?;

                for (nft_id, entry) in entries.iter().enumerate() {
                    let json_path = entry.path();

                    let image_path = if let Some(image_folder_path) = image_folder_path {
                        json_utils::find_image_for_json(&json_path).ok().or_else(|| {
                            let file_stem = json_path.file_stem()?.to_str()?;
                            let image_path = image_folder_path.join(format!("{}.jpg", file_stem));
                            if image_path.exists() {
                                Some(image_path)
                            } else {
                                let image_path = image_folder_path.join(format!("{}.jpeg", file_stem));
                                if image_path.exists() {
                                    Some(image_path)
                                } else {
                                    let image_path = image_folder_path.join(format!("{}.png", file_stem));
                                    image_path.exists().then_some(image_path)
                                }
                            }
                        })
                    } else {
                        None
                    };

                    if image_folder_path.is_some() && image_path.is_none() {
                        return Err("Error: No image found or provided for the NFT.".into());
                    }

                    let image_path_str = match image_path {
                        Some(p) => p.to_str()
                            .map(|s| s.to_owned())
                            .ok_or_else(|| "Failed to convert image path to string".to_string())
                            .map(Some),
                        None => Ok(None),
                    }?;

                    let nft_id = nft_id as u32;
                    mint_nft::mint_nft(collection, nft_id, Some(json_path.to_str().unwrap()), image_path_str.as_deref()).await?;
                }
            }
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

