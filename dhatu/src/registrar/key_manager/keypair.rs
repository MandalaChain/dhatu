use crate::error::{Error, KeypairGenerationError};
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

    #[cfg(feature = "unstable_sp_core")]
    pub fn keypair(&self) -> &Pair {
        &self.keypair
    }
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

    pub fn inner(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Clone)]
pub struct PrivateKey(pub(crate) Pair);

impl PrivateKey {
    pub fn public_key(&self) -> PublicAddress {
        self.clone().into()
    }

    pub fn inner(&self) -> &Pair {
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
        assert_eq!(keypair.keypair.public(), pair.public());
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
        let keypair = sr25519::Pair::from_string("endorse doctor arch helmet master dragon wild favorite property mercy vault maze", None).unwrap();
        let public_address: PublicAddress = keypair.clone().into();
        assert_eq!(public_address, PublicAddress::from_str(keypair.public().to_ss58check().as_str()).unwrap());
    }

    #[test]
    fn test_from_private_key_to_public_address() {
        let keypair = sr25519::Pair::from_string("endorse doctor arch helmet master dragon wild favorite property mercy vault maze", None).unwrap();
        let private_key = PrivateKey { 0: keypair };

        let public_address: PublicAddress = private_key.clone().into();
        assert_eq!(public_address, PublicAddress::from_str(private_key.inner().public().to_ss58check().as_str()).unwrap());
    }
}

#[cfg(test)]
mod mnemonic_phrase_tests {
    use super::*;

    #[test]
    fn test_new_with_valid_phrase() {
        let phrase = "endorse doctor arch helmet master dragon wild favorite property mercy vault maze";
        let password = Some(Password::new());
        let result = MnemonicPhrase::new(phrase, password);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().inner(), phrase);
    }

    #[test]
    fn test_new_with_invalid_phrase() {
        let phrase = "sample tornado pen frog valley library velvet figure guitar powder mirror churn";
        let password = Some(Password::new());
        let result = MnemonicPhrase::new(phrase, password);
        assert!(result.is_err());
    }

    #[test]
    fn test_new_without_password() {
        let phrase = "endorse doctor arch helmet master dragon wild favorite property mercy vault maze";
        let result = MnemonicPhrase::new(phrase, None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().inner(), phrase);
    }

    #[test]
    fn test_inner() {
        let phrase = "endorse doctor arch helmet master dragon wild favorite property mercy vault maze";
        let mnemonic = MnemonicPhrase::new(phrase, None).unwrap();
        assert_eq!(mnemonic.inner(), phrase);
    }
}

#[cfg(test)]
mod private_key_tests {
    use crate::registrar::key_manager::KeyManager;
    use schnorrkel::Keypair;

    use std::str;
    use base64;

    use super::*;

    #[test]
    fn test_public_key() {
        let pair = KeyManager::new_default().keypair;
        let private_key = PrivateKey(pair.clone());
        let public_key = private_key.public_key();
        assert_eq!(public_key, pair.into());
    }

    #[test]
    fn test_inner() {
        let private_key = PrivateKey(KeyManager::new_default().keypair);
        let inner = private_key.inner();
        assert_eq!(inner.to_raw_vec(), private_key.0.to_raw_vec());
    }

    #[test]
    fn test_from_str_valid() {
        
        // let mini_secret_key: MiniSecretKey = MiniSecretKey::generate();
        // let secret_key: SecretKey = mini_secret_key.expand(MiniSecretKey::ED25519_MODE);
        // let secret_key_bytes: [u8; 64] = secret_key.to_bytes();

        // panic!("secret_key: {:?}", SchnorrkelKeypair::generate().secret);

        // let valid_key_str = key_bytes.to_bytes().iter().map(|&c| c as char).collect::<String>();

        // let valid_key_str = str::from_utf8(&key_bytes).unwrap();
        // let valid_key_str = String::from_utf8(key_bytes[0..32].to_vec()).unwrap();

        // let valid_key_str = base64::encode(&secret_key_bytes);

        // let keypair_bytes = hex!("28b0ae221c6bb06856b287f60d7ea0d98552ea5a16db16956849aa371db3eb51fd190cce74df356432b410bd64682309d6dedb27c76845daf388557cbac3ca3446ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a");
        // let keypair: Keypair = Keypair::from_bytes(&keypair_bytes[..]).unwrap();
        
        let bytes = Keypair::generate().secret.to_bytes();
        // let vec = bytes.to_vec();
        // panic!("secret_key: {:?}", );

        // Convert bytes to string
        let encoded_string = base64::encode(&bytes);

        let as_bytes = encoded_string.as_bytes();

        let str = str::from_utf8(as_bytes).unwrap();

        // Convert string back to bytes
        let decoded_bytes = base64::decode(&encoded_string).unwrap();

        let sl = decoded_bytes.as_slice();

        // Ensure the decoded bytes match the original bytes
        assert_eq!(bytes, decoded_bytes.as_slice());

        // panic!("Encoded string: {}", encoded_string);
        println!("Decoded bytes: {:?}", decoded_bytes);


        // println!("valid_key_str: {valid_key_str}");

        let result = PrivateKey::from_str(str).unwrap();
        // assert!(result.is_ok());

        /*
        // Generate a new random Schnorrkel keypair
        let keypair = SchnorrkelKeypair::generate();

        // Extract the private key from the keypair
        let private_key = keypair.secret.to_bytes().to_vec();

        panic!("Private Key: {:?}", base64::encode(&private_key));
        */
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