use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use crate::{
    tx::extrinsics::callback_executor::Url,
    types::{MandalaClient, MandalaTransactionProgress, SenderChannel},
};

use super::{
    enums::{ExtrinsicStatus, Hash},
    extrinsics::{Transaction, TransactionMessage},
};

#[doc(hidden)]
type Inner = Arc<RwLock<HashMap<Hash, Transaction>>>;

/// extrinsics tracker.
/// track extrinsics status on the blockchain runtime.
pub struct ExtrinsicWatcher {
    /// inner map for transaction tracking.
    inner: Inner,
}

impl Clone for ExtrinsicWatcher {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl ExtrinsicWatcher {
    /// create new extrinsics tracker.
    pub fn new() -> Self {
        let inner = HashMap::new();
        let inner = Arc::new(RwLock::new(inner));

        Self { inner }
    }

    /// watch extrinsics transaction.
    pub async fn watch(
        &self,
        tx: MandalaTransactionProgress,
        external_notifier: Option<SenderChannel<TransactionMessage>>,
        callback: Option<Url>,
    ) -> Hash {
        let tx = Transaction::new(tx, external_notifier, callback);
        let tx_id = tx.id();

        self.watch_tx(tx).await;

        tx_id
    }

    /// check extrinsics status.
    pub async fn check(&self, tx_id: &Hash) -> Option<ExtrinsicStatus> {
        let inner = self.inner.read().await;

        let Some(tx) = inner.get(tx_id) else {
            return None;
        };

        Some(tx.status().await)
    }

    /// stop watching extrinsics transaction.
    /// will remove the transaction from the tracker inner map.
    pub async fn stop_watching(&self, tx_id: &Hash) {
        let mut inner = self.inner.write().await;
        inner.remove(tx_id);
    }

    /// watch extrinsics transaction.
    /// internal function. should not be exposed to the user.
    async fn watch_tx(&self, tx: Transaction) {
        let mut inner = self.inner.write().await;
        inner.insert(tx.id(), tx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registrar::key_manager::prelude::PublicAddress;
    use crate::registrar::signer::TxBuilder;
    use crate::tx::extrinsics::extrinsics_submitter::ExtrinsicSubmitter;
    use crate::tx::extrinsics::manager::facade::ExtrinsicFacade;
    use crate::types::MandalaConfig;
    use crate::types::MandalaExtrinsics;
    use std::str::FromStr;
    use std::sync::mpsc;
    pub(crate) use subxt::OnlineClient;

    fn mock_pair() -> sp_core::sr25519::Pair {
        sp_keyring::Sr25519Keyring::Alice.pair()
    }

    async fn mock_client() -> crate::types::NodeClient {
        OnlineClient::<MandalaConfig>::new().await.unwrap()
    }

    async fn create_tx_progress() -> MandalaTransactionProgress {
        let address = "5DJk1gegyQJk6BNs7LceZ1akt5e9fpm4gUYGzcfpKaLG9Mmb";
        let new_address = PublicAddress::from_str(address).unwrap();
        let pair = mock_pair();
        let node_client = mock_client().await;

        let value = 10000;
        // Create the payload using the `construct` function from `BalanceTransfer`
        let payload = crate::tx::extrinsics::prelude::transfer_balance::constructor::BalanceTransfer::construct(new_address, value);
        let extrinsic = TxBuilder::signed(&node_client.into(), pair, payload)
            .await
            .unwrap()
            .0;

        // Create a mock MandalaExtrinsics object
        let tx = MandalaExtrinsics::new(extrinsic);
        let tx_progress = ExtrinsicSubmitter::submit(tx).await.unwrap();

        tx_progress
    }

    #[tokio::test]
    async fn should_watch_unwatch_and_check_tx() {
        let tx_progress = create_tx_progress().await;

        let watcher = ExtrinsicWatcher::new();
        let tx_id = watcher.watch(tx_progress, None, None).await;

        let status = watcher.check(&tx_id).await.unwrap();

        match status {
            ExtrinsicStatus::Pending => {}
            _ => panic!("should be pending"),
        }

        watcher.stop_watching(&tx_id).await;

        let status = watcher.check(&tx_id).await;
        assert!(status.is_none());
    }
}
