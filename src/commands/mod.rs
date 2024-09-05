// commands/mod.rs

use crate::cli::Commands;
use crate::commands::account::account_info;
use crate::config::set_account;
use crate::error::Result;
use crate::commands::mint_collection::mint_collection;
use crate::commands::mint_nft::mint_nft;
use crate::commands::set_nft_metadata::set_nft_metadata;
use crate::commands::show_nft::show_nft;
use crate::commands::show_collection::show_collection;
use crate::config::set_rpc_url;

#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod assethub {}

pub mod account;
pub mod balance;
pub mod mint_collection;
pub mod mint_nft;
pub mod show_nft;
pub mod set_nft_metadata;
pub mod show_collection;
pub mod send;
#[cfg(feature = "nft")]
pub mod list_nfts;

pub async fn run_command(cli: Commands) -> Result<()> {
    match cli {
#[cfg(feature = "nft")]
        Commands::ListNfts { address } => list_nfts::list_nfts(address).await,
        Commands::MintCollection { json, image } => mint_collection(json.as_deref(), image.as_deref()).await,
        Commands::MintNft { collection_id, nft_id, json, image } => mint_nft(collection_id, nft_id, json.as_deref(), image.as_deref()).await,
        Commands::SetNftMetadata { collection_id, nft_id, json, image } => set_nft_metadata(collection_id, nft_id, json.as_deref(), image.as_deref()).await,
        Commands::ShowNft { collection_id, nft_id, json, image } => show_nft(collection_id, nft_id, json, image).await,
        Commands::ShowCollection { collection_id } => show_collection(collection_id).await,
        Commands::Send { address, amount } => send::send(address, amount).await,
        Commands::SetAccount { mnemonic, secret_uri } => set_account(mnemonic, secret_uri).await,
        Commands::SetRpc { rpc_url } => set_rpc_url(rpc_url).await,
        Commands::Balance { address } => balance::balance(address).await,
        Commands::Account { public_key } => account_info(public_key).await,
    }
}

