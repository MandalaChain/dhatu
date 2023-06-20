use std::{collections::HashMap, sync::Arc};


use tokio::sync::RwLock;

use crate::{
    types::{MandalaTransactionProgress, SenderChannel}, tx::extrinsics::callback_executor::Url,
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
