use std::sync::Arc;

use serde::Serialize;
use sp_core::H256;
use subxt::{blocks::ExtrinsicEvents};

use crate::types::MandalaConfig;

/// extrinsic transaction progress.
/// note that this track the transaction based on finalized block.
#[derive(Debug, Clone)]
pub enum ExtrinsicStatus {
    /// transaction is pending, and have not been included in any block.
    Pending,
    /// transaction failed, with certain reasons.
    /// this could be various reasons. see [here](SubstrateTxStatus) for more details.
    Failed(Reason),
    /// transaction is included in a finalized block.
    Success(ExtrinsicResult),
}

impl Default for ExtrinsicStatus {
    fn default() -> Self {
        Self::Pending
    }
}

/// failed extrinsic reason.
#[cfg(feature = "serde")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct Reason(String);

impl Reason {
    pub fn inner(&self) -> &str {
        self.0.as_str()
    }
}

impl ToString for Reason {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl From<String> for Reason {
    fn from(value: String) -> Self {
        Self(value)
    }
}


/// extrinsic result. contains events associated with the extrinsic.
/// 
/// note that for now, to access the raw events, you need to enable `unstable_sp_core` feature flag.
/// this restriction will be lifted in the future.
/// 
/// currently only supports returning the extrinsic hash.
/// 
// TODO : provide a way to access the inner events without depending on subxt and sp_core types.
#[derive(Debug)]
pub struct ExtrinsicResult(Arc<ExtrinsicEvents<MandalaConfig>>);

impl Clone for ExtrinsicResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl ExtrinsicResult {
    #[cfg(feature = "unstable_sp_core")]
    pub fn into_inner(self) -> ExtrinsicEvents<MandalaConfig> {
        Arc::try_unwrap(self.0).expect("should be able to unwrap!")
    }

    #[cfg(feature = "unstable_sp_core")]
    pub fn inner(&self) -> &ExtrinsicEvents<MandalaConfig> {
        &self.0
    }
}

impl ExtrinsicResult {
    pub fn hash(&self) -> Hash {
        self.0.extrinsic_hash().into()
    }
}

impl From<ExtrinsicEvents<MandalaConfig>> for ExtrinsicResult {
    fn from(value: ExtrinsicEvents<MandalaConfig>) -> Self {
        Self(Arc::new(value))
    }
}

/// extrinsic hash.
#[cfg(feature = "serde")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct Hash(String);

impl From<ExtrinsicResult> for Hash {
    fn from(value: ExtrinsicResult) -> Self {
        Self(value.hash().to_string())
    }
}

impl From<H256> for Hash {
    fn from(value: H256) -> Self {
        Self(value.to_string())
    }
}

impl ToString for Hash {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl Hash {
    pub fn inner_as_str(&self) -> &str {
        self.0.as_str()
    }

    // we disable this by default because substrate sp_core does not follow semver
    // and we need to have a stable public api!
    #[cfg(feature = "unstable_sp_core")]
    pub fn into_inner(&self) -> H256 {
        use std::str::FromStr;

        H256::from_str(self.inner_as_str()).expect("internal conversion shouldn't fail!")
    }
}
