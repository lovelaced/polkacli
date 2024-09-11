use crate::error::Result;
use colored::*;
use spinners::{Spinner, Spinners};
use subxt::{
    utils::{AccountId32, MultiAddress},
};
use std::str::FromStr;
use std::time::Duration;
use tokio::time::sleep;
use crate::commands::assethub;
use crate::client::get_client;

pub async fn send(recipient: String, amount: f64) -> Result<()> {
    // Establish a connection to the parachain
    let api = get_client().await?;
    println!("{}", "🚀 Connection with parachain established.".green().bold());

    // Parse recipient address
    let recipient: AccountId32 = AccountId32::from_str(&recipient)?;

    // Load sender account from config
    let account_signer = crate::config::load_account_from_config()?;
    let from: AccountId32 = account_signer.public_key().into();

    // Convert PAS to Planck (1 PAS = 10^10 Plancks)
    let plancks_per_pas = 10u128.pow(10) as f64;
    let amount_in_plancks = (amount * plancks_per_pas).round() as u128;

    // Create transfer payload
    let payload = assethub::tx().balances().transfer_keep_alive(MultiAddress::Id(recipient.clone()), amount_in_plancks);

    // Start the spinner for preparation
    let mut sp = Spinner::new(Spinners::Dots12, "⏳ Preparing transaction...".yellow().bold().to_string());
    sleep(Duration::from_secs(1)).await;
    sp.stop_and_persist("🚀", "Sending transaction to the network...".yellow().bold().to_string());

    // Sign and submit the transaction
    let extrinsic_result = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &account_signer)
        .await?;

    // Update the spinner for finalization with periodic status updates
    let mut sp = Spinner::new(Spinners::Dots12, "⏳ Finalizing transaction...".yellow().bold().to_string());

    let extrinsic_result = extrinsic_result.wait_for_finalized_success().await?;

    // Stop the spinner with a final message
    sp.stop_and_persist("✅", "Funds sent successfully!".green().bold().to_string());

    let extrinsic_hash = extrinsic_result.extrinsic_hash();

    // Output the result
    println!("\n{}\n", "💸 Transfer Details".blue().bold());
    println!(
        "{}: {}",
        "📤 From".cyan().bold(),
        from.to_string().bright_white()
    );
    println!(
        "{}: {}",
        "📥 To".cyan().bold(),
        recipient.to_string().bright_white()
    );
    println!(
        "{}: {:.10} PAS",
        "💰 Amount".cyan().bold(),
        amount
    );
    println!(
        "{}: {}",
        "🔗 Extrinsic Hash".cyan().bold(),
        format!("{:?}", extrinsic_hash).bright_white()
    );

    Ok(())
}

