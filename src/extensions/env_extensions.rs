use soroban_sdk::storage::Instance;
use soroban_sdk::{panic_with_error, Address, Env, Map, String};

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
    fn get_repos(&self) -> Map<String, Map<String, String>>;
    fn set_repos(&self, repos: Map<String, Map<String, String>>);
}

impl EnvExtensions for Env {
    // admin
    fn get_admin(&self) -> Option<Address> {
        get_instance_storage(self).get(&ADMIN_KEY)
    }

    // initialization
    fn set_admin(&self, admin: &Address) {
        get_instance_storage(self).set(&ADMIN_KEY, admin);
    }
    fn is_initialized(&self) -> bool {
        get_instance_storage(self).has(&ADMIN_KEY)
    }

    // authorization
    fn panic_if_not_admin(&self) {
        let admin = self.get_admin();
        if admin.is_none() {
            panic_with_error!(self, Error::Unauthorized);
        }
        admin.unwrap().require_auth()
    }

    // repos
    fn get_repos(&self) -> Map<String, Map<String, String>> {
        get_instance_storage(self)
            .get(&REPOS)
            .unwrap_or_else(|| Map::new(self))
    }
    fn set_repos(&self, repos: Map<String, Map<String, String>>) {
        get_instance_storage(self).set(&REPOS, &repos);
    }
}

fn get_instance_storage(e: &Env) -> Instance {
    e.storage().instance()
}
