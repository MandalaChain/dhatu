//! module responsible for all related task regarding user keypair. e.g creating, recovering, etc.
pub mod keypair;
pub mod password;

use std::str::FromStr;

use sp_core::{sr25519::Pair as Keys, Pair};

pub(crate) mod prelude {
    pub use super::keypair::*;
    pub use super::password::*;
}

use prelude::*;

use crate::{error::{Error, KeypairGenerationError}, };

/// represent a keypair manager.
pub struct KeyManager;

impl KeyManager {
    pub fn new_default() -> Keypair {
        let password = Password::new();
        Self::gen(password)
    }

    pub fn recover(pass: &str, phrase: &str) -> Result<Keypair, Error> {
        let password = Password::from_str(pass)?;
        Self::gen_from_phrase(password, phrase)
            .map_err(|e| KeypairGenerationError::Recover(e.to_string()).into())
    }
}

impl KeyManager {
    fn gen(password: Password) -> Keypair {
        let password_phrase = password.as_pwd();

        let (keypair, phrase, _) = Keys::generate_with_phrase(password_phrase);

        Self::construct(password, phrase, keypair)
    }

    fn gen_from_phrase(password: Password, phrase: &str) -> Result<Keypair, Box<dyn std::error::Error>> {
        let password_phrase = password.as_pwd();

        let (keys, _) = Keys::from_phrase(phrase, password_phrase)?;
        let keypair = Self::construct(password, String::from(phrase), keys);
        Ok(keypair)
    }

    fn construct(password: Password, phrase: String, keypair: Keys) -> Keypair {
        let phrase = MnemonicPhrase::new(&phrase, Some(password.clone()))
            .expect("internal function should not fail!");
        let pub_key = keypair.clone().into();

        Keypair::new(password, phrase, pub_key, keypair)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recover_valid() {
        let pass = "61c510ba04db830da8acd6595caf193d";
        let phrase = "endorse doctor arch helmet master dragon wild favorite property mercy vault maze";
        
        let keypair_result = KeyManager::recover(pass, phrase);
        assert!(keypair_result.is_ok());
        
        let keypair = keypair_result.unwrap();
        assert_eq!(keypair.password_hash().as_str(), pass);
        assert_eq!(keypair.phrase().inner(), phrase);
    }
    
    #[test]
    fn test_recover_invalid_phrase() {
        let pass = "483d968823979f7a937c65793ee91409";
        let phrase = "sample tornado pen frog valley library velvet figure guitar powder mirror churne";
        let keypair_result = KeyManager::recover(pass, phrase);
        assert!(keypair_result.is_err());
    }

    #[test]
    fn test_recover_invalid_pass() {
        let pass = "61457fa9cd845bb9";
        let phrase = "endorse doctor arch helmet master dragon wild favorite property mercy vault maze";
        let keypair_result = KeyManager::recover(pass, phrase);
        assert!(keypair_result.is_err());
    }
    
    #[test]
    fn test_gen_from_phrase() {
        let password = Password::new();
        let (_, phrase, _) = Keys::generate_with_phrase(password.as_pwd());
        
        let keypair_result = KeyManager::gen_from_phrase(password.clone(), phrase.as_str());
        assert!(keypair_result.is_ok());
        
        let keypair = keypair_result.unwrap();
        assert_eq!(keypair.password_hash().as_str(), password.as_str());
        assert_eq!(keypair.phrase().inner(), phrase);
    }
}
