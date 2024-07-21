#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, vec, Env, String, Symbol};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GithubOracleContract);
    let client = GithubOracleContractClient::new(&env, &contract_id);

    let key = symbol_short!("key");
    let original_value = String::from_str(&env.clone(), "value");
    let new_value = String::from_str(&env.clone(), "new_value");

    assert_eq!(client.fetch(&key.clone()), None);
    assert_eq!(
        client.try_set_value(&key.clone(), &original_value.clone()),
        true
    );
    assert_eq!(
        client.try_set_value(&key.clone(), &original_value.clone()),
        false
    );
    assert_eq!(client.fetch(&key.clone()), Some(original_value.clone()));
    assert_eq!(client.override_value(&key.clone(), &new_value.clone()), ());
    assert_eq!(client.fetch(&key.clone()), Some(new_value.clone()));
}
