use crate::error::Result;
use subxt::{OnlineClient, PolkadotConfig, utils::{AccountId32, MultiAddress}};
use crate::commands::statemint;


#[cfg(feature = "nft")]
pub async fn mint_collection() -> Result<()> {
    let api = OnlineClient::<PolkadotConfig>::from_url("wss://asset-hub-paseo-rpc.dwellir.com").await?;
    println!("Connection with parachain established.");

    let account_signer = crate::config::load_account_from_config()?;
    let admin: MultiAddress<AccountId32, ()> = account_signer.public_key().into();

    let config = statemint::runtime_types::pallet_nfts::types::CollectionConfig {
        settings: statemint::runtime_types::pallet_nfts::types::CollectionSettings::all_enabled(),
        max_supply: None,
        mint_settings: statemint::runtime_types::pallet_nfts::types::MintSettings {
            mint_type: statemint::runtime_types::pallet_nfts::types::MintType::Issuer,
            price: None,
            start_block: None,
            end_block: None,
            default_item_settings: statemint::runtime_types::pallet_nfts::types::ItemSettings::all_enabled(),
            __ignore: Default::default(),
        },
        __ignore: Default::default(),
    };

    let collection_creation_tx = statemint::tx().nfts().create(admin, config);

    let _collection_creation_events = api
        .tx()
        .sign_and_submit_then_watch_default(&collection_creation_tx, &account_signer)
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

