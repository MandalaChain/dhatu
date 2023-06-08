use crate::error::Error;
use futures::FutureExt;
use sp_core::sr25519::Pair;
use std::str::FromStr;

/// represent a user keypair and its infos.
#[derive(Clone)]

pub struct Keypair {
    password_hash: String,
    phrase: String,
    pub_key: String,
    keypair: Pair,
}

impl Keypair {
    pub(crate) fn new(
        password_hash: String,
        phrase: String,
        pub_key: String,
        keypair: Pair,
    ) -> Self {
        Self {
            password_hash,
            phrase,
            pub_key,
            keypair,
        }
    }

    pub fn password_hash(&self) -> &str {
        self.password_hash.as_ref()
    }

    pub fn phrase(&self) -> &str {
        self.phrase.as_ref()
    }

    pub fn pub_key(&self) -> &str {
        self.pub_key.as_ref()
    }

    pub fn keypair(&self) -> &Pair {
        &self.keypair
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublicKey(pub(crate) String);

impl FromStr for PublicKey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        subxt::utils::AccountId32::from_str(s)
            .map(|v| PublicKey(v.to_string()))
            .map_err(|e| Error::KeypairGenError(e.to_string()))
    }
}

impl From<Keypair> for PublicKey {
    fn from(value: Keypair) -> Self {
        PublicKey(String::from(value.pub_key()))
    }
}

#[derive(Clone)]
pub struct SecretKey(pub(crate) Pair);

impl FromStr for SecretKey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        schnorrkel::Keypair::from_bytes(s.as_bytes())
            .map(|v| SecretKey(Pair::from(v)))
            .map_err(|e| Error::KeypairGenError(e.to_string()))
    }
}

impl From<Keypair> for SecretKey {
    fn from(value: Keypair) -> Self {
        SecretKey(value.keypair().clone())
    }
}
