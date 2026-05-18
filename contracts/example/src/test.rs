#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Events, Env, IntoVal, String};

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ExampleContract);
    let client = ExampleContractClient::new(&env, &contract_id);

    let owner = String::from_str(&env, "alice");
    
    // Initialize should succeed
    client.initialize(&owner);

    // Verify owner is set
    assert_eq!(client.get_owner(), owner);
    
    // Verify counter is initialized to 0
    assert_eq!(client.get_counter(), 0);

    // Verify initialization event was published
    let events = env.events().all();
    assert_eq!(events.len(), 1);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn test_initialize_twice_fails() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ExampleContract);
    let client = ExampleContractClient::new(&env, &contract_id);

    let owner = String::from_str(&env, "alice");
    
    client.initialize(&owner);
    // Second initialization should panic
    client.initialize(&owner);
}

#[test]
fn test_increment() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ExampleContract);
    let client = ExampleContractClient::new(&env, &contract_id);

    let owner = String::from_str(&env, "alice");
    client.initialize(&owner);

    // Increment by 5
    let result = client.increment(&5);
    assert_eq!(result, 5);

    // Increment by 3
    let result = client.increment(&3);
    assert_eq!(result, 8);

    // Verify counter value
    assert_eq!(client.get_counter(), 8);
}

#[test]
fn test_decrement() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ExampleContract);
    let client = ExampleContractClient::new(&env, &contract_id);

    let owner = String::from_str(&env, "alice");
    client.initialize(&owner);

    // Set counter to 10
    client.increment(&10);

    // Decrement by 3
    let result = client.decrement(&3);
    assert_eq!(result, 7);

    // Decrement by 2
    let result = client.decrement(&2);
    assert_eq!(result, 5);

    // Verify counter value
    assert_eq!(client.get_counter(), 5);
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn test_increment_without_initialize_fails() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ExampleContract);
    let client = ExampleContractClient::new(&env, &contract_id);

    // Should panic because contract is not initialized
    client.increment(&5);
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn test_get_counter_without_initialize_fails() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ExampleContract);
    let client = ExampleContractClient::new(&env, &contract_id);

    // Should panic because contract is not initialized
    client.get_counter();
}

#[test]
fn test_events() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ExampleContract);
    let client = ExampleContractClient::new(&env, &contract_id);

    let owner = String::from_str(&env, "alice");
    client.initialize(&owner);
    client.increment(&5);
    client.decrement(&2);

    // Verify events were published
    let events = env.events().all();
    assert_eq!(events.len(), 3); // init, incr, decr
}

#[test]
fn test_negative_counter() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ExampleContract);
    let client = ExampleContractClient::new(&env, &contract_id);

    let owner = String::from_str(&env, "alice");
    client.initialize(&owner);

    // Decrement below zero
    let result = client.decrement(&5);
    assert_eq!(result, -5);

    // Increment back to positive
    let result = client.increment(&10);
    assert_eq!(result, 5);
}
