use sp_core::H256;

use super::facade::DhatuAssetsFacade;

pub(crate) type AssetDatabaseId = i64;
pub(crate) type UserEmail = String;
pub(crate) type TransactionId = H256;

/// trait for mandala blockchain nft assets
/// assets that implements this trait can be migrated with [asset facade](DhatuAssetsFacade).
pub trait Asset {
    /// return the contract address of given asset.
    fn contract_address(&self) -> &str;

    /// return the token id owned by some account of given asset.
    fn token_id(&self) -> i64;

    /// the `transfer` function selector of given asset.
    /// this could be obtained from the contract abi.
    fn function_selector(&self) -> &str;
}
