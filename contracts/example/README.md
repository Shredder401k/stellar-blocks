# Example Contract

A minimal Soroban smart contract demonstrating basic patterns and best practices for the stellar-blocks project.

## Overview

This contract implements a simple counter with the following features:
- Contract initialization with owner tracking
- Increment and decrement operations
- State persistence using Soroban storage
- Custom error handling
- Event emission for state changes
- Comprehensive test coverage

## Contract Interface

### Functions

#### `initialize(owner: String) -> Result<(), Error>`
Initializes the contract with an owner. Can only be called once.

**Parameters:**
- `owner`: String identifier for the contract owner

**Errors:**
- `Error::AlreadyInitialized`: Contract has already been initialized

**Events:**
- Publishes `init` event with owner

#### `increment(amount: i64) -> Result<i64, Error>`
Increments the counter by the specified amount.

**Parameters:**
- `amount`: The value to add to the counter

**Returns:**
- The new counter value

**Errors:**
- `Error::NotInitialized`: Contract has not been initialized

**Events:**
- Publishes `incr` event with amount

#### `decrement(amount: i64) -> Result<i64, Error>`
Decrements the counter by the specified amount.

**Parameters:**
- `amount`: The value to subtract from the counter

**Returns:**
- The new counter value

**Errors:**
- `Error::NotInitialized`: Contract has not been initialized

**Events:**
- Publishes `decr` event with amount

#### `get_counter() -> Result<i64, Error>`
Returns the current counter value.

**Returns:**
- The current counter value

**Errors:**
- `Error::NotInitialized`: Contract has not been initialized

#### `get_owner() -> Result<String, Error>`
Returns the contract owner.

**Returns:**
- The owner's identifier

**Errors:**
- `Error::NotInitialized`: Contract has not been initialized

## Building

```bash
cd contracts/example
cargo build --target wasm32-unknown-unknown --release
```

The compiled WASM will be at: `target/wasm32-unknown-unknown/release/example.wasm`

## Testing

Run all tests:
```bash
cargo test
```

Run tests with output:
```bash
cargo test -- --nocapture
```

## Deploying

### Testnet Deployment

1. Build the optimized contract:
   ```bash
   cargo build --target wasm32-unknown-unknown --release
   ```

2. Deploy using Soroban CLI:
   ```bash
   soroban contract deploy \
     --wasm target/wasm32-unknown-unknown/release/example.wasm \
     --source alice \
     --network testnet
   ```

3. Initialize the contract:
   ```bash
   soroban contract invoke \
     --id <CONTRACT_ID> \
     --source alice \
     --network testnet \
     -- initialize --owner "alice"
   ```

## Usage Examples

### Increment the counter
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  -- increment --amount 5
```

### Get current value
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --network testnet \
  -- get_counter
```

## Key Patterns Demonstrated

1. **Initialization Pattern**: One-time setup with owner tracking
2. **Storage Management**: Using instance storage for persistent state
3. **Error Handling**: Custom error types with Result returns
4. **Event Emission**: Publishing events for state changes
5. **Test Coverage**: Comprehensive unit tests including error cases

## For Contributors

This contract serves as a template for new contracts in the stellar-blocks project. When creating a new contract:

1. Copy this structure as a starting point
2. Modify the contract logic for your specific use case
3. Update tests to cover your new functionality
4. Document the interface in the contract's README
5. Ensure all tests pass before submitting a PR

## Resources

- [Soroban Documentation](https://soroban.stellar.org/docs)
- [Soroban SDK Docs](https://docs.rs/soroban-sdk/latest/soroban_sdk/)
- [Example Contracts](https://github.com/stellar/soroban-examples)
