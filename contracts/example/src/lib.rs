#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, contracterror, symbol_short, Env, String};

/// Storage keys for the contract
#[contracttype]
pub enum DataKey {
    Counter,
    Owner,
}

/// Custom error types for the contract
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotInitialized = 1,
    AlreadyInitialized = 2,
}

/// Example Soroban smart contract
/// 
/// This is a minimal starter contract that demonstrates:
/// - Contract initialization
/// - State management
/// - Basic operations (increment/decrement)
/// - Error handling
/// - Events
#[contract]
pub struct ExampleContract;

#[contractimpl]
impl ExampleContract {
    /// Initialize the contract with an owner
    /// 
    /// # Arguments
    /// * `env` - The contract environment
    /// * `owner` - The address of the contract owner
    /// 
    /// # Errors
    /// Returns `Error::AlreadyInitialized` if contract is already initialized
    pub fn initialize(env: Env, owner: String) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Owner) {
            return Err(Error::AlreadyInitialized);
        }

        env.storage().instance().set(&DataKey::Owner, &owner);
        env.storage().instance().set(&DataKey::Counter, &0i64);

        env.events()
            .publish((symbol_short!("init"),), owner);

        Ok(())
    }

    /// Increment the counter by a specified amount
    /// 
    /// # Arguments
    /// * `env` - The contract environment
    /// * `amount` - The amount to increment by
    /// 
    /// # Returns
    /// The new counter value
    /// 
    /// # Errors
    /// Returns `Error::NotInitialized` if contract is not initialized
    pub fn increment(env: Env, amount: i64) -> Result<i64, Error> {
        if !env.storage().instance().has(&DataKey::Counter) {
            return Err(Error::NotInitialized);
        }

        let mut counter: i64 = env.storage().instance().get(&DataKey::Counter).unwrap();
        counter += amount;
        env.storage().instance().set(&DataKey::Counter, &counter);

        env.events()
            .publish((symbol_short!("incr"),), amount);

        Ok(counter)
    }

    /// Decrement the counter by a specified amount
    /// 
    /// # Arguments
    /// * `env` - The contract environment
    /// * `amount` - The amount to decrement by
    /// 
    /// # Returns
    /// The new counter value
    /// 
    /// # Errors
    /// Returns `Error::NotInitialized` if contract is not initialized
    pub fn decrement(env: Env, amount: i64) -> Result<i64, Error> {
        if !env.storage().instance().has(&DataKey::Counter) {
            return Err(Error::NotInitialized);
        }

        let mut counter: i64 = env.storage().instance().get(&DataKey::Counter).unwrap();
        counter -= amount;
        env.storage().instance().set(&DataKey::Counter, &counter);

        env.events()
            .publish((symbol_short!("decr"),), amount);

        Ok(counter)
    }

    /// Get the current counter value
    /// 
    /// # Arguments
    /// * `env` - The contract environment
    /// 
    /// # Returns
    /// The current counter value
    /// 
    /// # Errors
    /// Returns `Error::NotInitialized` if contract is not initialized
    pub fn get_counter(env: Env) -> Result<i64, Error> {
        env.storage()
            .instance()
            .get(&DataKey::Counter)
            .ok_or(Error::NotInitialized)
    }

    /// Get the contract owner
    /// 
    /// # Arguments
    /// * `env` - The contract environment
    /// 
    /// # Returns
    /// The owner's address as a String
    /// 
    /// # Errors
    /// Returns `Error::NotInitialized` if contract is not initialized
    pub fn get_owner(env: Env) -> Result<String, Error> {
        env.storage()
            .instance()
            .get(&DataKey::Owner)
            .ok_or(Error::NotInitialized)
    }
}

mod test;
