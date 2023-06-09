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
    pub_key: PublicAddress,
    keypair: Pair,
}

impl Keypair {
    pub(crate) fn new(
        password_hash: Password,
        phrase: MnemonicPhrase,
        pub_key: PublicAddress,
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

    pub fn pub_key(&self) -> &PublicAddress {
        &self.pub_key
    }

    pub fn keypair(&self) -> &Pair {
        &self.keypair
    }
}
#[derive(thiserror::Error, Debug)]
pub enum KeypairGenerationError {
    #[error("{0}")]
    PublicAddress(String),

    #[error("fail to generate mnemonic phrase with {0}")]
    MnemonicPhrase(String),

    #[error("{0}")]
    PrivateKey(String),

    #[error("{0}")]
    Recover(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublicAddress(pub(crate) String);

impl FromStr for PublicAddress {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        subxt::utils::AccountId32::from_str(s)
            .map(|v| Self(v.to_string()))
            .map_err(|e| KeypairGenerationError::PublicAddress(e.to_string()).into())
    }
}

impl From<Keypair> for PublicAddress {
    fn from(value: Keypair) -> Self {
        value.pub_key().clone()
    }
}

impl From<Pair> for PublicAddress {
    fn from(value: Pair) -> Self {
        let value = value.public().to_ss58check();
        Self(value)
    }
}

impl From<PrivateKey> for PublicAddress {
    fn from(value: PrivateKey) -> Self {
        let address = value.0.public().to_ss58check();
        Self(address)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MnemonicPhrase(pub(crate) String);

impl MnemonicPhrase {
    pub fn new(phrase: &str, password: Option<Password>) -> Result<Self, Error> {
        let vrf = match password {
            Some(password) => Pair::from_phrase(phrase, Some(password.as_str())),
            None => Pair::from_phrase(phrase, None),
        };

        match vrf {
            Ok(_) => Ok(Self(String::from(phrase))),
            Err(e) => Err(KeypairGenerationError::MnemonicPhrase(String::from(phrase)))?,
        }
    }
}

#[derive(Clone)]
pub struct PrivateKey(pub(crate) Pair);

impl PrivateKey {
    pub fn public_key(&self) -> PublicAddress {
        self.clone().into()
    }
}

impl FromStr for PrivateKey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        schnorrkel::Keypair::from_bytes(s.as_bytes())
            .map(|v| PrivateKey(Pair::from(v)))
            .map_err(|e| KeypairGenerationError::PrivateKey(e.to_string()).into())
    }
}

impl From<Keypair> for PrivateKey {
    fn from(value: Keypair) -> Self {
        PrivateKey(value.keypair().clone())
    }
}

impl From<PrivateKey> for Pair {
    fn from(val: PrivateKey) -> Self {
        val.0
    }
}

impl From<Pair> for PrivateKey {
    fn from(value: Pair) -> Self {
        PrivateKey(value)
    }
}
