/// keypair modules contains stuff related to keypair manipulation, use, etc.
pub mod keypair;
/// password modules used to generate keypair.
pub mod password;

use std::str::FromStr;

use sp_core::{sr25519::Pair as Keys, Pair};

pub(crate) mod prelude {
    pub use super::keypair::*;
    pub use super::password::*;
}

use prelude::*;

use crate::error::{Error, KeypairGenerationError};

/// represent a keypair manager. used to create a new schnorrkel keypair or recover from a phrase.
pub struct KeyManager;

impl KeyManager {
    /// create a new keypair from a random generated password.
    pub fn new_default() -> Keypair {
        let password = Password::new();
        Self::gen(Some(password))
    }

    /// create a new keypair without password.
    pub fn new_without_password() -> Keypair {
        Self::gen(None)
    }

    /// recover a keypair from a phrase and a password, will fail if the phrase and password is invalid.
    pub fn recover(pass: Option<&str>, phrase: &str) -> Result<Keypair, Error> {
        let password = match pass {
            Some(pass) => Some(Password::from_str(pass)?),
            None => None,
        };
        Self::gen_from_phrase(password, phrase)
            .map_err(|e| KeypairGenerationError::Recover(e.to_string()).into())
    }
}

impl KeyManager {
    /// internal function. meant to be used to create a new keypair.
    fn gen(password: Option<Password>) -> Keypair {
        let (keypair, phrase, _) = match password.clone() {
            Some(password) => Keys::generate_with_phrase(password.as_pwd()),
            None => Keys::generate_with_phrase(None),
        };

        Self::construct(password, phrase, keypair)
    }

    /// internal function. meant to be used to recover a keypair from a password and its phrase.
    fn gen_from_phrase(
        password: Option<Password>,
        phrase: &str,
    ) -> Result<Keypair, Box<dyn std::error::Error>> {
        let (keypair, _) = match password.clone() {
            Some(pass) => Keys::from_phrase(phrase, pass.as_pwd())?,
            None => Keys::from_phrase(phrase, None)?,
        };

        let keypair = Self::construct(password, String::from(phrase), keypair);
        Ok(keypair)
    }

    /// construct a keypair from a password, phrase and a keypair.
    /// internal function. meant to be used to create a new keypair or recover a keypair.
    /// should not be exposed to user.
    fn construct(password: Option<Password>, phrase: String, keypair: Keys) -> Keypair {
        let phrase = MnemonicPhrase::new(&phrase, password.clone())
            .expect("internal function should not fail!");
        let pub_key = keypair.clone().into();

        Keypair::new(password, phrase, pub_key, keypair)
    }
}
