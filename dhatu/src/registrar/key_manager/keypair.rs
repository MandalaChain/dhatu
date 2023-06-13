use crate::error::Error;
use futures::FutureExt;
use sp_core::{crypto::Ss58Codec, sr25519::Pair, Pair as PairTraits};
use std::str::FromStr;
use subxt::utils::AccountId32;

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

impl PublicAddress {
    pub fn inner(&self) -> &str {
        self.0.as_str()
    }
}

impl FromStr for PublicAddress {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        AccountId32::from_str(s)
            .map(|v| Self(v.to_string()))
            .map_err(|e| KeypairGenerationError::PublicAddress(e.to_string()).into())
    }
}

impl From<PublicAddress> for AccountId32 {
    fn from(value: PublicAddress) -> Self {
        Self::from_str(&value.0).expect("converstion from valid public address shouldn't fail!")
    }
}

impl From<PublicAddress> for subxt::dynamic::Value {
    fn from(value: PublicAddress) -> Self {
        let acc = AccountId32::from(value);
        Self::from_bytes(acc)
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
            Err(_e) => Err(KeypairGenerationError::MnemonicPhrase(String::from(phrase)))?,
        }
    }

    pub fn inner(&self) -> &str{
        self.0.as_str()
    }
}

#[derive(Clone)]
pub struct PrivateKey(pub(crate) Pair);

impl PrivateKey {
    pub fn public_key(&self) -> PublicAddress {
        self.clone().into()
    }

    pub fn inner(&self) -> &Pair{
        &self.0
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

#[cfg(test)]
mod keypair_tests {
    use std::str::FromStr;

    use sp_core::{sr25519, Pair};
    use super::*;

    #[test]
    fn test_keypair_new() {
        let password_hash = Password::new();
        let phrase = MnemonicPhrase::new("endorse doctor arch helmet master dragon wild favorite property mercy vault maze", None).unwrap();
        let pub_key = PublicAddress::from_str("5DJk1gegyQJk6BNs7LceZ1akt5e9fpm4gUYGzcfpKaLG9Mmb").unwrap();
        let pair = sr25519::Pair::from_string("endorse doctor arch helmet master dragon wild favorite property mercy vault maze", None).unwrap();

        let keypair = Keypair::new(password_hash.clone(), phrase.clone(), pub_key.clone(), pair.clone());

        assert_eq!(keypair.password_hash().as_str(), password_hash.as_str());
        assert_eq!(*keypair.phrase(), phrase);
        assert_eq!(*keypair.pub_key(), pub_key);
        assert_eq!(keypair.keypair().public(), pair.public());
    }
}

#[cfg(test)]
mod keypair_generation_error_tests {
    use super::*;

    #[test]
    fn test_keypair_generation_error_public_address() {
        let err = KeypairGenerationError::PublicAddress("Invalid address format".to_string());
        assert_eq!(format!("{}", err), "Invalid address format");
    }

    #[test]
    fn test_keypair_generation_error_mnemonic_phrase() {
        let err = KeypairGenerationError::MnemonicPhrase("Failed to generate mnemonic".to_string());
        assert_eq!(
            format!("{}", err),
            "fail to generate mnemonic phrase with Failed to generate mnemonic"
        );
    }

    #[test]
    fn test_keypair_generation_error_private_key() {
        let err = KeypairGenerationError::PrivateKey("Invalid private key".to_string());
        assert_eq!(format!("{}", err), "Invalid private key");
    }

    #[test]
    fn test_keypair_generation_error_recover() {
        let err = KeypairGenerationError::Recover("Failed to recover keypair".to_string());
        assert_eq!(format!("{}", err), "Failed to recover keypair");
    }
}
