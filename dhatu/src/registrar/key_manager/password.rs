use std::str::FromStr;

use tiny_keccak::{Hasher, Sha3};

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use crate::error::{Error, PasswordGenerationError};

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
