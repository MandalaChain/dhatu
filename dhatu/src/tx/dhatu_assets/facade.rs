use std::{collections::HashMap, sync::Arc};

use futures::{future, FutureExt};
use sp_core::sr25519::Pair;
use tokio::sync::RwLock;

use crate::tx::extrinsics::prelude::{reserve::FundsReserve, BlockchainClient};

use super::{
    migration_transaction::{
        builder::MigrationTransactionBuilderStruct,
        traits::MigrationTransactionBuilder,
        types::{MigrationTransactionResultNotifier, MigrationTransactionResultReceiver},
    },
    traits::{Asset, AssetManagerAttributes, AssetManagerTrait, MigrationTransactionMap},
};

pub(crate) type PublicAddress = String;

pub struct DhatuAssetsFacade {
    txs: MigrationTransactionMap,
}

impl DhatuAssetsFacade {
    pub fn new() -> Self {
        let txs = HashMap::new();
        let txs = Arc::new(RwLock::new(txs));

        Self { txs }
    }

    // TODO : optimize the migration with queue
    pub fn migrate(
        &self,
        assets: Vec<impl Asset>,
        from: Pair,
        to: PublicAddress,
        client: BlockchainClient,
        reserve: &FundsReserve,
        notifier: MigrationTransactionResultNotifier,
    ) {
        let mut tx_batch = Vec::new();

        for asset in assets {
            let tx = MigrationTransactionBuilderStruct::new()
                .set_signer(from.clone())
                .set_notifier(notifier.clone())
                .set_gas_reserve(reserve.clone())
                .set_client(client.clone())
                .build();

            let tx = tx
                .construct_payload(
                    asset.contract_address(),
                    &to,
                    asset.token_id(),
                    asset.function_selector(),
                )
                .sign()
                .then(|tx| async move { tx.ensure_enough_gas().await })
                .then(|tx| async move { tx.submit().await });

            tx_batch.push(tx)
        }

        // TODO : refactor this to executes the futures in pararell
        let transactions =  future::join_all(tx_batch);
        tokio::task::spawn(transactions);
    }

    pub fn create_channels() -> (
        MigrationTransactionResultNotifier,
        MigrationTransactionResultReceiver,
    ) {
        tokio::sync::mpsc::unbounded_channel()
    }
}

impl AssetManagerAttributes for DhatuAssetsFacade {
    fn txs(&self) -> &super::traits::MigrationTransactionMap {
        &self.txs
    }
}

impl AssetManagerTrait for DhatuAssetsFacade {}
