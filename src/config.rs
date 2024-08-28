use crate::error::Result;
use dirs::home_dir;
use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
    str::FromStr,
};
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::{bip39::Mnemonic, sr25519::Keypair, SecretUri};

const CONFIG_DIR: &str = ".polkacli";
const CONFIG_FILE: &str = "config";

fn config_file_path() -> PathBuf {
    home_dir()
        .expect("Unable to find home directory")
        .join(CONFIG_DIR)
        .join(CONFIG_FILE)
}

fn read_config_file() -> Result<String> {
    fs::read_to_string(config_file_path()).map_err(|e| {
        if e.kind() == io::ErrorKind::NotFound {
            io::Error::new(io::ErrorKind::NotFound, "Configuration file not found").into()
        } else {
            e.into()
        }
    })
}

fn write_config_file(content: &str) -> Result<()> {
    let config_path = config_file_path();
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(config_path)?;

    writeln!(file, "{}", content)?;
    Ok(())
}

fn update_config(key: &str, value: &str) -> Result<()> {
    let mut config_content = read_config_file().unwrap_or_default();

    let mut updated = false;
    let new_content = config_content
        .lines()
        .map(|line| {
            if line.starts_with(key) {
                updated = true;
                format!("{} = \"{}\"", key, value)
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<String>>();

    if !updated {
        let new_line = format!("{} = \"{}\"", key, value);
        config_content.push('\n');
        config_content.push_str(&new_line);
    }

    write_config_file(&new_content.join("\n"))?;
    Ok(())
}

pub async fn set_account(mnemonic: Option<String>, secret_uri: Option<String>) -> Result<()> {
    match (mnemonic, secret_uri) {
        (Some(mnemonic), None) => set_account_from_mnemonic(mnemonic).await,
        (None, Some(secret_uri)) => set_account_from_uri(secret_uri).await,
        _ => Err("No mnemonic or secret URI provided.".into()),
    }
}

async fn set_account_from_mnemonic(mnemonic: String) -> Result<()> {
    let mnemonic = Mnemonic::parse(&mnemonic)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    Keypair::from_phrase(&mnemonic, None)?;
    update_config("mnemonic", &mnemonic.to_string())?;
    println!("Account mnemonic saved successfully.");
    Ok(())
}

async fn set_account_from_uri(secret_uri: String) -> Result<()> {
    let suri = SecretUri::from_str(&secret_uri)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    Keypair::from_uri(&suri)?;
    update_config("secret_uri", &secret_uri)?;
    println!("Account secret URI saved successfully.");
    Ok(())
}

pub async fn set_rpc_url(url: String) -> Result<()> {
    if OnlineClient::<PolkadotConfig>::from_url(&url).await.is_err() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Failed to connect to the provided RPC URL.").into());
    }

    update_config("rpc_url", &url)?;
    println!("RPC URL saved successfully.");
    Ok(())
}

fn load_value_from_config(key: &str) -> Result<Option<String>> {
    let config_content = read_config_file()?;
    Ok(config_content
        .lines()
        .find_map(|line| {
            if line.starts_with(key) {
                line.splitn(2, " = ").nth(1).map(|value| value.trim_matches('"').to_string())
            } else {
                None
            }
        }))
}

pub fn load_account_from_config() -> Result<Keypair> {
    if let Some(mnemonic) = load_value_from_config("mnemonic")? {
        let mnemonic = Mnemonic::parse(&mnemonic)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        Keypair::from_phrase(&mnemonic, None)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    } else if let Some(secret_uri) = load_value_from_config("secret_uri")? {
        let suri = SecretUri::from_str(&secret_uri)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        Keypair::from_uri(&suri)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    } else {
        Err(io::Error::new(io::ErrorKind::InvalidInput, "No valid mnemonic or secret URI found in config file.").into())
    }
}

pub fn load_rpc_url_from_config() -> Result<String> {
    load_value_from_config("rpc_url")?.ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "No valid RPC URL found in config file.").into())
}

pub fn load_pinata_jwt_from_config() -> Result<Option<String>> {
    load_value_from_config("pinata_jwt")
}

