use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "polkacli")]
#[command(about = "CLI for interacting with AssetHub")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[cfg(feature = "nft")]
    /// Mint a new NFT collection
    MintCollection {
        /// Folder containing JSON metadata files
        #[arg(long)]
        json: Option<String>,
        /// Folder containing images
        #[arg(long)]
        image: Option<String>,
    },
    #[cfg(feature = "nft")]
    /// Mint a new NFT within a collection
    MintNft {
        #[arg(value_name = "COLLECTION_ID")]
        collection_id: u32,
        #[arg(value_name = "NFT_ID")]
        nft_id: u32,
        /// Path to JSON metadata file
        #[arg(long)]
        json: Option<String>,
        /// Path to image file
        #[arg(long)]
        image: Option<String>,
    },
    ShowNft {
        #[arg(value_name = "COLLECTION_ID")]
        collection_id: u32,

        #[arg(value_name = "NFT_ID")]
        nft_id: u32,

        /// Flag to fetch and display JSON metadata
        #[clap(long)]
        json: bool,

        /// Flag to fetch and display the image
        #[clap(long)]
        image: bool,
    },

    ShowCollection {
        #[arg(value_name = "COLLECTION_ID")]
        collection_id: u32,
    },

    /// Set the account to use for CLI
    SetAccount {
        #[arg(long, value_name = "MNEMONIC", conflicts_with = "secret_uri")]
        mnemonic: Option<String>,

        #[arg(long, value_name = "SECRET_URI", conflicts_with = "mnemonic")]
        secret_uri: Option<String>,
    },

    /// Set the RPC URL for the client
    SetRpc {
        #[arg(value_name = "RPC_URL")]
        rpc_url: String,
    },

    /// Send funds to an address
    Send {
        #[arg(value_name = "ADDRESS")]
        address: String,

        #[arg(value_name = "AMOUNT")]
        amount: u128,
    },

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

