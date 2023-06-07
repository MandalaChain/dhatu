/// trait representing a web2 user
pub trait User {
    fn password(&self) -> &str;
    fn email(&self) -> &str;
}

/// trait representing a pure web3 user.
pub trait Verifiable {
    fn phrase(&self) -> &str;
}

/// represent a complete web2 to web3 user mapping.
pub trait VerifiableUser: Verifiable + User {}
