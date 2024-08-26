use crate::error::Result;
use subxt::{OnlineClient, PolkadotConfig, utils::{AccountId32, MultiAddress}};
use crate::commands::statemint;
use pallet_nfts::{CollectionSettings, ItemSettings};
use std::marker::PhantomData;

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
    println!("Connection with parachain established.");

    let account_signer = crate::config::load_account_from_config()?;
    let admin: MultiAddress<AccountId32, ()> = account_signer.public_key().into();

    // Define the collection config using the actual types from pallet_nfts
    let config = statemint::runtime_types::pallet_nfts::types::CollectionConfig {
        settings: to_collection_bitflags(CollectionSettings::all_enabled()), // Convert using to_collection_bitflags
        max_supply: None,
        mint_settings: statemint::runtime_types::pallet_nfts::types::MintSettings {
            mint_type: statemint::runtime_types::pallet_nfts::types::MintType::Issuer,
            price: None,
            start_block: None,
            end_block: None,
            default_item_settings: to_item_bitflags(ItemSettings::all_enabled()), // Convert using to_item_bitflags
            __ignore: Default::default(),
        },
        __ignore: Default::default(),
    };

    // Directly pass the config without wrapping in Static
    let payload = statemint::tx().nfts().create(admin, config);

    let _collection_creation_events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &account_signer)
        .await?
        .wait_for_finalized_success()
        .await?;

    println!("Collection created.");

    Ok(())
}

#[cfg(feature = "nft")]
pub async fn mint_nft(collection_id: u32, nft_id: u32) -> Result<()> {
    let api = OnlineClient::<PolkadotConfig>::from_url("wss://asset-hub-paseo-rpc.dwellir.com").await?;
    println!("Connection with parachain established.");

    let account_signer = crate::config::load_account_from_config()?;
    let account: MultiAddress<AccountId32, ()> = account_signer.public_key().into();

    // Optionally include the witness (can be None if not required)
    let witness: Option<statemint::runtime_types::pallet_nfts::types::MintWitness<u32, u128>> = None;

    let nft_creation_tx = statemint::tx()
        .nfts()
        .mint(collection_id, nft_id, account.clone(), witness);

    let _nft_creation_events = api
        .tx()
        .sign_and_submit_then_watch_default(&nft_creation_tx, &account_signer)
        .await?
        .wait_for_finalized_success()
        .await?;

    println!("NFT created.");

    Ok(())
}

