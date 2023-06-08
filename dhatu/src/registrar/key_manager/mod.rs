/// module responsible for all related task regarding user keypair. e.g creating, recovering, etc.
pub mod keypair;
pub mod password;

use std::str::FromStr;

use sp_core::{crypto::Ss58Codec, sr25519::Pair as Keys, Pair};

pub mod prelude {
    pub use super::keypair::*;
    pub use super::password::*;
}

use prelude::*;

use crate::{error::DhatuError, tx::extrinsics::prelude::GenericError};

/// represent a keypair manager.
pub struct KeyManager;

impl KeyManager {
    pub fn new_default() -> Keypair {
        let password = Password::new();
        Self::gen(password)
    }

    pub fn recover(pass: &str, phrase: &str) -> Result<Keypair, DhatuError> {
        let password = Password::from_str(pass)?;
        Self::gen_from_phrase(password, phrase)
            .map_err(|e| DhatuError::KeypairGenError(e.to_string()))
    }
}

impl KeyManager {
    fn gen(password: Password) -> Keypair {
        let password_phrase = password.as_pwd();

        let (keypair, phrase, _) = Keys::generate_with_phrase(password_phrase);

        Self::construct(password, phrase, keypair)
    }

    fn gen_from_phrase(password: Password, phrase: &str) -> Result<Keypair, GenericError> {
        let password_phrase = password.as_pwd();

        let (keys, _) = Keys::from_phrase(phrase, password_phrase)?;
        let keypair = Self::construct(password, String::from(phrase), keys);
        Ok(keypair)
    }

    fn construct(password: Password, phrase: String, keypair: Keys) -> Keypair {
        let pub_key = keypair.public().to_ss58check();
        let password_hash = password.as_pwd().unwrap().to_string();

        Keypair::new(password_hash, phrase, pub_key, keypair)
    }
}

#[cfg(test)]
mod tests {

    use mockall::mock;

    use super::*;

    mock! {
        VerifiableUser{}

        impl User for VerifiableUser {
            fn password(&self) -> &str;
            fn email(&self) -> &str;

        }

        impl Verifiable for VerifiableUser {
            fn phrase(&self) -> &str;
        }

        impl VerifiableUser  for VerifiableUser {}
    }

    mock! {
        TestUser{}

        impl User for TestUser {
            fn password(&self) -> &str;
            fn email(&self) -> &str;

        }
    }

    fn email() -> String {
        String::from("test@example.com")
    }

    fn password() -> String {
        String::from("some very very secret password")
    }

    fn create_mock_user() -> MockTestUser {
        let mut user = MockTestUser::new();

        user.expect_email().return_const(email());
        user.expect_password().return_const(password());

        user
    }

    fn create_mock_verifiable_user() -> MockVerifiableUser {
        let mut user = MockVerifiableUser::new();

        user.expect_email().return_const(email());
        user.expect_password().return_const(password());

        let keypair = KeyManager::with_user(&user);

        user.expect_phrase()
            .return_const(keypair.phrase().to_string());

        user
    }

    #[test]
    fn should_produce_consistent_hash() {
        let email = "test@example.com";
        let password = "some very very secret password";

        let first_try_pass = Password::new_with_creds(email, password);
        let second_try_pass = Password::new_with_creds(email, password);

        assert_eq!(first_try_pass.0, second_try_pass.0)
    }

    #[test]
    fn should_recover_keypair() {
        assert!(std::panic::catch_unwind(should_produce_consistent_hash).is_ok());

        let user = create_mock_verifiable_user();

        let recovered_keypair = KeyManager::recover_with_creds(&user);

        assert!(recovered_keypair.is_ok());
    }

    #[test]
    fn should_consistently_verify_keypair() {
        assert!(std::panic::catch_unwind(should_produce_consistent_hash).is_ok());

        let user = create_mock_verifiable_user();

        assert!(KeyManager::verify(&user))
    }
}
