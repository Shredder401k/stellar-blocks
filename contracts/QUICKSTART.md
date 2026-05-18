# Quick Start Guide for Soroban Contracts

This guide will get you from zero to deployed contract in under 10 minutes.

## 1. Initial Setup (One-time)

Run the setup script to install all prerequisites:

```bash
cd contracts
chmod +x setup.sh
./setup.sh
```

This installs:
- Rust toolchain
- wasm32-unknown-unknown target
- Soroban CLI
- Configures testnet
- Creates and funds a default identity

## 2. Test the Example Contract

```bash
cd example

# Build the contract
cargo build --target wasm32-unknown-unknown --release

# Run tests
cargo test

# Deploy to testnet
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/example.wasm \
  --source default \
  --network testnet
```

Save the contract ID from the deploy output!

## 3. Interact with Your Contract

Initialize the contract:
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source default \
  --network testnet \
  -- initialize --owner "alice"
```

Increment the counter:
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source default \
  --network testnet \
  -- increment --amount 5
```

Get the current value:
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --network testnet \
  -- get_counter
```

## 4. Create Your Own Contract

```bash
# From the contracts directory
mkdir my-contract
cd my-contract
cargo init --lib
```

Copy the `Cargo.toml` from the example contract and update the package name:
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
```

Add your contract to the workspace in `contracts/Cargo.toml`:
```toml
[workspace]
resolver = "2"
members = [
    "example",
    "my-contract",  # Add this line
]
```

## 5. Basic Contract Template

Create `src/lib.rs`:

```rust
#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct MyContract;

#[contractimpl]
impl MyContract {
    pub fn hello(env: Env, to: String) -> String {
        format!("Hello, {}!", to)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{Env, String};

    #[test]
    fn test_hello() {
        let env = Env::default();
        let contract_id = env.register_contract(None, MyContract);
        let client = MyContractClient::new(&env, &contract_id);

        let result = client.hello(&String::from_str(&env, "World"));
        assert_eq!(result, String::from_str(&env, "Hello, World!"));
    }
}
```

## 6. Build and Test

```bash
# Build
cargo build --target wasm32-unknown-unknown --release

# Test
cargo test

# Deploy
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/my_contract.wasm \
  --source default \
  --network testnet
```

## Common Commands Cheat Sheet

| Task | Command |
|------|---------|
| Build contract | `cargo build --target wasm32-unknown-unknown --release` |
| Run tests | `cargo test` |
| Run tests with output | `cargo test -- --nocapture` |
| Deploy to testnet | `soroban contract deploy --wasm <path> --source default --network testnet` |
| Invoke function | `soroban contract invoke --id <ID> --source default --network testnet -- <function> --arg value` |
| Inspect contract | `soroban contract inspect --wasm <path>` |
| Optimize WASM | `soroban contract optimize --wasm <path>` |
| Fund account | `soroban config identity fund <identity> --network testnet` |
| List identities | `soroban config identity ls` |

## Troubleshooting

### "error: linker `rust-lld` not found"
```bash
rustup update
rustup target add wasm32-unknown-unknown
```

### "error: failed to get `soroban-sdk`"
```bash
cargo clean
cargo update
```

### "Contract not found" when invoking
Make sure you're using the correct contract ID from the deploy output and that you're on the right network.

### Tests failing with "register_contract" errors
Ensure you have the testutils feature enabled in dev-dependencies:
```toml
[dev-dependencies]
soroban-sdk = { version = "21.0.0", features = ["testutils"] }
```

## Next Steps

- Read the full documentation in `contracts/README.md`
- Review the example contract for patterns
- Check `contracts/CONTRIBUTING.md` for contribution guidelines
- Browse [Soroban Examples](https://github.com/stellar/soroban-examples) for more patterns

## Resources

- [Soroban Docs](https://soroban.stellar.org/docs)
- [Soroban SDK Reference](https://docs.rs/soroban-sdk/latest/soroban_sdk/)
- [Stellar Quest](https://quest.stellar.org/) - Interactive tutorials
- [Soroban Discord](https://discord.gg/stellar) - Community support
