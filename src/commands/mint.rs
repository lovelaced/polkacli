use crate::error::Result;
use colored::*;
use spinners::{Spinner, Spinners};
use subxt::{
    OnlineClient, PolkadotConfig,
    utils::{AccountId32, MultiAddress},
};
use crate::commands::statemint;
use pallet_nfts::{CollectionSettings, ItemSettings};
use std::marker::PhantomData;
use std::time::Duration;
use tokio::time::sleep;

// Function to convert CollectionSettings to the required BitFlags1 type
fn to_collection_bitflags(
    settings: CollectionSettings,
) -> statemint::runtime_types::pallet_nfts::types::BitFlags1<
    statemint::runtime_types::pallet_nfts::types::CollectionSetting,
> {
    let bits = settings.0.bits();
    statemint::runtime_types::pallet_nfts::types::BitFlags1(bits, PhantomData)
}

// Function to convert ItemSettings to the required BitFlags2 type
fn to_item_bitflags(
    settings: ItemSettings,
) -> statemint::runtime_types::pallet_nfts::types::BitFlags2<
    statemint::runtime_types::pallet_nfts::types::ItemSetting,
> {
    let bits = settings.0.bits();
    statemint::runtime_types::pallet_nfts::types::BitFlags2(bits, PhantomData)
}

#[cfg(feature = "nft")]
pub async fn mint_collection() -> Result<()> {
    let api = OnlineClient::<PolkadotConfig>::from_url("wss://asset-hub-paseo-rpc.dwellir.com").await?;
    println!("{}", "🚀 Connection with parachain established.".green().bold());

    let account_signer = crate::config::load_account_from_config()?;
    let admin: MultiAddress<AccountId32, ()> = account_signer.public_key().into();

    let config = statemint::runtime_types::pallet_nfts::types::CollectionConfig {
        settings: to_collection_bitflags(CollectionSettings::all_enabled()),
        max_supply: None,
        mint_settings: statemint::runtime_types::pallet_nfts::types::MintSettings {
            mint_type: statemint::runtime_types::pallet_nfts::types::MintType::Issuer,
            price: None,
            start_block: None,
            end_block: None,
            default_item_settings: to_item_bitflags(ItemSettings::all_enabled()),
            __ignore: Default::default(),
        },
        __ignore: Default::default(),
    };

    let payload = statemint::tx().nfts().create(admin, config);

    // Start the spinner for preparation
    let mut sp = Spinner::new(Spinners::Dots12, "⏳ Preparing transaction...".yellow().bold().to_string());
    sleep(Duration::from_secs(1)).await;
    sp.stop_and_persist("🚀", "Sending transaction to the network...".yellow().bold().to_string());

    let extrinsic_result = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &account_signer)
        .await?;

    // Update the spinner for finalization with periodic status updates
    let mut sp = Spinner::new(Spinners::Dots12, "⏳ Finalizing transaction...".yellow().bold().to_string());

    for i in 1..=5 {
        sleep(Duration::from_secs(2)).await;
        sp.stop_and_persist(
            "⏳",
            format!("Finalizing transaction... ({}s)", i * 2)
                .yellow()
                .bold()
                .to_string(),
        );
        sp = Spinner::new(Spinners::Dots12, "".into());
    }

    let extrinsic_result = extrinsic_result.wait_for_finalized_success().await?;

    // Stop the spinner with a final message
    sp.stop_and_persist("✅", "Collection creation finalized!".green().bold().to_string());

    let extrinsic_hash = extrinsic_result.extrinsic_hash();

    // Find the `Created` event
    let created_event = extrinsic_result.find_first::<statemint::nfts::events::Created>()?;

    if let Some(event) = created_event {
       if let statemint::nfts::events::Created { collection, .. } = event {
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

#[cfg(feature = "nft")]
pub async fn mint_nft(collection_id: u32, nft_id: u32) -> Result<()> {
    let api = OnlineClient::<PolkadotConfig>::from_url("wss://asset-hub-paseo-rpc.dwellir.com").await?;
    println!("{}", "🚀 Connection with parachain established.".green().bold());

    let account_signer = crate::config::load_account_from_config()?;
    let account: MultiAddress<AccountId32, ()> = account_signer.public_key().into();

    let witness: Option<statemint::runtime_types::pallet_nfts::types::MintWitness<u32, u128>> = None;

    let nft_creation_tx = statemint::tx()
        .nfts()
        .mint(collection_id, nft_id, account.clone(), witness);

    // Start the spinner for preparation
    let mut sp = Spinner::new(Spinners::Dots12, "⏳ Preparing transaction...".yellow().bold().to_string());
    sleep(Duration::from_secs(1)).await;
    sp.stop_and_persist("🚀", "Sending transaction to the network...".yellow().bold().to_string());

    let extrinsic_result = api
        .tx()
        .sign_and_submit_then_watch_default(&nft_creation_tx, &account_signer)
        .await?;

    // Update the spinner for finalization with periodic status updates
    let mut sp = Spinner::new(Spinners::Dots12, "⏳ Finalizing transaction...".yellow().bold().to_string());

    for i in 1..=5 {
        sleep(Duration::from_secs(2)).await;
        sp.stop_and_persist(
            "⏳",
            format!("Finalizing transaction... ({}s)", i * 2)
                .yellow()
                .bold()
                .to_string(),
        );
        sp = Spinner::new(Spinners::Dots12, "".into());
    }

    let extrinsic_result = extrinsic_result.wait_for_finalized_success().await?;

    // Stop the spinner with a final message
    sp.stop_and_persist("✅", "NFT minting finalized!".green().bold().to_string());

    let extrinsic_hash = extrinsic_result.extrinsic_hash();

    // Find the `Minted` event or other relevant event
    let minted_event = extrinsic_result.find_first::<statemint::nfts::events::Issued>()?;

    if let Some(event) = minted_event {
        if let statemint::nfts::events::Issued { collection, item, .. } = event {
            println!("\n{}\n", "🎉 NFT Minted Successfully!".blue().bold());
            println!(
                "{}: {}",
                "📦 Collection ID".cyan().bold(),
                collection.to_string().bright_white()
            );
            println!(
                "{}: {}",
                "🎨 NFT ID".cyan().bold(),
                item.to_string().bright_white()
            );
        }
    } else {
        println!("{}", "❌ NFT mint event not found in events.".red().bold());
    }

    println!(
        "{}: {}",
        "🔗 Extrinsic Hash".cyan().bold(),
        format!("{:?}", extrinsic_hash).bright_white()
    );

    Ok(())
}

