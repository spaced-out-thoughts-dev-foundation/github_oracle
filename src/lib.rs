#![no_std]
use crate::extensions::env_extensions::EnvExtensions;
use soroban_sdk::{
    contract, contractimpl, log, panic_with_error, Address, BytesN, Env, String, Vec,
};
use types::config_data::ConfigData;
use types::error::Error;

mod extensions;
mod test;
mod types;

#[contract]
pub struct GithubOracleContract;

#[contractimpl]
impl GithubOracleContract {
    pub fn config(e: Env, config: ConfigData) {
        log!(&e, "config: {:?}", config);
        config.admin.require_auth();
        if e.is_initialized() {
            e.panic_with_error(Error::AlreadyInitialized);
        }
        e.set_admin(&config.admin);

        Self::__add_repos(&e, config.repos);
    }

    pub fn version(_e: Env) -> u32 {
        env!("CARGO_PKG_VERSION")
            .split(".")
            .next()
            .unwrap()
            .parse::<u32>()
            .unwrap()
    }

    pub fn admin(e: Env) -> Option<Address> {
        e.get_admin()
    }

    pub fn add_repos(e: Env, repos: Vec<String>) {
        e.panic_if_not_admin();
        Self::__add_repos(&e, repos);
    }

    pub fn remove_repos(e: Env, repos: Vec<String>) {
        e.panic_if_not_admin();
        Self::__remove_repos(&e, repos);
    }

    pub fn get_repos(e: Env) -> Vec<String> {
        e.get_repos()
    }

    pub fn update_contract(e: Env, wasm_hash: BytesN<32>) {
        e.panic_if_not_admin();
        e.deployer().update_current_contract_wasm(wasm_hash)
    }

    fn __add_repos(e: &Env, repos: Vec<String>) {
        let mut current_repos = e.get_repos();
        for repo in repos.iter() {
            //check if the asset has been already added
            if e.get_repo_index(&repo).is_some() {
                panic_with_error!(&e, Error::RepoAlreadyExists);
            }
            e.set_repo_index(&repo, current_repos.len());
            current_repos.push_back(repo);
        }
        if current_repos.len() >= 256 {
            panic_with_error!(&e, Error::RepoLimitExceeded);
        }
        e.set_repos(current_repos);
    }

    fn __remove_repos(e: &Env, repos: Vec<String>) {
        let mut current_repos = e.get_repos();
        for repo in repos.iter() {
            let index = e.get_repo_index(&repo);
            if index.is_none() {
                panic_with_error!(&e, Error::RepoMissing);
            }
            let index = index.unwrap();

            current_repos.remove(index as u32);
            e.remove_repo_index(&repo);
        }
        e.set_repos(current_repos);
    }
}
