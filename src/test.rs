#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, vec, Address, Env, String, Vec};

fn setup_contract<'a>() -> (Env, GithubOracleContractClient<'a>, ConfigData) {
    let env = Env::default();

    let admin = Address::generate(&env);

    let contract_id = &Address::from_string(&String::from_str(
        &env,
        "CDXHQTB7FGRMWTLJJLNI3XPKVC6SZDB5SFGZUYDPEGQQNC4G6CKE4QRC",
    ));

    env.register_contract(contract_id, GithubOracleContract);

    let client = GithubOracleContractClient::new(&env, &contract_id);

    env.budget().reset_unlimited();
    env.mock_all_auths();

    let original_repos: Vec<String> = vec![&env];

    let config = ConfigData {
        admin: admin.clone(),
        repos: original_repos.clone(),
    };

    client.config(&config);

    let repos = client.get_repos();
    assert_eq!(repos, original_repos);

    (env, client, config)
}

fn soroban_cli_repo<'a>(env: &Env) -> String {
    String::from_str(&env, "sdf/soroban-cli")
}

fn digicus_repo<'a>(env: &Env) -> String {
    String::from_str(&env, "spaced-out-thoughts-dev-foundation/digicus")
}

fn soroban_sdk_repo<'a>(env: &Env) -> String {
    String::from_str(&env, "sdf/soroban-sdk")
}

#[test]
fn adds_one_repo() {
    let (env, client, _config) = setup_contract();

    let new_repos: Vec<String> = vec![&env, soroban_sdk_repo(&env)];
    client.add_repos(&new_repos.clone());

    let repos = client.get_repos();

    assert_eq!(repos, new_repos);
}

#[test]
fn adds_two_repos() {
    let (env, client, _config) = setup_contract();

    let new_repos: Vec<String> = vec![&env, soroban_sdk_repo(&env), digicus_repo(&env)];
    client.add_repos(&new_repos.clone());

    let repos = client.get_repos();

    assert_eq!(repos, new_repos);
}

#[test]
fn adds_repos_twice() {
    let (env, client, _config) = setup_contract();

    let first_repos_to_add: Vec<String> = vec![&env, soroban_sdk_repo(&env)];
    client.add_repos(&first_repos_to_add.clone());

    let repos = client.get_repos();
    assert_eq!(repos, first_repos_to_add);

    let second_repos_to_add: Vec<String> = vec![&env, soroban_cli_repo(&env), digicus_repo(&env)];
    client.add_repos(&second_repos_to_add.clone());

    let repos = client.get_repos();
    let expected_repos = vec![
        &env,
        soroban_sdk_repo(&env),
        soroban_cli_repo(&env),
        digicus_repo(&env),
    ];
    assert_eq!(repos, expected_repos);
}

#[test]
#[should_panic]
fn panics_when_adding_existing_repo() {
    let (env, client, _config) = setup_contract();

    let repos_to_add: Vec<String> = vec![&env, soroban_sdk_repo(&env)];
    client.add_repos(&repos_to_add.clone());

    let repos = client.get_repos();
    assert_eq!(repos, repos_to_add);

    client.add_repos(&repos_to_add.clone());
}

#[test]
fn adds_then_removes_repo() {
    let (env, client, _config) = setup_contract();

    let repos_to_add: Vec<String> = vec![&env, soroban_sdk_repo(&env)];
    client.add_repos(&repos_to_add.clone());

    let repos = client.get_repos();
    assert_eq!(repos, repos_to_add);

    let repos_to_remove = vec![&env];
    client.remove_repos(&repos_to_remove.clone());

    let repos = client.get_repos();
    assert_eq!(repos, vec![&env, soroban_sdk_repo(&env)]);
}

#[test]
#[should_panic]
fn attempt_to_remove_repo_that_does_not_exist() {
    let (env, client, _config) = setup_contract();

    let repos_to_remove: Vec<String> = vec![&env, soroban_sdk_repo(&env)];
    client.add_repos(&repos_to_remove.clone());

    let repos = client.get_repos();
    assert_eq!(repos, repos_to_remove);

    let repos_to_remove = vec![&env, digicus_repo(&env)];
    client.remove_repos(&repos_to_remove);
}
