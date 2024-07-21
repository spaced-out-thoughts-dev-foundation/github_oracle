#![no_std]
use soroban_sdk::{contract, contractimpl, log, Env, String, Symbol};

#[contract]
pub struct GithubOracleContract;

#[contractimpl]
impl GithubOracleContract {
    pub fn fetch(env: Env, key: Symbol) -> Option<String> {
        let value: Option<String> = env.storage().instance().get(&key);

        // instance itself and all entries in storage().instance(), i.e, COUNTER.
        env.storage().instance().extend_ttl(50, 100);

        return value;
    }

    pub fn try_set_value(env: Env, key: Symbol, value: String) -> bool {
        if let Some(v) = Self::fetch(env.clone(), key.clone()) {
            log!(&env, "key found with value: {}", v);
            return false;
        }

        env.storage().instance().set(&key, &value);
        return true;
    }

    pub fn override_value(env: Env, key: Symbol, value: String) {
        env.storage().instance().set(&key, &value);
    }
}

mod test;
