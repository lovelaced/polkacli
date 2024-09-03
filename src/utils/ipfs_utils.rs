use crate::error::Result;
use reqwest::Client;
use std::error::Error;

pub async fn pin_to_ipfs(data: &[u8]) -> Result<String> {
    let pinata_jwt = crate::config::load_pinata_jwt_from_config()?;
    let pinata_gateway = "https://api.pinata.cloud";

    let client = Client::new();

    if let Some(jwt) = pinata_jwt {
        let form = reqwest::multipart::Form::new()
            .part("file", reqwest::multipart::Part::bytes(data.to_vec()).file_name("data"));

        let response = client
            .post(&format!("{}/pinning/pinFileToIPFS", pinata_gateway))
            .bearer_auth(jwt)  // Use JWT for authorization
            .multipart(form)
            .send()
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        if !response.status().is_success() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to pin to IPFS via Pinata: {:?}", response.text().await),
            )));
        }

        let pin_response: serde_json::Value = response.json().await.map_err(|e| Box::new(e) as Box<dyn Error>)?;
        let ipfs_hash = pin_response["IpfsHash"].as_str().ok_or("Failed to parse IPFS hash from Pinata response")?;

        Ok(format!("ipfs://{}", ipfs_hash))
    } else {
        let response = client
            .post("https://ipfs.io/ipfs")
            .body(data.to_vec())
            .send()
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let ipfs_hash = response.text().await.map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(format!("ipfs://{}", ipfs_hash))
    }
}

