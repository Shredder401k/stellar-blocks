# Stellar Blocks - Smart Contracts

This directory contains Soroban smart contracts for the stellar-blocks project. Each contract is designed to be modular and can be developed independently by contributors.

## Structure

```
contracts/
├── README.md           # This file
├── Cargo.toml          # Workspace configuration
└── example/            # Example starter contract
    ├── Cargo.toml
    └── src/
        ├── lib.rs
        └── test.rs
```

## Prerequisites

Before working with these contracts, ensure you have the following installed:

1. **Rust** (latest stable version)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Soroban CLI**
   ```bash
   cargo install --locked soroban-cli
   ```

3. **WebAssembly target**
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

## Building Contracts

Build all contracts in the workspace:
```bash
cd contracts
cargo build --target wasm32-unknown-unknown --release
```

Build a specific contract:
```bash
cd contracts/example
cargo build --target wasm32-unknown-unknown --release
```

## Testing Contracts

Run tests for all contracts:
```bash
cd contracts
cargo test
```

Run tests for a specific contract:
```bash
cd contracts/example
cargo test
```

## Deploying Contracts

### Deploy to Testnet

1. Configure Soroban CLI for testnet:
   ```bash
   soroban config network add --global testnet \
     --rpc-url https://soroban-testnet.stellar.org:443 \
     --network-passphrase "Test SDF Network ; September 2015"
   ```

2. Create an identity:
   ```bash
   soroban config identity generate --global alice
   ```

3. Fund your account:
   ```bash
   soroban config identity fund alice --network testnet
   ```

4. Deploy the contract:
   ```bash
   soroban contract deploy \
     --wasm target/wasm32-unknown-unknown/release/example.wasm \
     --source alice \
     --network testnet
   ```

## Adding New Contracts

To add a new contract to the workspace:

1. Create a new directory under `contracts/`:
   ```bash
   mkdir contracts/my-contract
   ```

2. Initialize the contract:
   ```bash
   cd contracts/my-contract
   cargo init --lib
   ```

3. Update `contracts/my-contract/Cargo.toml`:
   ```toml
   [package]
   name = "my-contract"
   version = "0.1.0"
   edition = "2021"

   [lib]
   crate-type = ["cdylib"]

   [dependencies]
   soroban-sdk = "21.0.0"

   [dev-dependencies]
   soroban-sdk = { version = "21.0.0", features = ["testutils"] }

   [profile.release]
   opt-level = "z"
   overflow-checks = true
   debug = 0
   strip = "symbols"
   debug-assertions = false
   panic = "abort"
   codegen-units = 1
   lto = true

   [profile.release-with-logs]
   inherits = "release"
   debug-assertions = true
   ```

4. Add the new contract to the workspace in `contracts/Cargo.toml`:
   ```toml
   members = [
       "example",
       "my-contract",  # Add this line
   ]
   ```

## Contract Development Guidelines

### For Contributors

1. **One Contract Per Issue**: Each contract should address a specific use case or SEP implementation
2. **Testing Required**: All contracts must include comprehensive unit tests
3. **Documentation**: Add inline comments and update this README with contract-specific details
4. **Optimization**: Use the release profile for production builds to minimize WASM size

### Code Standards

- Follow Rust naming conventions (snake_case for functions, PascalCase for types)
- Use `soroban-sdk` macros (`#[contract]`, `#[contractimpl]`, etc.)
- Include error handling with custom error types
- Write integration tests using `soroban-sdk` testutils

## Common Commands Reference

| Command | Description |
|---------|-------------|
| `cargo build --target wasm32-unknown-unknown --release` | Build optimized WASM |
| `cargo test` | Run all tests |
| `soroban contract optimize --wasm <path>` | Optimize WASM size |
| `soroban contract invoke --id <CONTRACT_ID> -- <function>` | Invoke contract function |
| `soroban contract inspect --wasm <path>` | Inspect contract interface |

## Resources

- [Soroban Documentation](https://soroban.stellar.org/docs)
- [Soroban Examples](https://github.com/stellar/soroban-examples)
- [Stellar SDK Rust Docs](https://docs.rs/soroban-sdk/latest/soroban_sdk/)
- [Soroban CLI Reference](https://soroban.stellar.org/docs/reference/soroban-cli)
