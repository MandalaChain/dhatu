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

    /// can only be interpreted from 64 bytes secret seed hex string. see [here](sp_core::Pair::from_string_with_seed) for more details.
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

#[cfg(test)]
mod keypair_tests {
    use std::str::FromStr;

    use super::*;
    use sp_core::{sr25519, Pair};

    #[test]
    fn test_keypair_new() {
        let password_hash = Password::new();
        let phrase = MnemonicPhrase::new(
            "endorse doctor arch helmet master dragon wild favorite property mercy vault maze",
            None,
        )
        .unwrap();
        let pub_key =
            PublicAddress::from_str("5DJk1gegyQJk6BNs7LceZ1akt5e9fpm4gUYGzcfpKaLG9Mmb").unwrap();
        let pair = sr25519::Pair::from_string(
            "endorse doctor arch helmet master dragon wild favorite property mercy vault maze",
            None,
        )
        .unwrap();

        let keypair = Keypair::new(
            Some(password_hash.clone()),
            phrase.clone(),
            pub_key.clone(),
            pair.clone(),
        );

        assert_eq!(
            keypair.password_hash().unwrap().as_str(),
            password_hash.as_str()
        );
        assert_eq!(*keypair.phrase(), phrase);
        assert_eq!(*keypair.pub_key(), pub_key);
        assert_eq!(keypair.keypair.public(), pair.public());
    }
}

#[cfg(test)]
mod public_address_tests {
    use sp_core::sr25519;

    use crate::registrar::key_manager::KeyManager;

    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_inner() {
        let address = PublicAddress("test".to_string());
        assert_eq!(address.inner(), "test");
    }

    #[test]
    fn test_from_str_valid() {
        let address_str = "5DJk1gegyQJk6BNs7LceZ1akt5e9fpm4gUYGzcfpKaLG9Mmb";
        let result = PublicAddress::from_str(address_str);
        assert!(result.is_ok());
        let address = result.unwrap();
        assert_eq!(address.inner(), address_str);
    }

    #[test]
    #[should_panic]
    fn test_from_str_invalid() {
        let address_str = "AAAAC3NzaC1lZDI1NTE5AAAAIDmmRndO+3zhA/z6QAgNCR521OuIe5/8ojCkuo3U7ngi"; // ed25519
        let _result = PublicAddress::from_str(address_str).unwrap();
    }

    #[test]
    fn test_account_id32_from_public_address() {
        let address_str = "5DJk1gegyQJk6BNs7LceZ1akt5e9fpm4gUYGzcfpKaLG9Mmb";
        let address = PublicAddress::from_str(address_str).unwrap();
        let _account_id = AccountId32::from(address);
    }

    #[test]
    fn test_from_keypair_to_public_address() {
        let keypair = KeyManager::new_default();
        let public_address: PublicAddress = keypair.clone().into();
        assert_eq!(public_address, keypair.pub_key);
    }

    #[test]
    fn test_from_pair_to_public_address() {
        let keypair = sr25519::Pair::from_string(
            "endorse doctor arch helmet master dragon wild favorite property mercy vault maze",
            None,
        )
        .unwrap();
        let public_address: PublicAddress = keypair.clone().into();
        assert_eq!(
            public_address,
            PublicAddress::from_str(keypair.public().to_ss58check().as_str()).unwrap()
        );
    }

    #[test]
    fn test_from_private_key_to_public_address() {
        let keypair = sr25519::Pair::from_string(
            "endorse doctor arch helmet master dragon wild favorite property mercy vault maze",
            None,
        )
        .unwrap();
        let private_key = PrivateKey { 0: keypair };

        let public_address: PublicAddress = private_key.clone().into();
        assert_eq!(
            public_address,
            PublicAddress::from_str(private_key.0.public().to_ss58check().as_str()).unwrap()
        );
    }
}

#[cfg(test)]
mod mnemonic_phrase_tests {
    use super::*;

    #[test]
    fn test_new_with_valid_phrase() {
        let phrase =
            "endorse doctor arch helmet master dragon wild favorite property mercy vault maze";
        let password = Some(Password::new());
        let result = MnemonicPhrase::new(phrase, password);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().inner(), phrase);
    }

    #[test]
    fn test_new_with_invalid_phrase() {
        let phrase =
            "sample tornado pen frog valley library velvet figure guitar powder mirror churn";
        let password = Some(Password::new());
        let result = MnemonicPhrase::new(phrase, password);
        assert!(result.is_err());
    }

    #[test]
    fn test_new_without_password() {
        let phrase =
            "endorse doctor arch helmet master dragon wild favorite property mercy vault maze";
        let result = MnemonicPhrase::new(phrase, None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().inner(), phrase);
    }

    #[test]
    fn test_inner() {
        let phrase =
            "endorse doctor arch helmet master dragon wild favorite property mercy vault maze";
        let mnemonic = MnemonicPhrase::new(phrase, None).unwrap();
        assert_eq!(mnemonic.inner(), phrase);
    }
}

#[cfg(test)]
mod private_key_tests {
    use super::*;
    use crate::registrar::key_manager::KeyManager;

    #[test]
    fn test_public_address() {
        let pair = KeyManager::new_default().keypair;
        let private_key = PrivateKey(pair.clone());
        let public_key = private_key.public_address();
        assert_eq!(public_key, pair.into());
    }

    #[test]
    fn test_from_str_valid() {
        let mini_secret_key = "0xd5836897dc77e6c87e5cc268abaaa9c661bcf19aea9f0f50a1e149d21ce31eb7";
        let result = PrivateKey::from_str(mini_secret_key);
        assert!(result.is_ok());
    }

    #[test]
    fn test_from_str_invalid() {
        let invalid_private_key_str = "invalid_private_key";
        let result = PrivateKey::from_str(invalid_private_key_str);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_keypair() {
        let keypair = KeyManager::new_default();
        let private_key = PrivateKey::from(keypair.clone());
        assert_eq!(private_key.0.to_raw_vec(), keypair.keypair.to_raw_vec());
    }

    #[test]
    fn test_from_pair() {
        let pair = KeyManager::new_default().keypair;
        let private_key = PrivateKey::from(pair.clone());
        assert_eq!(private_key.0.to_raw_vec(), pair.to_raw_vec());
    }
}
