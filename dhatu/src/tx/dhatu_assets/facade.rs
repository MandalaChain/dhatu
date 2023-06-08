use std::{collections::HashMap, sync::Arc};

use crate::{error::Error, registrar::key_manager::prelude::Keypair, MandalaClient};
use futures::{future, FutureExt};
use sp_core::sr25519::Pair;
use std::str::FromStr;
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublicKey(String);

impl FromStr for PublicKey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        subxt::utils::AccountId32::from_str(s)
            .map(|v| PublicKey(v.to_string()))
            .map_err(|e| Error::KeypairGenError(e.to_string()))
    }
}

impl From<Keypair> for PublicKey {
    fn from(value: Keypair) -> Self {
        PublicKey(String::from(value.pub_key()))
    }
}

#[derive(Clone)]
pub struct SecretKey(Pair);

impl FromStr for SecretKey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        schnorrkel::Keypair::from_bytes(s.as_bytes())
            .map(|v| SecretKey(Pair::from(v)))
            .map_err(|e| Error::KeypairGenError(e.to_string()))
    }
}

impl From<Keypair> for SecretKey {
    fn from(value: Keypair) -> Self {
        SecretKey(value.keypair().clone())
    }
}

pub struct DhatuAssetsFacade {
    txs: MigrationTransactionMap,
    client: MandalaClient,
}

impl DhatuAssetsFacade {
    pub fn new(mandala_client: MandalaClient) -> Self {
        let txs = HashMap::new();
        let txs = Arc::new(RwLock::new(txs));

        Self {
            client: mandala_client,
            txs,
        }
    }

    // TODO : optimize the migration with queue
    pub fn migrate(
        &self,
        assets: Vec<impl Asset>,
        from: SecretKey,
        to: PublicKey,
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

impl AssetManagerAttributes for DhatuAssetsFacade {
    fn txs(&self) -> &super::traits::MigrationTransactionMap {
        &self.txs
    }
}

impl AssetManagerTrait for DhatuAssetsFacade {}
