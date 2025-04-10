# Dhatu Account Creator

Dhatu is a Rust library that provides core functionality for interacting with Substrate-based blockchains, with a primary focus on Mandala-based blockchains. This project demonstrates how to create an account using the Dhatu library.

## Features

- **Substrate Integration**: Built on top of Substrate framework with support for Mandala-based blockchains
- **Transaction Management**: Comprehensive transaction handling and signing capabilities
- **Identity Management**: Robust keypair and identity management system
- **Runtime Types**: Auto-generated runtime types for Mandala node integration
- **Error Handling**: Comprehensive error handling system

## Prerequisites

- Rust and Cargo installed (version 1.56.0 or later)
- Basic knowledge of Rust programming

## Installation

1. Create a new Rust project:
```bash
cargo new dhatu-account
cd dhatu-account
```

2. Add the required dependencies to your `Cargo.toml`:
```toml
[package]
name = "dhatu-account"
version = "0.1.0"
edition = "2021"

[dependencies]
dhatu = { version = "0.2.2", features = ["sp-keyring", "subxt", "unstable_sp_core"] }
tokio = { version = "1.0", features = ["full"] }
hex = "0.4.3"
sp-keyring = "24.0.0"
sp-core = "21.0.0"
```

## Implementation

The following code demonstrates how to:
1. Connect to a blockchain node (using a dummy WebSocket URL)
2. Create an account using the Sr25519 keyring
3. Get the public key and account ID

```rust
use dhatu::ext::sp_core::crypto::{AccountId32, Pair};
use dhatu::ext::sp_keyring::sr25519::Keyring as Sr25519Keyring;
use dhatu::ext::subxt::{OnlineClient, SubstrateConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the blockchain node (replace with your actual WebSocket URL)
    let api = OnlineClient::<SubstrateConfig>::from_url("wss://your-blockchain-node.com").await?;
    println!("Connected to blockchain node.");

    // Create a default Alice keypair using the keyring
    let alice = Sr25519Keyring::Alice;
    let keypair = alice.pair();
    let public_key = keypair.public();
    let account_id = AccountId32::from(public_key);

    println!("Alice's public key: {:?}", public_key);
    println!("Alice's account id: {:?}", account_id);

    Ok(())
}
```

## Running the Project

To run the project:
```bash
cargo run
```

Expected output:
```
Connected to blockchain node.
Alice's public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d (5GrwvaEF...)
Alice's account id: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d (5GrwvaEF...)
```

## Explanation

1. **Connection**: The code connects to a blockchain node using a WebSocket URL. Replace `wss://your-blockchain-node.com` with your actual node's WebSocket URL.

2. **Keypair Generation**: 
   - Uses `Sr25519Keyring::Alice` to create a predefined keypair
   - The keyring provides several predefined accounts (Alice, Bob, Charlie, etc.)

3. **Account Information**:
   - `public_key`: The raw public key in hex format
   - `account_id`: The SS58-encoded account address (starts with "5GrwvaEF...")

## Library Features

The Dhatu library provides several feature flags:

- `default`: Includes tokio and serde
- `unstable`: Enables all features including substrate primitives
- `sp_keyring`: Enables substrate keyring functionality
- `asset_migration`: Enables asset migration features
- `tokio`: Async runtime support
- `serde`: Serialization/deserialization support
- `unstable_sp_core`: Unstable substrate core features
- `subxt`: Substrate extrinsic support

## Next Steps

You can extend this implementation to:
- Check account balance
- Make transactions
- Create custom keypairs
- Sign and verify messages

## Dependencies

- `subxt`: Substrate extrinsic support
- `schnorrkel`: Cryptography
- `tokio`: Async runtime
- `serde`: Serialization
- `parity-scale-codec`: SCALE codec
- `sp-keyring`: Substrate keyring (optional)

## Resources

- [Dhatu Documentation](https://docs.rs/dhatu/latest/dhatu/)
- [Mandala Chain Documentation](https://mandalachain.io/docs)
- [Substrate Documentation](https://docs.substrate.io/)
- [Subxt Documentation](https://docs.rs/subxt/latest/subxt/)

## License

This project is licensed under the Apache-2.0 License.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. 
