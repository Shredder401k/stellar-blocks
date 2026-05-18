# Contract Template Guide

Use this as a reference when creating new contracts for the stellar-blocks project.

## Directory Structure

```
contracts/my-contract/
├── Cargo.toml
├── README.md
└── src/
    ├── lib.rs
    └── test.rs
```

## Cargo.toml Template

```toml
[package]
name = "my-contract"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
description = "Brief description of what this contract does"

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

## lib.rs Template

```rust
#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, Symbol, Address};

/// Storage keys for the contract
#[contracttype]
pub enum DataKey {
    // Define your storage keys here
    Owner,
    // Add more as needed
}

/// Custom error types
#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    Unauthorized = 3,
    // Add more error types as needed
}

/// [Brief description of your contract]
/// 
/// [Detailed description of what this contract does,
/// its purpose, and how it fits into the stellar-blocks ecosystem]
#[contract]
pub struct MyContract;

#[contractimpl]
impl MyContract {
    /// Initialize the contract
    /// 
    /// # Arguments
    /// * `env` - The contract environment
    /// * `owner` - The address of the contract owner
    /// 
    /// # Errors
    /// Returns `Error::AlreadyInitialized` if already initialized
    pub fn initialize(env: Env, owner: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Owner) {
            return Err(Error::AlreadyInitialized);
        }

        env.storage().instance().set(&DataKey::Owner, &owner);

        env.events()
            .publish((symbol_short!("init"),), owner);

        Ok(())
    }

    /// [Function description]
    /// 
    /// # Arguments
    /// * `env` - The contract environment
    /// * `param` - [Parameter description]
    /// 
    /// # Returns
    /// [Return value description]
    /// 
    /// # Errors
    /// [Error conditions]
    pub fn my_function(env: Env, param: i64) -> Result<i64, Error> {
        // Verify initialization
        if !env.storage().instance().has(&DataKey::Owner) {
            return Err(Error::NotInitialized);
        }

        // Your logic here

        // Emit event
        env.events()
            .publish((symbol_short!("event"),), param);

        Ok(param)
    }

    /// Get the contract owner
    pub fn get_owner(env: Env) -> Result<Address, Error> {
        env.storage()
            .instance()
            .get(&DataKey::Owner)
            .ok_or(Error::NotInitialized)
    }
}

mod test;
```

## test.rs Template

```rust
#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register_contract(None, MyContract);
    let client = MyContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    
    // Initialize should succeed
    client.initialize(&owner);

    // Verify owner is set
    assert_eq!(client.get_owner(), owner);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn test_initialize_twice_fails() {
    let env = Env::default();
    let contract_id = env.register_contract(None, MyContract);
    let client = MyContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    
    client.initialize(&owner);
    // Second initialization should panic
    client.initialize(&owner);
}

#[test]
fn test_my_function() {
    let env = Env::default();
    let contract_id = env.register_contract(None, MyContract);
    let client = MyContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    client.initialize(&owner);

    // Test your function
    let result = client.my_function(&42);
    assert_eq!(result, 42);
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn test_function_without_initialize_fails() {
    let env = Env::default();
    let contract_id = env.register_contract(None, MyContract);
    let client = MyContractClient::new(&env, &contract_id);

    // Should panic because contract is not initialized
    client.my_function(&42);
}

#[test]
fn test_events() {
    let env = Env::default();
    let contract_id = env.register_contract(None, MyContract);
    let client = MyContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    client.initialize(&owner);
    client.my_function(&42);

    // Verify events were published
    let events = env.events().all();
    assert!(events.len() >= 2); // init + function event
}

// Add more tests for:
// - Edge cases
// - Error conditions
// - Complex workflows
// - Authorization checks
```

## README.md Template

```markdown
# [Contract Name]

[Brief description of what this contract does]

## Overview

[Detailed description of the contract's purpose, use cases, and how it fits into the stellar-blocks ecosystem]

## Contract Interface

### Functions

#### `initialize(owner: Address) -> Result<(), Error>`
[Function description]

**Parameters:**
- `owner`: [Parameter description]

**Errors:**
- `Error::AlreadyInitialized`: [Error description]

**Events:**
- Publishes `init` event with owner

#### `my_function(param: i64) -> Result<i64, Error>`
[Function description]

**Parameters:**
- `param`: [Parameter description]

**Returns:**
- [Return value description]

**Errors:**
- `Error::NotInitialized`: [Error description]

**Events:**
- Publishes `event` event with param

[Continue for all public functions...]

## Building

```bash
cd contracts/my-contract
cargo build --target wasm32-unknown-unknown --release
```

## Testing

```bash
cargo test
```

## Deploying

### Testnet

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/my_contract.wasm \
  --source default \
  --network testnet
```

### Initialize

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source default \
  --network testnet \
  -- initialize --owner <OWNER_ADDRESS>
```

## Usage Examples

### [Example 1]
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source default \
  --network testnet \
  -- my_function --param 42
```

## Integration with stellar-blocks

[Explain how this contract integrates with the frontend components in the stellar-blocks library]

## Security Considerations

[List any security considerations, access control mechanisms, or important notes for users]

## Contributing

See `contracts/CONTRIBUTING.md` for guidelines on contributing to this contract.

## License

MIT
```

## Checklist for New Contracts

Before submitting a PR for a new contract:

- [ ] Contract follows the template structure
- [ ] All public functions have documentation comments
- [ ] Custom error types are defined and used
- [ ] Storage keys use enums
- [ ] Events are emitted for state changes
- [ ] Comprehensive tests cover all functions
- [ ] Error cases have tests with `#[should_panic]`
- [ ] README documents the complete interface
- [ ] Contract builds without warnings
- [ ] All tests pass
- [ ] Contract has been deployed to testnet
- [ ] Contract added to workspace in `contracts/Cargo.toml`
- [ ] PR links to the issue and includes testnet contract ID

## Common Patterns

### Access Control
```rust
fn require_owner(env: &Env, caller: &Address) -> Result<(), Error> {
    let owner: Address = env.storage()
        .instance()
        .get(&DataKey::Owner)
        .ok_or(Error::NotInitialized)?;
    
    if caller != &owner {
        return Err(Error::Unauthorized);
    }
    
    Ok(())
}
```

### Pausable Pattern
```rust
#[contracttype]
pub enum DataKey {
    Paused,
    // ...
}

pub fn pause(env: Env, caller: Address) -> Result<(), Error> {
    require_owner(&env, &caller)?;
    env.storage().instance().set(&DataKey::Paused, &true);
    Ok(())
}

fn require_not_paused(env: &Env) -> Result<(), Error> {
    let paused: bool = env.storage()
        .instance()
        .get(&DataKey::Paused)
        .unwrap_or(false);
    
    if paused {
        return Err(Error::ContractPaused);
    }
    
    Ok(())
}
```

### Counter Pattern
```rust
fn increment_counter(env: &Env, key: &DataKey) -> u64 {
    let current: u64 = env.storage()
        .instance()
        .get(key)
        .unwrap_or(0);
    
    let new_value = current + 1;
    env.storage().instance().set(key, &new_value);
    new_value
}
```

## Resources

- [Soroban Documentation](https://soroban.stellar.org/docs)
- [Soroban Examples](https://github.com/stellar/soroban-examples)
- [Soroban SDK Docs](https://docs.rs/soroban-sdk/latest/soroban_sdk/)
