# Stylus ERC-20 Token Example (Rust)

This project demonstrates the implementation of an ERC-20 token using [Stylus](https://developer.arbitrum.io/stylus/) and Rust. It follows the WASM contract standard for Arbitrum Nitro, incorporating features like `Ownable`, `Mintable`, and a CLI interface to mint tokens.

## Project Structure

- `lib.rs`: defines the main contract, composing `Erc20`, `Ownable`, and `Erc20Metadata`.
- `erc20.rs`: extends the base ERC-20 logic with a unified error handler `MyERC20Error`.
- `main.rs`: deployment entry point using Stylus.
- `example.rs`: CLI script to mint tokens to a specified address.

## Requirements

- Rust (nightly toolchain recommended)
- [cargo-stylus](https://docs.arbitrum.io/stylus/quickstart/) installed:
  ```bash
  cargo install cargo-stylus
  ```
- Arbitrum Nitro node running locally or access to a compatible JSON-RPC endpoint

## Build and Deploy

### 1. Build the contract

```bash
cargo stylus build
```

### 2. Deploy

Make sure your private key is a valid 64-character hex string prefixed with `0x`:

```bash
cargo stylus deploy \
  --private-key 0xyour64charhexkey000000000000000000000000000000000000000000000000000000000000 \
  --no-verify
```

> If you see `Odd number of digits`, your key is likely malformed.

## CLI: Mint Tokens

### Dependencies (in `Cargo.toml`):

```toml
[dependencies]
ethers = { version = "2", features = ["abigen", "tokio", "rustls-tls"] }
eyre = "0.6"
dotenvy = "0.15"
tokio = { version = "1.37", features = ["macros", "rt-multi-thread"] }
```

### Configure environment variables

Create a `.env` file:

```
PRIVATE_KEY=0x...
TOKEN_ADDRESS=0x...
RECIPIENT=0x...
RPC_URL=https://...
```

### Run mint script

```bash
cargo run --bin example
```

This will mint 1,000 tokens (with 18 decimals) to the address defined in `RECIPIENT` using the deployed token contract.

Happy hacking with Stylus and Rust 🦀✨
