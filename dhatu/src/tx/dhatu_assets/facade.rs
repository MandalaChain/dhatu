use std::{collections::HashMap, sync::Arc};

use crate::{registrar::key_manager::prelude::{PrivateKey, PublicAddress}, types::MandalaClient};
use futures::{future, FutureExt};


use tokio::sync::RwLock;

use crate::tx::extrinsics::prelude::{reserve::FundsReserve};

use super::{
    migration_transaction::{
        builder::MigrationTransactionBuilderStruct,
        traits::MigrationTransactionBuilder,
        types::{MigrationTransactionResultNotifier, MigrationTransactionResultReceiver},
    },
    traits::{Asset, },
};


pub struct DhatuAssetsFacade {
    client: MandalaClient,
}

impl DhatuAssetsFacade {
    pub fn new(mandala_client: MandalaClient) -> Self {

        Self {
            client: mandala_client,
        }
    }

    // TODO : optimize the migration with queue
    pub fn migrate(
        &self,
        assets: Vec<impl Asset>,
        from: PrivateKey,
        to: PublicAddress,
        reserve: &FundsReserve,
        notifier: MigrationTransactionResultNotifier,
    ) {
        let mut tx_batch = Vec::new();
        let client = self.client.inner();

        for asset in assets {
            let tx = MigrationTransactionBuilderStruct::new()
                .set_signer(from.0.clone())
                .set_notifier(notifier.clone())
                .set_gas_reserve(reserve.clone())
                .set_client(client.clone())
                .build();

            let tx = tx
                .construct_payload(
                    asset.contract_address(),
                    &to.0,
                    asset.token_id(),
                    asset.function_selector(),
                )
                .sign()
                .then(|tx| async move { tx.ensure_enough_gas().await })
                .then(|tx| async move { tx.submit().await });

            tx_batch.push(tx)
        }

        // TODO : refactor this to executes the futures in pararell
        let transactions = future::join_all(tx_batch);
        tokio::task::spawn(transactions);
    }

    pub fn create_channels() -> (
        MigrationTransactionResultNotifier,
        MigrationTransactionResultReceiver,
    ) {
        tokio::sync::mpsc::unbounded_channel()
    }
}


