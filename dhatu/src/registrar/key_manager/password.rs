use std::str::FromStr;

use tiny_keccak::{Hasher, Sha3};

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use crate::error::{Error, PasswordGenerationError};

/// default password length used to generate password.
pub const DEFAULT_PASSWORD_LENGTH: usize = 32;

/// represents a password hash used to securely generate and recover user keypair.
#[derive(Clone)]
pub struct Password(pub String);

impl ToString for Password {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl Password {
    /// generate a new password with [default](DEFAULT_PASSWORD_LENGTH) length of 32.
    /// uses [rand] with [Alphanumeric] to generate a random string.
    pub fn new() -> Self {
        Self(Self::generate_random_string(DEFAULT_PASSWORD_LENGTH))
    }

    /// get the string representation of this password.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// internal function. should not be exposed to user.
    /// used to convieniently convert this password to a `Optoion<&str>`.
    /// meant to be used to generate and recover keypair.
    pub(crate) fn as_pwd(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    /// # NOTE : EXPERIMENTAL.
    /// maybe removed in the stable release.
    ///
    /// generate a new password with a email and password.
    /// maybe useful for web3 social login implementation.
    pub fn new_with_creds(email: &str, password: &str) -> Self {
        let hash = Self::gen_hash(email, password);
        let hash = Self::to_hex(hash);

        Self(hash)
    }
}

impl FromStr for Password {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.len().cmp(&DEFAULT_PASSWORD_LENGTH) {
            std::cmp::Ordering::Less => Err(PasswordGenerationError::InvalidLength)?,
            _ => Ok(Self(String::from(s))),
        }
    }
}
impl From<Option<&str>> for Password {
    fn from(value: Option<&str>) -> Self {
        let value = value
            .expect("this implemetation should only been called from inside the crates!")
            .to_string();

        Password(value)
    }
}

impl Password {
    /// internal function. should not be exposed to user.
    /// generate sha3(keccak) hash from email and password.
    fn gen_hash(email: &str, password: &str) -> Sha3 {
        let mut hasher = tiny_keccak::Sha3::v256();

        hasher.update(email.as_bytes());
        hasher.update(password.as_bytes());

        hasher
    }
    /// internal function. should not be exposed to user.
    /// encode the hash to hex string.
    fn to_hex(_hash: Sha3) -> String {
        let mut hash = [0; 32];
        _hash.finalize(&mut hash);

        hex::encode(hash)
    }

    /// internal function. should not be exposed to user.
    /// generate random string with given length.
    fn generate_random_string(length: usize) -> String {
        let rng = thread_rng();
        let random_string: String = rng
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect();

        random_string
    }
}
#[cfg(test)]
mod password_tests {
    use super::*;

    #[test]
    fn test_password_new() {
        let password = Password::new();
        assert_eq!(password.0.len(), DEFAULT_PASSWORD_LENGTH);
    }

    #[test]
    fn test_password_from_str_valid() {
        let password_str = "2ff7a050bef5dd0b2982f6538287acde";
        let password = Password::from_str(password_str).unwrap();
        assert_eq!(password.0, password_str, "valid password");
    }

    #[test]
    fn test_password_from_str_invalid() {
        let password_str = "shortpwd";
        let result = Password::from_str(password_str);
        assert!(result.is_err(), "invalid password");
    }
    #[test]
    fn test_gen_hash() {
        let email = "test@example.com";
        let password = "password123";

        let hash = Password::gen_hash(email, password);

        let expected_hash = Password::gen_hash(email, password);

        // Convert the hash values to strings
        let hash_str = Password::to_hex(hash);
        let expected_hash_str = Password::to_hex(expected_hash);

        // Perform assertions on the hash or its properties
        assert_eq!(hash_str, expected_hash_str, "hash");
    }

    #[test]
    fn test_to_hex() {
        // Create an example Sha3 hash
        let mut hash = Sha3::v256();

        // Update the hash with some data
        hash.update(b"test");

        // Clone the hash before finalizing
        let cloned_hash = hash.clone();

        // Finalize the cloned hash and obtain the result
        let mut result = [0; 32];
        cloned_hash.finalize(&mut result);

        // Convert the result to a hex string using the to_hex function
        let hash_hex = Password::to_hex(hash);

        // Convert the result to a hex string using the hex crate
        let expected_hex = hex::encode(result);

        // Perform assertion to check if the conversion is correct
        assert_eq!(hash_hex, expected_hex);
    }

    #[test]

    fn test_generate_random_string() {
        // Define the desired length of the random string
        let length = DEFAULT_PASSWORD_LENGTH;

        // Generate a random string using the generate_random_string function
        let random_string = Password::generate_random_string(length);

        // Check if the generated string has the correct length
        assert_eq!(random_string.len(), length);

        // Check if the generated string only contains alphanumeric characters
        assert!(random_string.chars().all(|c| c.is_ascii_alphanumeric()));
    }
}
