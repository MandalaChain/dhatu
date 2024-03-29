use subxt::ext::sp_core::H256;

use crate::{
    registrar::key_manager::prelude::PublicAddress,
    tx::extrinsics::transaction_constructor::calldata::Selector,
};

pub(crate) type AssetDatabaseId = i64;
pub(crate) type UserEmail = String;
pub(crate) type TransactionId = H256;

/// trait for mandala blockchain nft assets
/// assets that implements this trait can be migrated with [asset facade](super::facade::DhatuAssetsFacade).
pub trait Asset {
    /// return the contract address of given asset.
    fn contract_address(&self) -> PublicAddress;

    /// return the token id owned by some account of given asset.
    fn token_id(&self) -> u32;

    /// the `transfer` function selector of given asset.
    /// this could be obtained from the contract abi.
    fn function_selector(&self) -> Selector;
}
