use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::{PriceUpdateV2, get_feed_id_from_hex};

declare_id!("8SkhDK31jrk7hCB7SLpvmCe4UdfhvCgRQkS197YxYTX1"); // Replace with your real program ID

#[program]
pub mod send_usd {
    use super::*;

    /// Fetch and log the SOL/USD price
    pub fn fetch_price(ctx: Context<FetchPrice>) -> Result<()> {
        let price_account = &ctx.accounts.price_account;

        // ✅ Decode the Pyth price feed
        let price_update = PriceUpdateV2::try_from_slice(&price_account.data.borrow())?;

        // ✅ Get the SOL/USD price
        let sol_price = price_update.get_price_no_older_than(
            &Clock::get()?,
            60,  // Maximum age of the price update (in seconds)
            &get_feed_id_from_hex("rec5EKMGg6MxZYaMdyBfgwp4d5rB9T1VQH5pJv5LtFJ")?,  // SOL/USD feed ID
        )?;

        // ✅ Log the SOL/USD price
        let sol_price_value = sol_price.price as f64 * 10f64.powi(sol_price.exponent as i32);
        msg!("✅ SOL/USD Price: ${:.2}", sol_price_value); // Log price to Solana logs

        Ok(())
    }
}

#[derive(Accounts)]
pub struct FetchPrice<'info> {
    /// CHECK: Pyth price feed account
    pub price_account: AccountInfo<'info>,
}