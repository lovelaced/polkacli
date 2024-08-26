use subxt_signer::sr25519::Keypair;
use subxt_signer::bip39::Mnemonic;
use std::fs;
use std::io::{self, Write};
use dirs;
use std::str::FromStr;
use subxt_signer::SecretUri;
use crate::error::Result;

pub async fn set_account(mnemonic: Option<String>, secret_uri: Option<String>) -> Result<()> {
    if let Some(mnemonic) = mnemonic {
        set_account_from_mnemonic(mnemonic).await
    } else if let Some(secret_uri) = secret_uri {
        set_account_from_uri(secret_uri).await
    } else {
        Err("No mnemonic or secret URI provided.".into())
    }
}

async fn set_account_from_mnemonic(mnemonic: String) -> Result<()> {
    let mnemonic = Mnemonic::parse(&mnemonic)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    let _pair = Keypair::from_phrase(&mnemonic, None)?;

    save_account_to_config("mnemonic", &mnemonic.to_string())?;

    println!("Account mnemonic saved successfully.");
    Ok(())
}

async fn set_account_from_uri(secret_uri: String) -> Result<()> {
    let suri = SecretUri::from_str(&secret_uri)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    let _pair = Keypair::from_uri(&suri)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    save_account_to_config("secret_uri", &secret_uri)?;

    println!("Account secret URI saved successfully.");
    Ok(())
}

pub fn save_account_to_config(key: &str, value: &str) -> Result<()> {
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

pub fn load_account_from_config() -> Result<Keypair> {
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

