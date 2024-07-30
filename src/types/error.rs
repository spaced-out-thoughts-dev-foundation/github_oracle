use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    AlreadyInitialized = 0,
    Unauthorized = 1,
    RepoMissing = 2,
    RepoAlreadyExists = 3,
    RepoLimitExceeded = 7,
}
