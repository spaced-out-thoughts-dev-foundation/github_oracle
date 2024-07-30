use soroban_sdk::storage::Instance;
use soroban_sdk::{panic_with_error, Address, Env, String, Vec};

use crate::types::error::Error;

// constants
const REPOS: &str = "repos";
const ADMIN_KEY: &str = "admin";

pub trait EnvExtensions {
    // admin
    fn get_admin(&self) -> Option<Address>;
    fn set_admin(&self, admin: &Address);

    // initialization
    fn is_initialized(&self) -> bool;

    // authorization
    fn panic_if_not_admin(&self);

    // repos
    fn get_repos(&self) -> Vec<String>;
    fn set_repos(&self, repos: Vec<String>);

    // repo index
    fn get_repo_index(&self, repo_name: &String) -> Option<u8>;
    fn remove_repo_index(&self, repo_name: &String);
    fn set_repo_index(&self, repo_name: &String, index: u32);
}
impl EnvExtensions for Env {
    fn get_admin(&self) -> Option<Address> {
        get_instance_storage(self).get(&ADMIN_KEY)
    }

    fn set_admin(&self, admin: &Address) {
        get_instance_storage(self).set(&ADMIN_KEY, admin);
    }

    fn is_initialized(&self) -> bool {
        get_instance_storage(self).has(&ADMIN_KEY)
    }

    fn panic_if_not_admin(&self) {
        let admin = self.get_admin();
        if admin.is_none() {
            panic_with_error!(self, Error::Unauthorized);
        }
        admin.unwrap().require_auth()
    }

    fn get_repos(&self) -> Vec<String> {
        get_instance_storage(self)
            .get(&REPOS)
            .unwrap_or_else(|| Vec::new(self))
    }

    fn set_repos(&self, repos: Vec<String>) {
        get_instance_storage(self).set(&REPOS, &repos);
    }

    fn get_repo_index(&self, repo_name: &String) -> Option<u8> {
        let index: Option<u32> = get_instance_storage(self).get(repo_name);
        if index.is_none() {
            return None;
        }
        return Some(index.unwrap() as u8);
    }

    fn remove_repo_index(&self, repo_name: &String) {
        get_instance_storage(self).remove(repo_name);
    }

    fn set_repo_index(&self, repo_name: &String, index: u32) {
        get_instance_storage(self).set(repo_name, &index);
    }
}

fn get_instance_storage(e: &Env) -> Instance {
    e.storage().instance()
}
