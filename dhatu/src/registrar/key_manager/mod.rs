/// keypair modules contains stuff related to keypair manipulation, use, etc.
pub mod keypair;
/// password modules used to generate keypair.
pub mod password;

use std::str::FromStr;

use subxt::ext::sp_core::{sr25519::Pair as Keys, Pair};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::PasswordGenerationError::InvalidLength;

    #[test]
    fn test_new_default() {
        let keypair = KeyManager::new_default();
        assert!(keypair.password_hash().is_some());
    }

    #[test]
    fn test_new_without_password() {
        let keypair = KeyManager::new_without_password();
        assert!(keypair.password_hash().is_none());
    }

    #[test]
    fn test_recover_valid() {
        let pass = "61c510ba04db830da8acd6595caf193d";
        let phrase =
            "endorse doctor arch helmet master dragon wild favorite property mercy vault maze";

        let keypair_result = KeyManager::recover(Some(pass), phrase);
        assert!(keypair_result.is_ok());

        let keypair = keypair_result.unwrap();
        assert_eq!(keypair.password_hash().unwrap().as_str(), pass);
        assert_eq!(keypair.phrase().inner(), phrase);
    }

    #[test]
    fn test_recover_invalid_phrase() {
        let pass = "483d968823979f7a937c65793ee91409";
        let phrase =
            "sample tornado pen frog valley library velvet figure guitar powder mirror churne";
        let keypair_result = KeyManager::recover(Some(pass), phrase);
        assert!(keypair_result.is_err());
        if let Err(err) = keypair_result {
            assert_eq!(
                format!("{:?}", err),
                format!(
                    "{:?}",
                    Error::Keypair(KeypairGenerationError::Recover(
                        "Invalid phrase".to_string()
                    ))
                )
            );
        }
    }

    #[test]
    fn test_recover_invalid_pass() {
        let pass = "61457fa9cd845bb9";
        let phrase =
            "endorse doctor arch helmet master dragon wild favorite property mercy vault maze";
        let keypair_result = KeyManager::recover(Some(pass), phrase);
        assert!(keypair_result.is_err());
        if let Err(err) = keypair_result {
            assert_eq!(
                format!("{:?}", err),
                format!("{:?}", Error::Password(InvalidLength))
            );
        }
    }

    #[test]
    fn test_gen_from_phrase() {
        let password = Password::new();
        let (_, phrase, _) = Keys::generate_with_phrase(password.as_pwd());

        let keypair_result = KeyManager::gen_from_phrase(Some(password.clone()), phrase.as_str());
        assert!(keypair_result.is_ok());

        let keypair = keypair_result.unwrap();
        assert_eq!(keypair.password_hash().unwrap().as_str(), password.as_str());
        assert_eq!(keypair.phrase().inner(), phrase);
    }
}
