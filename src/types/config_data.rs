use soroban_sdk::{contracttype, Address, String, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigData {
    pub admin: Address,
    pub repos: Vec<String>,
}
