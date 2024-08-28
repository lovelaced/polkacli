use subxt::{OnlineClient, PolkadotConfig};
use crate::config::load_rpc_url_from_config;
use crate::error::Result;
use once_cell::sync::OnceCell;

static CLIENT: OnceCell<OnlineClient<PolkadotConfig>> = OnceCell::new();

pub async fn get_client() -> Result<&'static OnlineClient<PolkadotConfig>> {
    if CLIENT.get().is_none() {
        let url = load_rpc_url_from_config().unwrap_or_else(|_| {
            "wss://asset-hub-paseo-rpc.dwellir.com".to_string() // Default value if not configured
        });
        let client = OnlineClient::<PolkadotConfig>::from_url(&url).await?;
        CLIENT.set(client).unwrap();
    }
    Ok(CLIENT.get().unwrap())
}

