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

    assert_repo_names(repos, new_repos);
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

    assert_repo_names(repos, expected_repos);
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

#[test]
fn adds_then_clears_repos() {
    let (env, client, _config) = setup_contract();

    let repos_to_add: Vec<String> = vec![&env, soroban_sdk_repo(&env)];
    client.add_repos(&repos_to_add.clone());

    let repos = client.get_repos();
    assert_eq!(repos, repos_to_add);

    client.clear_repos();

    let repos = client.get_repos();
    assert_eq!(repos, vec![&env]);
}

#[test]
fn adds_then_clears_repo_with_issues() {
    let (env, client, _config) = setup_contract();

    let repos_to_add: Vec<String> = vec![&env, soroban_sdk_repo(&env)];
    client.add_repos(&repos_to_add.clone());

    let issues_to_add: Vec<String> = vec![
        &env,
        String::from_str(&env, "issue-1"),
        String::from_str(&env, "issue-2"),
        String::from_str(&env, "issue-3"),
    ];
    client.add_issues(&soroban_sdk_repo(&env), &issues_to_add.clone());

    let repos = client.get_repos();
    assert_eq!(repos, repos_to_add);

    client.clear_repos();

    let repos = client.get_repos();
    assert_eq!(repos, vec![&env]);
}

#[test]
fn get_issues_empty() {
    let (env, client, _config) = setup_contract();

    let repos_to_add: Vec<String> = vec![&env, soroban_sdk_repo(&env)];
    client.add_repos(&repos_to_add.clone());

    let issues = client.get_issues_for_repo(&soroban_sdk_repo(&env));
    assert_eq!(issues, Map::new(&env));
}

#[test]
fn add_single_issue() {
    let (env, client, _config) = setup_contract();

    let repos_to_add: Vec<String> = vec![&env, soroban_sdk_repo(&env)];
    client.add_repos(&repos_to_add.clone());

    let issues_to_add: Vec<String> = vec![&env, String::from_str(&env, "issue-1")];
    client.add_issues(&soroban_sdk_repo(&env), &issues_to_add.clone());

    let mut expected: Map<String, String> = Map::new(&env);
    expected.set(
        String::from_str(&env, "issue-1"),
        String::from_str(&env, "unclaimed"),
    );

    let issues = client.get_issues_for_repo(&soroban_sdk_repo(&env));
    assert_eq!(issues, expected);
}

#[test]
fn add_multiple_issues() {
    let (env, client, _config) = setup_contract();

    let repos_to_add: Vec<String> = vec![&env, soroban_sdk_repo(&env)];
    client.add_repos(&repos_to_add.clone());

    let issues_to_add: Vec<String> = vec![
        &env,
        String::from_str(&env, "issue-1"),
        String::from_str(&env, "issue-2"),
        String::from_str(&env, "issue-3"),
    ];
    client.add_issues(&soroban_sdk_repo(&env), &issues_to_add.clone());

    let mut expected: Map<String, String> = Map::new(&env);
    expected.set(
        String::from_str(&env, "issue-1"),
        String::from_str(&env, "unclaimed"),
    );
    expected.set(
        String::from_str(&env, "issue-2"),
        String::from_str(&env, "unclaimed"),
    );
    expected.set(
        String::from_str(&env, "issue-3"),
        String::from_str(&env, "unclaimed"),
    );

    let issues = client.get_issues_for_repo(&soroban_sdk_repo(&env));
    assert_eq!(issues, expected);
}

#[test]
fn add_issues_twice() {
    let (env, client, _config) = setup_contract();

    let repos_to_add: Vec<String> = vec![&env, soroban_sdk_repo(&env), digicus_repo(&env)];
    client.add_repos(&repos_to_add.clone());

    let issues_to_add: Vec<String> = vec![
        &env,
        String::from_str(&env, "issue-1"),
        String::from_str(&env, "issue-2"),
        String::from_str(&env, "issue-3"),
    ];
    client.add_issues(&soroban_sdk_repo(&env), &issues_to_add.clone());

    let mut expected: Map<String, String> = Map::new(&env);
    expected.set(
        String::from_str(&env, "issue-1"),
        String::from_str(&env, "unclaimed"),
    );
    expected.set(
        String::from_str(&env, "issue-2"),
        String::from_str(&env, "unclaimed"),
    );
    expected.set(
        String::from_str(&env, "issue-3"),
        String::from_str(&env, "unclaimed"),
    );

    let issues = client.get_issues_for_repo(&soroban_sdk_repo(&env));
    assert_eq!(issues, expected);

    let issues_to_add: Vec<String> = vec![
        &env,
        String::from_str(&env, "issue-4"),
        String::from_str(&env, "issue-5"),
    ];
    client.add_issues(&digicus_repo(&env), &issues_to_add.clone());

    let mut expected2: Map<String, String> = Map::new(&env);
    expected2.set(
        String::from_str(&env, "issue-4"),
        String::from_str(&env, "unclaimed"),
    );
    expected2.set(
        String::from_str(&env, "issue-5"),
        String::from_str(&env, "unclaimed"),
    );

    let issues = client.get_issues_for_repo(&digicus_repo(&env));
    assert_eq!(issues, expected2);

    let mut expected_all_repos_with_issues: Map<String, Map<String, String>> = Map::new(&env);
    expected_all_repos_with_issues.set(soroban_sdk_repo(&env), expected);
    expected_all_repos_with_issues.set(digicus_repo(&env), expected2);
    assert_eq!(
        client.get_repos_and_issues(),
        expected_all_repos_with_issues
    );
}

#[test]
#[should_panic]
fn add_issues_to_nonexistent_repo() {
    let (env, client, _config) = setup_contract();

    let repos_to_add: Vec<String> = vec![&env, soroban_sdk_repo(&env)];
    client.add_repos(&repos_to_add.clone());

    let issues_to_add: Vec<String> = vec![
        &env,
        String::from_str(&env, "issue-1"),
        String::from_str(&env, "issue-2"),
        String::from_str(&env, "issue-3"),
    ];
    client.add_issues(&digicus_repo(&env), &issues_to_add.clone());
}

#[test]
fn remove_single_issue() {
    let (env, client, _config) = setup_contract();

    let repos_to_add: Vec<String> = vec![&env, soroban_sdk_repo(&env)];
    client.add_repos(&repos_to_add.clone());

    let issues_to_add: Vec<String> = vec![&env, String::from_str(&env, "issue-1")];
    client.add_issues(&soroban_sdk_repo(&env), &issues_to_add.clone());

    let issues = client.get_issues_for_repo(&soroban_sdk_repo(&env));
    assert_eq!(issues.len(), 1);

    let issues_to_remove: Vec<String> = vec![&env, String::from_str(&env, "issue-1")];
    client.remove_issues(&soroban_sdk_repo(&env), &issues_to_remove.clone());

    let issues = client.get_issues_for_repo(&soroban_sdk_repo(&env));
    assert_eq!(issues.len(), 0);
}

#[test]
fn remove_multiple_issues() {
    let (env, client, _config) = setup_contract();

    let repos_to_add: Vec<String> = vec![&env, soroban_sdk_repo(&env)];
    client.add_repos(&repos_to_add.clone());

    let issues_to_add: Vec<String> = vec![
        &env,
        String::from_str(&env, "issue-1"),
        String::from_str(&env, "issue-2"),
        String::from_str(&env, "issue-3"),
    ];
    client.add_issues(&soroban_sdk_repo(&env), &issues_to_add.clone());

    let issues = client.get_issues_for_repo(&soroban_sdk_repo(&env));
    assert_eq!(issues.len(), 3);

    let issues_to_remove: Vec<String> = vec![
        &env,
        String::from_str(&env, "issue-1"),
        String::from_str(&env, "issue-3"),
    ];
    client.remove_issues(&soroban_sdk_repo(&env), &issues_to_remove.clone());

    let issues = client.get_issues_for_repo(&soroban_sdk_repo(&env));
    assert_eq!(issues.len(), 1);
}

#[test]
fn remove_issues_twice() {
    let (env, client, _config) = setup_contract();

    let repos_to_add: Vec<String> = vec![&env, soroban_sdk_repo(&env)];
    client.add_repos(&repos_to_add.clone());

    let issues_to_add: Vec<String> = vec![
        &env,
        String::from_str(&env, "issue-1"),
        String::from_str(&env, "issue-2"),
        String::from_str(&env, "issue-3"),
    ];
    client.add_issues(&soroban_sdk_repo(&env), &issues_to_add.clone());

    let issues = client.get_issues_for_repo(&soroban_sdk_repo(&env));
    assert_eq!(issues.len(), 3);

    let issues_to_remove: Vec<String> = vec![
        &env,
        String::from_str(&env, "issue-1"),
        String::from_str(&env, "issue-3"),
    ];
    client.remove_issues(&soroban_sdk_repo(&env), &issues_to_remove.clone());

    let issues = client.get_issues_for_repo(&soroban_sdk_repo(&env));
    assert_eq!(issues.len(), 1);

    let issues_to_remove: Vec<String> = vec![&env, String::from_str(&env, "issue-2")];
    client.remove_issues(&soroban_sdk_repo(&env), &issues_to_remove.clone());

    let issues = client.get_issues_for_repo(&soroban_sdk_repo(&env));
    assert_eq!(issues.len(), 0);
}

#[test]
#[should_panic]
fn remove_issues_from_nonexistent_repo() {
    let (env, client, _config) = setup_contract();

    let repos_to_add: Vec<String> = vec![&env, soroban_sdk_repo(&env)];
    client.add_repos(&repos_to_add.clone());

    let issues_to_add: Vec<String> = vec![
        &env,
        String::from_str(&env, "issue-1"),
        String::from_str(&env, "issue-2"),
        String::from_str(&env, "issue-3"),
    ];
    client.add_issues(&soroban_sdk_repo(&env), &issues_to_add.clone());

    let issues_to_remove: Vec<String> = vec![
        &env,
        String::from_str(&env, "issue-1"),
        String::from_str(&env, "issue-3"),
    ];
    client.remove_issues(&digicus_repo(&env), &issues_to_remove.clone());
}

#[test]
#[should_panic]
fn remove_issues_that_do_not_exist() {
    let (env, client, _config) = setup_contract();

    let repos_to_add: Vec<String> = vec![&env, soroban_sdk_repo(&env)];
    client.add_repos(&repos_to_add.clone());

    let issues_to_add: Vec<String> = vec![
        &env,
        String::from_str(&env, "issue-1"),
        String::from_str(&env, "issue-2"),
        String::from_str(&env, "issue-3"),
    ];
    client.add_issues(&soroban_sdk_repo(&env), &issues_to_add.clone());

    let issues_to_remove: Vec<String> = vec![
        &env,
        String::from_str(&env, "issue-1"),
        String::from_str(&env, "issue-4"),
    ];
    client.remove_issues(&soroban_sdk_repo(&env), &issues_to_remove.clone());
}

fn assert_repo_names(actual: Vec<String>, expected: Vec<String>) {
    assert_eq!(actual.len(), expected.len());

    for repo in actual {
        assert!(expected.contains(&repo));
    }
}
