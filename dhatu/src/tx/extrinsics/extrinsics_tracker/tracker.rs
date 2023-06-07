use std::{collections::HashMap, sync::Arc};

use sp_core::H256;
use tokio::sync::RwLock;

use crate::tx::extrinsics::{
    prelude::{NotificationMessage, TransactionId},
    types::{BlockchainClient, ExtrinsicTracker},
};

use super::{enums::ExtrinsicStatus, extrinsics::Transaction};

#[doc(hidden)]
type Inner = Arc<RwLock<HashMap<H256, Transaction>>>;

pub struct ExtrinsicWatcher {
    inner: Inner,
    client: BlockchainClient,
    transaction_notifier: tokio::sync::mpsc::UnboundedSender<NotificationMessage>,
}

impl Clone for ExtrinsicWatcher {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            client: self.client.clone(),
            transaction_notifier: self.transaction_notifier.clone(),
        }
    }
}

impl ExtrinsicWatcher {
    pub fn new(
        client: BlockchainClient,
        transaction_notifier: tokio::sync::mpsc::UnboundedSender<NotificationMessage>,
    ) -> Self {
        let inner = HashMap::new();
        let inner = Arc::new(RwLock::new(inner));

        Self {
            inner,
            client,
            transaction_notifier,
        }
    }

    pub async fn watch(&self, tx: ExtrinsicTracker, callback: Option<String>) -> TransactionId {
        let tx = Transaction::new(tx, self.transaction_notifier.clone(), callback);
        let tx_id = tx.id();

        self.watch_tx(tx).await;

        tx_id
    }

    pub async fn check(&self, tx_id: &TransactionId) -> Option<ExtrinsicStatus> {
        let inner = self.inner.read().await;

        let Some(tx) = inner.get(tx_id) else {
            return None;
        };

        Some(tx.status().await)
    }

    pub async fn stop_watching(&self, tx_id: &TransactionId) {
        let mut inner = self.inner.write().await;
        inner.remove(tx_id);
    }

    async fn watch_tx(&self, tx: Transaction) {
        let mut inner = self.inner.write().await;
        inner.insert(tx.id(), tx);
    }
}
