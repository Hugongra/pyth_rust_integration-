use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::{Client, Cluster};
use solana_sdk::signature::Keypair;
use std::{rc::Rc, str::FromStr};
use anyhow::Result;
use pyth_solana_receiver_sdk::price_update::{PriceUpdateV2, get_feed_id_from_hex};

const PYTH_SOL_USD: &str = "rec5EKMGg6MxZYaMdyBfgwp4d5rB9T1VQH5pJv5LtFJ";  // Pyth SOL/USD price feed address

#[tokio::main]
async fn main() -> Result<()> {
    // ✅ Load the wallet keypair from file
    let payer = Rc::new(Keypair::new()); // Temporary keypair (not recommended for real use)

    // ✅ Alternative: Load from a file (recommended)
    // let payer = Rc::new(Keypair::from_file("~/.config/solana/id.json")?);

    // ✅ Connect to Solana Devnet
    let client = Client::new(Cluster::Devnet, payer.clone());

    // ✅ Program ID (Replace with your deployed program ID)
    let program_id = Pubkey::from_str("8SkhDK31jrk7hCB7SLpvmCe4UdfhvCgRQkS197YxYTX1")?;
    let program = client.program(program_id)?;

    // ✅ Fetch the Pyth price feed account
    let price_account = program.rpc().get_account(&Pubkey::from_str(PYTH_SOL_USD)?);
    let price_update = PriceUpdateV2::try_from_slice(&price_account.data.borrow())?;

    // ✅ Get the SOL/USD price
    let sol_price = price_update.get_price_no_older_than(
        &solana_sdk::clock::Clock::get()?,
        60,  // Maximum age of the price update (in seconds)
        &get_feed_id_from_hex(PYTH_SOL_USD)?,  // SOL/USD feed ID
    )?;

    // ✅ Print the SOL/USD price
    println!("SOL/USD Price:");
    println!("Price: {:.2}", sol_price.price as f64 * 10f64.powi(sol_price.exponent as i32));
    println!("Confidence: ±{:.2}", sol_price.conf as f64 * 10f64.powi(sol_price.exponent as i32));

    Ok(())
}