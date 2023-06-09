use sp_core::H256;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reason(String);

impl Reason {
    pub fn inner(&self) -> &str {
        self.0.as_str()
    }
}

impl From<String> for Reason {
    fn from(value: String) -> Self {
        Self(value)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hash(String);

impl From<H256> for Hash {
    fn from(value: H256) -> Self {
        Self(value.to_string())
    }
}

impl Hash {
    pub fn inner_as_str(&self) -> &str {
        self.0.as_str()
    }

    // we disable this by default because substrate sp_core does not follow semver
    // and we need to have a stable public api!
    #[cfg(feature = "unstable_sp_core")]
    pub fn hash(&self) -> H256 {
        use std::str::FromStr;

        H256::from_str(self.inner_as_str()).expect("internal conversion shouldn't fail!")
    }
}


#[derive(Debug, Clone)]
pub enum ExtrinsicStatus {
    Pending,
    Failed(Reason),
    Success(Hash),
}

impl Default for ExtrinsicStatus {
    fn default() -> Self {
        Self::Pending
    }
}
