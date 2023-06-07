use sp_core::sr25519::Pair as Keys;

/// represent a user keypair and its infos.
pub struct Keypair {
    password_hash: String,
    phrase: String,
    pub_key: String,
    keypair: Keys,
}

impl Keypair {
    pub(crate) fn new(password_hash: String, phrase: String, pub_key: String, keypair: Keys) -> Self {
        Self {
            password_hash,
            phrase,
            pub_key,
            keypair,
        }
    }

    pub fn password_hash(&self) -> &str {
        self.password_hash.as_ref()
    }

    pub fn phrase(&self) -> &str {
        self.phrase.as_ref()
    }

    pub fn pub_key(&self) -> &str {
        self.pub_key.as_ref()
    }

    pub fn keypair(&self) -> &Keys {
        &self.keypair
    }
}
