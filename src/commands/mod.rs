// commands/mod.rs

use crate::cli::Commands;
use crate::commands::account::account_info;
use crate::config::set_account;
use crate::error::Result;
#[cfg(feature = "nft")]
use crate::commands::mint::{mint_collection, mint_nft};

#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod statemint {}

pub mod account;
pub mod balance;
pub mod mint;



pub async fn run_command(cli: Commands) -> Result<()> {
    match cli {
        #[cfg(feature = "nft")]
        Commands::MintCollection => mint_collection().await,
        #[cfg(feature = "nft")]
        Commands::MintNft { collection_id, nft_id } => mint_nft(collection_id, nft_id).await,
        Commands::SetAccount { mnemonic, secret_uri } => set_account(mnemonic, secret_uri).await,
        Commands::Balance { address } => balance::balance(address).await,
        Commands::Account { public_key } => account_info(public_key).await,
    }
}

