use crate::error::Error;
use futures::FutureExt;
use sp_core::{crypto::Ss58Codec, sr25519::Pair, Pair as PairTraits};
use std::str::FromStr;

use super::prelude::Password;

/// represent a user keypair and its infos.
#[derive(Clone)]

pub struct Keypair {
    password_hash: Password,
    phrase: MnemonicPhrase,
    pub_key: PublicKey,
    keypair: Pair,
}

impl Keypair {
    pub fn new(
        password_hash: Password,
        phrase: MnemonicPhrase,
        pub_key: PublicKey,
        keypair: Pair,
    ) -> Self {
        Self {
            password_hash,
            phrase,
            pub_key,
            keypair,
        }
    }

    pub fn password_hash(&self) -> &Password {
        &self.password_hash
    }

    pub fn phrase(&self) -> &MnemonicPhrase {
        &self.phrase
    }

    pub fn pub_key(&self) -> &PublicKey {
        &self.pub_key
    }

    pub fn keypair(&self) -> &Pair {
        &self.keypair
    }
}
#[derive(thiserror::Error, Debug)]
pub enum KeypairGenerationError {
    #[error("{0}")]
    Pubkey(String),

    #[error("fail to generate mnemonic phrase with {0}")]
    MnemonicPhrase(String),

    #[error("{0}")]
    SecretKey(String),

    #[error("{0}")]
    Other(String)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublicKey(pub(crate) String);

impl FromStr for PublicKey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        subxt::utils::AccountId32::from_str(s)
            .map(|v| PublicKey(v.to_string()))
            .map_err(|e| KeypairGenerationError::Pubkey(e.to_string()).into())
    }
}

impl From<Keypair> for PublicKey {
    fn from(value: Keypair) -> Self {
        value.pub_key().clone()
    }
}

impl From<Pair> for PublicKey {
    fn from(value: Pair) -> Self {
        let value = value.public().to_ss58check();
        PublicKey(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MnemonicPhrase(pub(crate) String);

impl MnemonicPhrase {
    pub fn new(phrase: &str, password: Password) -> Result<Self, Error> {
        let vrf = Pair::from_phrase(phrase, password.as_pwd());

        match vrf {
            Ok(_) => Ok(Self(String::from(phrase))),
            Err(e) => Err(KeypairGenerationError::MnemonicPhrase(String::from(phrase)))?,
        }
    }
}

#[derive(Clone)]
pub struct SecretKey(pub(crate) Pair);

impl FromStr for SecretKey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        schnorrkel::Keypair::from_bytes(s.as_bytes())
            .map(|v| SecretKey(Pair::from(v)))
            .map_err(|e| KeypairGenerationError::SecretKey(e.to_string()).into())
    }
}

impl From<Keypair> for SecretKey {
    fn from(value: Keypair) -> Self {
        SecretKey(value.keypair().clone())
    }
}

impl From<SecretKey> for Pair {
    fn from(val: SecretKey) -> Self {
        val.0
    }
}

impl From<Pair> for SecretKey {
    fn from(value: Pair) -> Self {
        SecretKey(value)
    }
}
