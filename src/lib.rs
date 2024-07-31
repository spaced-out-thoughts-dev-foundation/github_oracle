#![no_std]
use crate::extensions::env_extensions::EnvExtensions;
use soroban_sdk::{
    contract, contractimpl, log, panic_with_error, Address, BytesN, Env, Map, String, Vec,
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

    pub fn add_issues(e: Env, repo_name: String, issues: Vec<String>) {
        e.panic_if_not_admin();
        Self::__add_issues(&e, repo_name, issues);
    }

    pub fn clear_repos(e: Env) {
        e.panic_if_not_admin();
        e.set_repos(Map::new(&e));
    }

    pub fn remove_repos(e: Env, repos: Vec<String>) {
        e.panic_if_not_admin();
        Self::__remove_repos(&e, repos);
    }

    pub fn remove_issues(e: Env, repo_name: String, issues: Vec<String>) {
        e.panic_if_not_admin();
        Self::__remove_issues(&e, repo_name, issues);
    }

    pub fn get_repos(e: Env) -> Vec<String> {
        e.get_repos().keys()
    }

    pub fn get_repos_and_issues(e: Env) -> Map<String, Map<String, String>> {
        e.get_repos()
    }

    pub fn get_issues_for_repo(e: Env, repo_name: String) -> Map<String, String> {
        let issues = e.get_repos().get(repo_name);

        if issues.is_none() {
            panic_with_error!(&e, Error::RepoMissing);
        }

        issues.unwrap()
    }

    pub fn update_contract(e: Env, wasm_hash: BytesN<32>) {
        e.panic_if_not_admin();
        e.deployer().update_current_contract_wasm(wasm_hash)
    }

    fn __add_repos(e: &Env, repos: Vec<String>) {
        let mut current_repos = e.get_repos();
        for repo in repos.iter() {
            //check if the asset has been already added
            if current_repos.contains_key(repo.clone()) {
                panic_with_error!(&e, Error::RepoAlreadyExists);
            }

            current_repos.set(repo.clone(), Map::new(e));
        }
        if current_repos.len() >= 256 {
            panic_with_error!(&e, Error::RepoLimitExceeded);
        }
        e.set_repos(current_repos);
    }

    fn __add_issues(e: &Env, repo: String, issues: Vec<String>) {
        let mut current_repos = e.get_repos();
        if !current_repos.contains_key(repo.clone()) {
            panic_with_error!(&e, Error::RepoMissing);
        }

        let mut current_issues = current_repos.get(repo.clone()).unwrap();
        for issue in issues.iter() {
            //check if the issue has been already added
            if current_issues.contains_key(issue.clone()) {
                panic_with_error!(&e, Error::IssueAlreadyExists);
            }

            current_issues.set(issue.clone(), String::from_str(e, "unclaimed"));
        }
        if current_issues.len() >= 512 {
            panic_with_error!(&e, Error::IssueLimitExceeded);
        }
        current_repos.set(repo.clone(), current_issues);
        e.set_repos(current_repos);
    }

    fn __remove_repos(e: &Env, repos: Vec<String>) {
        let mut current_repos = e.get_repos();
        for repo in repos.iter() {
            if !current_repos.contains_key(repo.clone()) {
                panic_with_error!(&e, Error::RepoMissing);
            }

            current_repos.remove(repo.clone());
        }
        e.set_repos(current_repos);
    }

    fn __remove_issues(e: &Env, repo: String, issues: Vec<String>) {
        let mut current_repos = e.get_repos();
        if !current_repos.contains_key(repo.clone()) {
            panic_with_error!(&e, Error::RepoMissing);
        }

        let mut current_issues = current_repos.get(repo.clone()).unwrap();
        for issue in issues.iter() {
            if !current_issues.contains_key(issue.clone()) {
                panic_with_error!(&e, Error::IssueMissing);
            }

            current_issues.remove(issue.clone());
        }
        current_repos.set(repo.clone(), current_issues);
        e.set_repos(current_repos);
    }
}
