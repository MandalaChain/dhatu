use std::{collections::HashMap, sync::Arc};

use futures::{future, FutureExt};
use sp_core::sr25519::Pair;
use tokio::sync::RwLock;

use crate::extrinsics::{
    funds_reserve::traits::FundsReserveTraits,
    prelude::{reserve::FundsReserve, BlockchainClient},
};

use super::{
    migration_transaction::{
        builder::MigrationTransactionBuilderStruct,
        traits::{MigrationTask, MigrationTransactionBuilder},
        types::{MigrationTransactionResultNotifier, MigrationTransactionResultReceiver},
    },
    traits::{Asset, AssetManagerAttributes, AssetManagerTrait, MigrationTransactionMap},
};

pub type PublicAddress = String;

pub struct AssetManager {
    notifier: MigrationTransactionResultNotifier,
    txs: MigrationTransactionMap,
}

impl AssetManager {
    pub fn new(notifier: MigrationTransactionResultNotifier) -> Self {
        let txs = HashMap::new();
        let txs = Arc::new(RwLock::new(txs));

        Self { notifier, txs }
    }

    pub fn migrate(
        &self,
        assets: Vec<impl Asset>,
        from: Pair,
        to: PublicAddress,
        client: BlockchainClient,
        reserve: &FundsReserve,
    ) {
        let mut vec = Vec::new();
        let notifier = self.notifider();

        for (asset) in assets {
            let mut tx = MigrationTransactionBuilderStruct::new()
                .set_signer(from.clone())
                .set_notifier(notifier.clone())
                .set_gas_reserve(reserve.clone())
                .set_client(client.clone())
                .build();

            let sign_tx_ops = tx
                .construct_payload(
                    asset.contract_address(),
                    &to,
                    asset.token_id(),
                    asset.function_selector(),
                )
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

impl AssetManagerAttributes for AssetManager {
    fn notifider(&self) -> &MigrationTransactionResultNotifier {
        &self.notifier
    }

    fn txs(&self) -> &super::traits::MigrationTransactionMap {
        &self.txs
    }
}

impl AssetManagerTrait for AssetManager {}
