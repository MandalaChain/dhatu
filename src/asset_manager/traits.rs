use std::{collections::HashMap, sync::Arc};

use sp_core::H256;
use tokio::sync::RwLock;

use super::migration_transaction::types::{
    MigrationTransactionResultNotifier,
};

pub type AssetDatabaseId = i64;
pub type UserEmail = String;
pub type TransactionId = H256;
pub type MigrationTransactionMap =
    Arc<RwLock<HashMap<TransactionId, (UserEmail, AssetDatabaseId)>>>;

pub trait AssetManagerAttributes {
    fn notifider(&self) -> &MigrationTransactionResultNotifier;

    fn txs(&self) -> &MigrationTransactionMap;
}

pub trait AssetManagerTrait: AssetManagerAttributes {}

pub trait Asset {
    fn contract_address(&self) -> &str;

    fn token_id(&self) -> i64;

    fn function_selector(&self) -> &str;
}
