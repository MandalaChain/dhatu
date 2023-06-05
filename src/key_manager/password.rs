use std::str::FromStr;

use tiny_keccak::{Hasher, Sha3};

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use crate::extrinsics::prelude::GenericError;

const DEFAULT_PASSWORD_LENGTH: usize = 32;

/// represents a password hash used to securely generate and recover user keypair.
#[derive(Clone)]
pub struct Password(pub String);

impl FromStr for Password {
    type Err = GenericError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(String::from(s)))
    }
}

impl Password {
    pub fn new() -> Self {
        Self(Self::generate_random_string(DEFAULT_PASSWORD_LENGTH))
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn as_pwd(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    pub fn new_with_creds(email: &str, password: &str) -> Self {
        let hash = Self::gen_hash(email, password);
        let hash = Self::to_hex(hash);

        Self(hash)
    }

    fn gen_hash(email: &str, password: &str) -> Sha3 {
        let mut hasher = tiny_keccak::Sha3::v256();

        hasher.update(email.as_bytes());
        hasher.update(password.as_bytes());

        hasher
    }

    fn to_hex(_hash: Sha3) -> String {
        let mut hash = [0; 32];
        _hash.finalize(&mut hash);

        hex::encode(hash)
    }

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
