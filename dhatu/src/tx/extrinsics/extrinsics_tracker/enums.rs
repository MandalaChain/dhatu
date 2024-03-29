use std::sync::Arc;

use serde::Serialize;
use subxt::blocks::ExtrinsicEvents;
use subxt::ext::sp_core::H256;

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

impl ExtrinsicStatus {
    /// Returns `true` if the extrinsic status is [`Pending`].
    ///
    /// [`Pending`]: ExtrinsicStatus::Pending
    #[must_use]
    pub fn is_pending(&self) -> bool {
        matches!(self, Self::Pending)
    }

    /// Returns `true` if the extrinsic status is [`Failed`].
    ///
    /// [`Failed`]: ExtrinsicStatus::Failed
    #[must_use]
    pub fn is_failed(&self) -> bool {
        matches!(self, Self::Failed(..))
    }

    /// Returns `true` if the extrinsic status is [`Success`].
    ///
    /// [`Success`]: ExtrinsicStatus::Success
    #[must_use]
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success(..))
    }
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
    pub fn into_inner(self) -> Arc<ExtrinsicEvents<MandalaConfig>> {
        self.0
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
        let hex_str = format!("0x{}", hex::encode(value.as_bytes()));
        Self(hex_str)
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

#[cfg(test)]
mod hash_tests {
    use super::*;
    use subxt::ext::sp_core::hexdisplay::HexDisplay;

    #[test]
    fn test_from_h256() {
        let h256 = H256::random();

        let hex_display_h256 = HexDisplay::from(&h256.0);
        let h256_str = format!("0x{}", hex_display_h256.to_string());

        let hash_str = Hash::from(h256).to_string();

        assert_eq!(hash_str, h256_str)
    }
}
