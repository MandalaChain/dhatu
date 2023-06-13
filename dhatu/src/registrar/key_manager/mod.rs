//! module responsible for all related task regarding user keypair. e.g creating, recovering, etc.
pub mod keypair;
pub mod password;

use std::str::FromStr;

use sp_core::{sr25519::Pair as Keys, Pair};

pub mod prelude {
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

