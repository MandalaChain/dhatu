use std::{collections::HashMap, sync::Arc};

use futures::{future, FutureExt};
use sp_core::sr25519::Pair;
use tokio::sync::RwLock;

use crate::{
    db::{
        client::Database,
        model::prelude::{AssetEntity, AssetMetadataEntity},
        traits::AssetMigrationAttr,
    },
    extrinsics::{
        funds_reserve::traits::FundsReserveTraits,
        prelude::{reserve::FundsReserve, BlockchainClient},
    },
};

use super::{
    migration_transaction::{
        builder::MigrationTransactionBuilderStruct,
        traits::{MigrationTask, MigrationTransactionBuilder},
        types::{MigrationTransactionResultNotifier, MigrationTransactionResultReceiver},
    },
    traits::{
        AssetManagerAttributes, AssetManagerTask, AssetManagerTrait, MigrationTransactionMap,
    },
    types::ManageAssetTask,
};

pub type PublicAddress = String;

pub struct AssetManager {
    db: Database,
    notifier: Option<MigrationTransactionResultNotifier>,
    txs: MigrationTransactionMap,
}

impl AssetManager {
    pub fn new(db: Database) -> Self {
        let txs = HashMap::new();
        let txs = Arc::new(RwLock::new(txs));

        Self {
            db,
            notifier: None,
            txs,
        }
    }

    pub fn migrate(
        &self,
        assets: Vec<(AssetEntity, AssetMetadataEntity)>,
        from: Pair,
        to: PublicAddress,
        client: BlockchainClient,
        reserve: &FundsReserve,
    ) {
        let mut vec = Vec::new();
        let notifier = self.notifider();

        for (asset, metadata) in assets {
            let mut tx = MigrationTransactionBuilderStruct::new()
                .set_signer(from.clone())
                .set_notifier(notifier.clone())
                .set_gas_reserve(reserve.clone())
                .set_client(client.clone())
                .build();

            let sign_tx_ops = tx
                .construct_payload(&metadata.address, &to, asset.token_id, &metadata.selector)
                .sign();

            vec.push(sign_tx_ops)
        }

        let transactions = async move {
            let txs = future::join_all(vec).await;
            let mut vec = Vec::new();

            for tx in txs {
                vec.push(tx.ensure_enough_gas());
            }

            let txs = future::join_all(vec).await;
            let mut vec = Vec::new();

            for tx in txs {
                vec.push(tx.submit())
            }

            let txs = future::join_all(vec).await;
        };

        tokio::task::spawn(transactions);
    }

    fn create_channels() -> (
        MigrationTransactionResultNotifier,
        MigrationTransactionResultReceiver,
    ) {
        tokio::sync::mpsc::unbounded_channel()
    }
}

impl AssetManagerTask for AssetManager {
    fn update_owner(db: Database, asset_id: i64, email: &str) -> ManageAssetTask {
        db.update_owner(email, asset_id)
    }

    fn set_migration_as_main(db: Database, email: String) -> ManageAssetTask {
        db.set_migration_account_as_main(email)
    }

    fn receive_notifier(
        inner: MigrationTransactionMap,
        db: Database,
        mut channel: MigrationTransactionResultReceiver,
    ) {
        let task = async move {
            loop {
                let (tx_id, _status, _) = channel.recv().await.unwrap();
                let mut inner = inner.write().await;

                let (email, asset_id) = inner.remove(&tx_id).unwrap();

                Self::update_owner(db.clone(), asset_id, &email).await;

                if Self::is_asset_migration_done(db.clone(), email.clone()).await {
                    Self::set_migration_as_main(db.clone(), email).await;
                }
            }
        };

        tokio::spawn(task);
    }

    fn start(&mut self) {
        let (sender, receiver) = Self::create_channels();

        self.notifier = Some(sender);

        let inner = self.txs.clone();
        let db = self.db.clone();

        Self::receive_notifier(inner, db, receiver);
    }

    fn is_asset_migration_done(db: Database, email: String) -> ManageAssetTask<bool> {
        async move {
            let main_account_assets = db.query_main_account_assets(email).await.unwrap();
            main_account_assets.is_empty()
        }
        .boxed()
    }
}

impl AssetManagerAttributes for AssetManager {
    fn database(&self) -> &Database {
        &self.db
    }

    fn notifider(&self) -> &MigrationTransactionResultNotifier {
        self.notifier
            .as_ref()
            .expect("channel should have been built when starting")
    }

    fn txs(&self) -> &super::traits::MigrationTransactionMap {
        &self.txs
    }
}

impl AssetManagerTrait for AssetManager {}
