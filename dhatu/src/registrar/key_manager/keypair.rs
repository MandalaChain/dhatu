use crate::error::{Error, KeypairGenerationError};
use futures::FutureExt;
use sp_core::{crypto::Ss58Codec, sr25519::Pair, Pair as PairTraits};
use std::str::FromStr;
use subxt::utils::AccountId32;

use super::prelude::Password;

/// represent a keypair.
#[derive(Clone)]
pub struct Keypair {
    /// password hash used to generate this keypair. if any.
    password_hash: Option<Password>,
    /// mnemonic phrase used to generate this keypair.
    phrase: MnemonicPhrase,
    /// public address of this keypair.
    pub_key: PublicAddress,
    /// the actual keypair.
    keypair: Pair,
}

impl Keypair {
    /// create a new keypair. it should not be exposed to user.
    /// meant to be used only to create and recover keypair.
    pub(crate) fn new(
        password_hash: Option<Password>,
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

    /// get the password hash used to generate this keypair.
    pub fn password_hash(&self) -> Option<&Password> {
        self.password_hash.as_ref()
    }

    /// get the mnemonic phrase used to generate this keypair.
    pub fn phrase(&self) -> &MnemonicPhrase {
        &self.phrase
    }

    /// get the public address of this keypair.
    pub fn pub_key(&self) -> &PublicAddress {
        &self.pub_key
    }

    #[cfg(feature = "unstable_sp_core")]
    /// get the actual keypair. this is only enabled on `unstable_sp_core` feature.
    /// due to the unstable nature of the `sp_core` crate.
    pub fn keypair(&self) -> &Pair {
        &self.keypair
    }
}

/// public address representation of some keypair.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublicAddress(pub(crate) String);

impl PublicAddress {
    /// get the string representation of this public address.
    pub fn inner(&self) -> &str {
        self.0.as_str()
    }

    #[cfg(feature = "subxt")]
    /// convert this public address to subxt `AccountId32`.
    /// only available if `subxt` or `unstable` feature flag is enabled.
    pub fn inner_as_subxt_account_id(&self) -> AccountId32 {
        AccountId32::from_str(self.inner())
            .expect("converstion from valid public address shouldn't fail!")
    }

    #[cfg(feature = "subxt")]
    #[cfg(feature = "unstable_sp_core")]
    /// convert this public address to sp_core `AccountId32`.
    /// only available if `unstable` feature flag is enabled.
    pub fn inner_as_sp_core_acoount_id(&self) -> sp_core::crypto::AccountId32 {
        sp_core::crypto::AccountId32::from_str(self.inner())
            .expect("converstion from valid public address shouldn't fail!")
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

impl ToString for PublicAddress {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

/// mnemonic phrase representation for used to recover a keypair.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MnemonicPhrase(pub(crate) String);

impl MnemonicPhrase {
    /// internal function. should not be exposed to user.
    /// meant to be used only to create and recover keypair.
    pub(crate) fn new(phrase: &str, password: Option<Password>) -> Result<Self, Error> {
        let vrf = match password {
            Some(password) => Pair::from_phrase(phrase, Some(password.as_str())),
            None => Pair::from_phrase(phrase, None),
        };

        match vrf {
            Ok(_) => Ok(Self(String::from(phrase))),
            Err(_e) => Err(KeypairGenerationError::MnemonicPhrase(String::from(phrase)))?,
        }
    }

    /// get the string representation of this mnemonic phrase.
    pub fn inner(&self) -> &str {
        self.0.as_str()
    }
}

impl From<Keypair> for MnemonicPhrase {
    fn from(value: Keypair) -> Self {
        value.phrase
    }
}

impl ToString for MnemonicPhrase {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

/// private or secret key representation of some keypair.
#[derive(Clone)]
pub struct PrivateKey(pub(crate) Pair);

impl PrivateKey {
    /// sign a message using this private key.
    /// note that this returns a raw bytes array.
    pub fn sign(&self, message: &[u8]) -> [u8; 64] {
        self.0.sign(message).0
    }

    /// get the public address of this private key.
    pub fn public_address(&self) -> PublicAddress {
        self.clone().into()
    }

    #[cfg(feature = "unstable_sp_core")]
    /// get the actual keypair. this is only enabled on `unstable_sp_core` feature.
    pub fn inner(&self) -> &Pair {
        &self.0
    }
}

impl FromStr for PrivateKey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pair = Pair::from_string(s, None).map_err(|e| KeypairGenerationError::PrivateKey(e))?;
        Ok(Self(pair))
    }
}

impl From<Keypair> for PrivateKey {
    fn from(value: Keypair) -> Self {
        PrivateKey(value.keypair.clone())
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
