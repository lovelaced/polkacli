use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "polkacli")]
#[command(about = "CLI for interacting with Statemint")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[cfg(feature = "nft")]
    /// Mint a new NFT collection
    MintCollection,
    #[cfg(feature = "nft")]
    /// Mint a new NFT within a collection
    MintNft {
        #[arg(value_name = "COLLECTION_ID")]
        collection_id: u32,
        #[arg(value_name = "NFT_ID")]
        nft_id: u32,
    },
    ShowNft {
        #[arg(value_name = "COLLECTION_ID")]
        collection_id: u32,
        #[arg(value_name = "NFT_ID")]
        nft_id: u32,
    },
    /// Set the account to use for cli
    SetAccount {
        #[arg(long, value_name = "MNEMONIC", conflicts_with = "secret_uri")]
        mnemonic: Option<String>,
        #[arg(long, value_name = "SECRET_URI", conflicts_with = "mnemonic")]
        secret_uri: Option<String>,
    },
    /// Send funds to address for amount
    Send {
        #[arg(value_name = "ADDRESS")]
        address: String,
        #[arg(value_name = "AMOUNT")]
        amount: u128,
    },
    /// Print the balance of the configured account
    /// Print the balance of the configured account or a provided address
    Balance {
        #[arg(value_name = "ADDRESS")]
        address: Option<String>,
    },
    /// Retrieve information for a given account by public key
    Account {
        #[arg(value_name = "PUBLIC_KEY")]
        public_key: String,
    },
}

