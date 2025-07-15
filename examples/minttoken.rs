//! src/bin/example.rs
//! Simple CLI that mints 1 000 tokens to a given recipient using your Stylus‑based ERC‑20 contract.
//!
//! Usage (environment variables):
//!   PRIVATE_KEY=0x...   # 64‑hex‑char key with 0x prefix – must be the contract owner
//!   TOKEN_ADDRESS=0x... # deployed contract address
//!   RECIPIENT=0x...     # address that will receive the tokens
//!   RPC_URL=https://... # JSON‑RPC endpoint (defaults to http://localhost:8547)
//!
//! Then run:
//!   cargo run --bin example
//!
//! Dependencies (add to Cargo.toml):
//!   ethers   = { version = "2", features = ["abigen", "tokio", "rustls-tls"] }
//!   eyre     = "0.6"
//!   dotenvy  = "0.15"
//!   tokio    = { version = "1.37", features = ["macros", "rt-multi-thread"] }

use ethers::prelude::*;
use std::{env, hash::Hash, sync::Arc, time::Duration};

abigen!(
    MyErc20Token,
    r#"[
        function mint(address to, uint256 amount) external
    ]"#
);

/// Decimals used by the token – change if your contract uses a different value.
const DECIMALS: usize = 18;
/// Whole tokens to mint.
const AMOUNT_UNITS: u64 = 1_000;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Load .env if present
    dotenv::dotenv().ok();

    // ---------------- Wallet ----------------
    let pk =
        env::var("PRIVATE_KEY").expect("PRIVATE_KEY env var missing – hex string with 0x prefix");
    let wallet: LocalWallet = pk.parse()?;

    // --------------- Provider --------------
    let rpc = env::var("RPC_URL").unwrap_or_else(|_| "http://localhost:8547".to_owned());
    let provider = Provider::<Http>::try_from(rpc)?.interval(Duration::from_millis(1500));

    let chain_id = provider.get_chainid().await?;
    let wallet = wallet.with_chain_id(chain_id.as_u64());
    let client = SignerMiddleware::new(provider, wallet);
    let client = Arc::new(client);

    // -------------- Contract --------------
    let addr: Address = env::var("TOKEN_ADDRESS")
        .expect("TOKEN_ADDRESS env var missing")
        .parse()?;
    let token = MyErc20Token::new(addr, client.clone());

    // -------------- Recipient -------------
    let recipient: Address = env::var("RECIPIENT")
        .expect("RECIPIENT env var missing")
        .parse()?;

    // -------------- Amount ----------------
    let amount = U256::from(AMOUNT_UNITS) * U256::exp10(DECIMALS);

    println!("Minting {AMOUNT_UNITS} tokens to {recipient:?}…");
    let call = token.mint(recipient, amount);
    let sent_tx = call.send().await?;
    let pending_tx = sent_tx.interval(Duration::from_secs(2));
    let receipt = pending_tx
        .await?
        .expect("tx dropped from mempool or failed");
    println!("Tx mined ✅ – hash: {:?}", receipt.transaction_hash);

    Ok(())
}
