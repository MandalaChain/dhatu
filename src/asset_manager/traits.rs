use std::{collections::HashMap, sync::Arc};

use sp_core::H256;
use tokio::sync::RwLock;

use crate::{
    db::{client::Database},
};

use super::{
    migration_transaction::types::{
        MigrationTransactionResultNotifier, MigrationTransactionResultReceiver,
    },
    types::ManageAssetTask,
};

pub trait AssetManagerTask {
    fn update_owner(db: Database, asset_id: i64, email: &str) -> ManageAssetTask;

    fn set_migration_as_main(db: Database, email: String) -> ManageAssetTask;

    fn receive_notifier(
        inner: MigrationTransactionMap,
        db: Database,
        channel: MigrationTransactionResultReceiver,
    );

    /// check main account for assets
    fn is_asset_migration_done(db: Database, email: String) -> ManageAssetTask<bool>;

    fn start(&mut self);
}

pub type AssetDatabaseId = i64;
pub type UserEmail = String;
pub type TransactionId = H256;
pub type MigrationTransactionMap =
    Arc<RwLock<HashMap<TransactionId, (UserEmail, AssetDatabaseId)>>>;

pub trait AssetManagerAttributes {
    fn database(&self) -> &Database;

    fn notifider(&self) -> &MigrationTransactionResultNotifier;

    fn txs(&self) -> &MigrationTransactionMap;
}

pub trait AssetManagerTrait: AssetManagerAttributes + AssetManagerTask {}
