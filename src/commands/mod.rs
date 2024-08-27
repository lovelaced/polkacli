// commands/mod.rs

use crate::cli::Commands;
use crate::commands::account::account_info;
use crate::config::set_account;
use crate::error::Result;
#[cfg(feature = "nft")]
use crate::commands::mint::{mint_collection, mint_nft};
use crate::commands::show_nft::show_nft;
use crate::commands::show_collection::show_collection;

#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod assethub {}

pub mod account;
pub mod balance;
pub mod mint;
pub mod show_nft;
pub mod show_collection;
pub mod send;

pub async fn run_command(cli: Commands) -> Result<()> {
    match cli {
        #[cfg(feature = "nft")]
        Commands::MintCollection => mint_collection().await,
        #[cfg(feature = "nft")]
        Commands::MintNft { collection_id, nft_id } => mint_nft(collection_id, nft_id).await,
        Commands::ShowNft { collection_id, nft_id } => show_nft(collection_id, nft_id).await,
        Commands::ShowCollection { collection_id } => show_collection(collection_id).await,
        Commands::Send { address, amount } => send::send(address, amount).await,
        Commands::SetAccount { mnemonic, secret_uri } => set_account(mnemonic, secret_uri).await,
        Commands::Balance { address } => balance::balance(address).await,
        Commands::Account { public_key } => account_info(public_key).await,
    }
}

