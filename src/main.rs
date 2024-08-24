use clap::{Parser, Subcommand};
use subxt::{
    PolkadotConfig,
    utils::{AccountId32, MultiAddress},
    OnlineClient,
};
use subxt_signer::sr25519::Keypair;
use subxt_signer::bip39::Mnemonic;
use std::fs;
use std::io::{self, Write};
use dirs;
use std::str::FromStr;
use subxt_signer::SecretUri;


// This is the generated module from your metadata file
#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod statemint {}

type StatemintConfig = PolkadotConfig;

#[derive(Parser)]
#[command(name = "polkacli")]
#[command(about = "CLI for interacting with Statemint")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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
    /// Set the account to use for minting
    SetAccount {
        #[arg(long, value_name = "MNEMONIC", conflicts_with = "secret_uri")]
        mnemonic: Option<String>,
        #[arg(long, value_name = "SECRET_URI", conflicts_with = "mnemonic")]
        secret_uri: Option<String>,
    },
    /// Print the balance of the configured account
    Balance,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Err(err) = match cli.command {
        #[cfg(feature = "nft")]
        Commands::MintCollection => mint_collection().await,
        #[cfg(feature = "nft")]
        Commands::MintNft { collection_id, nft_id } => mint_nft(collection_id, nft_id).await,
        Commands::SetAccount { mnemonic, secret_uri } => {
            if let Some(mnemonic) = mnemonic {
                set_account_from_mnemonic(mnemonic).await
            } else if let Some(secret_uri) = secret_uri {
                set_account_from_uri(secret_uri).await
            } else {
                Err("No mnemonic or secret URI provided.".into())
            }
        },
        Commands::Balance => balance().await,
    } {
        eprintln!("{:?}", err);
    }
}

#[cfg(feature = "nft")]
async fn mint_collection() -> Result<(), Box<dyn std::error::Error>> {
    let api = OnlineClient::<StatemintConfig>::from_url("wss://asset-hub-paseo-rpc.dwellir.com").await?;
    println!("Connection with parachain established.");

    let account_signer = load_account_from_config()?;
    let admin: MultiAddress<AccountId32, ()> = account_signer.public_key().into();

    // Use the `all_enabled` settings for the collection
    let config = statemint::runtime_types::pallet_nfts::types::CollectionConfig {
        settings: statemint::runtime_types::pallet_nfts::types::CollectionSettings::all_enabled(),
        max_supply: None,
        mint_settings: statemint::runtime_types::pallet_nfts::types::MintSettings {
            mint_type: statemint::runtime_types::pallet_nfts::types::MintType::Issuer,
            price: None,
            start_block: None,
            end_block: None,
            default_item_settings: statemint::runtime_types::pallet_nfts::types::ItemSettings::all_enabled(),
            __ignore: Default::default(), // Adding the __ignore field
        },
        __ignore: Default::default(), // Adding the __ignore field
    };

    // Send the `create` transaction
    let collection_creation_tx = statemint::tx()
        .nfts()
        .create(admin, config);

    let _collection_creation_events = api
        .tx()
        .sign_and_submit_then_watch_default(&collection_creation_tx, &account_signer)
        .await
        .map(|e| {
            println!("Collection creation submitted, waiting for transaction to be finalized...");
            e
        })?
        .wait_for_finalized_success()
        .await?;

    println!("Collection created.");

    Ok(())
}

#[cfg(feature = "nft")]
async fn mint_nft(collection_id: u32, nft_id: u32) -> Result<(), Box<dyn std::error::Error>> {
    let api = OnlineClient::<StatemintConfig>::from_url("wss://asset-hub-paseo-rpc.dwellir.com").await?;
    println!("Connection with parachain established.");

    let account_signer = load_account_from_config()?;
    let account: MultiAddress<AccountId32, ()> = account_signer.public_key().into();

    // Optionally include the witness (can be None if not required)
    let witness: Option<statemint::runtime_types::pallet_nfts::types::MintWitness<u32, u128>> = None;

    let nft_creation_tx = statemint::tx()
        .nfts()
        .mint(collection_id, nft_id, account.clone(), witness);

    let _nft_creation_events = api
        .tx()
        .sign_and_submit_then_watch_default(&nft_creation_tx, &account_signer)
        .await
        .map(|e| {
            println!("NFT creation submitted, waiting for transaction to be finalized...");
            e
        })?
        .wait_for_finalized_success()
        .await?;

    println!("NFT created.");

    Ok(())
}

async fn balance() -> Result<(), Box<dyn std::error::Error>> {
    let api = OnlineClient::<StatemintConfig>::from_url("wss://asset-hub-paseo-rpc.dwellir.com").await?;
    println!("Connection with parachain established.");

    let account_signer = load_account_from_config()?;
    let account: subxt::utils::AccountId32 = account_signer.public_key().into();

    // Hardcoded decimals value (12) since 1 PAS = 10^12 Plancks
    let decimals = 10;

    // Build a storage query to access the system account information.
    let storage_query = statemint::storage().system().account(account.clone());

    // Fetch the account information.
    let result: Option<
        statemint::runtime_types::frame_system::AccountInfo<
            u32,
            statemint::runtime_types::pallet_balances::types::AccountData<u128>,
        >
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;

    // Handle the case where the account has no data (i.e., account doesn't exist).
    if let Some(account_info) = result {
        // Get the free balance
        let free_balance = account_info.data.free;

        // Convert the free balance to a human-readable format
        let unit = 10u128.pow(decimals as u32);
        let human_readable_balance = free_balance as f64 / unit as f64;

        println!("Free balance of account {}: {:.12} PAS", account, human_readable_balance);
    } else {
        // Print a message when no account data is found
        println!("No account data found for account {}", account);
    }

    Ok(())
}

async fn set_account_from_mnemonic(mnemonic: String) -> Result<(), Box<dyn std::error::Error>> {
    let mnemonic = Mnemonic::parse(&mnemonic)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    let pair = Keypair::from_phrase(&mnemonic, None)?;

    save_account_to_config("mnemonic", &mnemonic.to_string())?;

    println!("Account mnemonic saved successfully.");
    Ok(())
}

async fn set_account_from_uri(secret_uri: String) -> Result<(), Box<dyn std::error::Error>> {
    let suri = SecretUri::from_str(&secret_uri)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    let pair = Keypair::from_uri(&suri)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    save_account_to_config("secret_uri", &secret_uri)?;

    println!("Account secret URI saved successfully.");
    Ok(())
}

fn save_account_to_config(key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = dirs::home_dir().unwrap().join(".polkacli");
    let config_file = config_dir.join("config");

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(config_file)?;

    writeln!(file, "{} = \"{}\"", key, value)?;

    Ok(())
}

fn load_account_from_config() -> Result<Keypair, Box<dyn std::error::Error>> {
    let config_file = dirs::home_dir().unwrap().join(".polkacli").join("config");

    if !config_file.exists() {
        return Err(Box::new(io::Error::new(io::ErrorKind::NotFound, "Account configuration not found.")));
    }

    let config_content = fs::read_to_string(config_file)?;

    if let Some(mnemonic_phrase) = config_content
        .lines()
        .find(|line| line.starts_with("mnemonic"))
        .and_then(|line| line.split(" = ").nth(1))
        .map(|mnemonic| mnemonic.trim_matches('"'))
    {
        let mnemonic = Mnemonic::parse(mnemonic_phrase)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        return Keypair::from_phrase(&mnemonic, None)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>);
    }

    if let Some(secret_uri) = config_content
        .lines()
        .find(|line| line.starts_with("secret_uri"))
        .and_then(|line| line.split(" = ").nth(1))
        .map(|secret_uri| secret_uri.trim_matches('"'))
    {
        let suri = SecretUri::from_str(secret_uri)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        return Keypair::from_uri(&suri)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>);
    }

    Err(Box::new(io::Error::new(io::ErrorKind::InvalidInput, "No valid mnemonic or secret URI found in config file.")))
}

