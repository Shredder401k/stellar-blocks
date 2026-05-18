# Contributing to Stellar Blocks Smart Contracts

Thank you for contributing to the stellar-blocks smart contract ecosystem! This guide will help you get started with developing Soroban contracts for the project.

## Getting Started

### Prerequisites

Before you begin, ensure you have:
- Rust (latest stable version)
- Soroban CLI
- wasm32-unknown-unknown target

Run the setup script to install all prerequisites:
```bash
cd contracts
chmod +x setup.sh
./setup.sh
```

### Development Workflow

1. **Claim an Issue**: Browse the issue tracker for contracts labeled with `Wave: Easy`, `Wave: Medium`, or `Wave: Advanced`. Comment to request assignment.

2. **Create Your Contract**: 
   ```bash
   cd contracts
   mkdir my-contract
   cd my-contract
   cargo init --lib
   ```

3. **Configure Cargo.toml**: Copy the configuration from `contracts/example/Cargo.toml` and update the package name.

4. **Add to Workspace**: Update `contracts/Cargo.toml` to include your contract in the `members` array.

5. **Implement**: Write your contract following the patterns in the example contract.

6. **Test**: Write comprehensive tests covering all functionality and edge cases.

7. **Document**: Create a README.md in your contract directory documenting the interface and usage.

## Code Standards

### Contract Structure

Every contract should follow this structure:
```
my-contract/
├── Cargo.toml
├── README.md
└── src/
    ├── lib.rs      # Main contract code
    └── test.rs     # Test module
```

### Rust Conventions

- Use `snake_case` for function names and variables
- Use `PascalCase` for types and enums
- Use `SCREAMING_SNAKE_CASE` for constants
- Add documentation comments (`///`) for all public functions
- Include examples in documentation where helpful

### Contract Patterns

1. **Initialization**: Implement an `initialize` function for one-time setup
2. **Error Handling**: Define custom error types using `#[contracttype]` enums
3. **Storage Keys**: Use enums for storage keys to avoid collisions
4. **Events**: Emit events for significant state changes
5. **Access Control**: Implement owner/admin checks where needed

### Testing Requirements

All contracts must include:
- ✅ Unit tests for all public functions
- ✅ Error case testing (use `#[should_panic]`)
- ✅ Edge case coverage (boundary values, empty inputs, etc.)
- ✅ Event verification tests
- ✅ Integration tests for complex workflows

Example test structure:
```rust
#[test]
fn test_happy_path() {
    // Test normal operation
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn test_error_case() {
    // Test error handling
}

#[test]
fn test_edge_case() {
    // Test boundary conditions
}
```

## Building and Testing

### Build Your Contract
```bash
cd contracts/my-contract
cargo build --target wasm32-unknown-unknown --release
```

### Run Tests
```bash
cargo test
```

### Run Tests with Output
```bash
cargo test -- --nocapture
```

### Optimize WASM Size
```bash
soroban contract optimize --wasm target/wasm32-unknown-unknown/release/my_contract.wasm
```

## Deployment Testing

Before submitting your PR, test deployment to testnet:

1. **Build the contract**:
   ```bash
   cargo build --target wasm32-unknown-unknown --release
   ```

2. **Deploy to testnet**:
   ```bash
   soroban contract deploy \
     --wasm target/wasm32-unknown-unknown/release/my_contract.wasm \
     --source default \
     --network testnet
   ```

3. **Test contract functions**:
   ```bash
   soroban contract invoke \
     --id <CONTRACT_ID> \
     --source default \
     --network testnet \
     -- function_name --arg1 value1
   ```

## Documentation Requirements

Each contract must include a README.md with:

1. **Overview**: Brief description of the contract's purpose
2. **Interface**: List all public functions with parameters and return types
3. **Usage Examples**: CLI commands showing how to interact with the contract
4. **Building Instructions**: How to build and test
5. **Deployment Guide**: Steps to deploy to testnet/mainnet

## Pull Request Checklist

Before submitting your PR, ensure:

- [ ] Contract builds without warnings
- [ ] All tests pass (`cargo test`)
- [ ] Code follows Rust conventions and project patterns
- [ ] Custom error types are defined and used
- [ ] Events are emitted for state changes
- [ ] README.md documents the contract interface
- [ ] Contract has been deployed and tested on testnet
- [ ] PR description links to the issue and includes contract ID from testnet

## Common Patterns

### Storage Management
```rust
#[contracttype]
pub enum DataKey {
    Counter,
    Owner,
}

// Set value
env.storage().instance().set(&DataKey::Counter, &value);

// Get value
let value: i64 = env.storage().instance().get(&DataKey::Counter).unwrap();

// Check existence
if env.storage().instance().has(&DataKey::Counter) {
    // ...
}
```

### Error Handling
```rust
#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    NotInitialized = 1,
    Unauthorized = 2,
    InvalidInput = 3,
}

pub fn my_function(env: Env) -> Result<i64, Error> {
    if !is_valid() {
        return Err(Error::InvalidInput);
    }
    Ok(42)
}
```

### Event Emission
```rust
use soroban_sdk::symbol_short;

env.events()
    .publish((symbol_short!("transfer"),), (from, to, amount));
```

## Getting Help

- Check the [Soroban Documentation](https://soroban.stellar.org/docs)
- Review the example contract in `contracts/example/`
- Ask questions in the project's discussion forum
- Reference [Soroban Examples](https://github.com/stellar/soroban-examples)

## Wave Program Points

Contract contributions are valued based on complexity:
- **Wave: Easy** (100-250 points): Simple contracts with basic CRUD operations
- **Wave: Medium** (250-500 points): Contracts with complex logic or multiple interactions
- **Wave: Advanced** (500-1000 points): Advanced contracts with cross-contract calls, complex state management, or SEP implementations

Points are awarded after PR review, merge, and testnet validation.

## Questions?

If you have questions about contract development or the contribution process, please:
1. Check the main CONTRIBUTING.md in the repository root
2. Review existing contracts for patterns
3. Open a discussion in the project's forum
4. Tag maintainers in your issue or PR

Happy coding! 🚀
