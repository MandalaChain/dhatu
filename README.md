# Dhatu

Dhatu is a Rust library that provides core functionality for interacting with Substrate-based blockchains, with a primary focus on Mandala-based blockchains. It aims to abstract away the complexity of blockchain interactions while providing a robust and developer-friendly interface.

## Features

- **Substrate Integration**: Built on top of Substrate framework with support for Mandala-based blockchains
- **Transaction Management**: Comprehensive transaction handling and signing capabilities
- **Identity Management**: Robust keypair and identity management system
- **Runtime Types**: Auto-generated runtime types for Mandala node integration
- **Error Handling**: Comprehensive error handling system
- **Extensible Architecture**: Modular design allowing for future extensions to other Substrate-based blockchains

## Components

### Dhatu Core (`dhatu/`)

The core library providing the main functionality:

- **types**: Global crate-level types
- **tx**: Transaction module with extrinsic abstractions
- **registrar**: Identity management and keypair handling
- **runtime_types**: Auto-generated Mandala node runtime types
- **error**: Error handling system
- **ext**: External library re-exports

### Mandala Node Runner (`mandala-node-runner/`)

A utility for running and managing Mandala nodes:

- **SubstrateNodeBuilder**: Configurable builder for setting up Substrate nodes
- **SubstrateNode**: Runtime management of Substrate nodes
- **Error Handling**: Comprehensive error management for node operations

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
dhatu = { version = "0.2.2", features = ["unstable"] }
mandala-node-runner = { path = "path/to/mandala-node-runner" }
```

## Features

The library provides several feature flags:

- `default`: Includes tokio and serde
- `unstable`: Enables all features including substrate primitives
- `sp_keyring`: Enables substrate keyring functionality
- `asset_migration`: Enables asset migration features
- `tokio`: Async runtime support
- `serde`: Serialization/deserialization support
- `unstable_sp_core`: Unstable substrate core features
- `subxt`: Substrate extrinsic support

## Usage

### Basic Setup

```rust
use dhatu::registrar::Keypair;
use dhatu::tx::Transaction;

// Create a new keypair
let keypair = Keypair::generate();

// Create and sign a transaction
let tx = Transaction::new()
    .with_signer(&keypair)
    .build();
```

### Running a Mandala Node

```rust
use mandala_node_runner::SubstrateNode;

// Start a new node
let node = SubstrateNode::builder()
    .arg("--dev")
    .spawn()?;

// Get the WebSocket port
let ws_port = node.ws_port();

// Node will be automatically killed when dropped
```

## Dependencies

- `subxt`: Substrate extrinsic support
- `schnorrkel`: Cryptography
- `tokio`: Async runtime
- `serde`: Serialization
- `parity-scale-codec`: SCALE codec
- `sp-keyring`: Substrate keyring (optional)

## License

This project is licensed under the Apache-2.0 License.

## Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

## Documentation

For more detailed documentation, please refer to:
- [Substrate Documentation](https://substrate.dev/docs/en/)
- [Subxt Documentation](https://docs.rs/subxt/latest/subxt/)
- [Mandala Documentation](https://github.com/zianksm/dhatu/tree/dev/dhatu#readme)
