use std::{collections::HashMap, sync::Arc};


use tokio::sync::RwLock;

use crate::{
    types::{MandalaClient, MandalaTransactionProgress, SenderChannel},
};

use super::{
    enums::{ExtrinsicStatus, Hash},
    extrinsics::{Transaction, TransactionMessage},
};

#[doc(hidden)]
type Inner = Arc<RwLock<HashMap<Hash, Transaction>>>;

pub struct ExtrinsicWatcher {
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
    pub fn new(_client: MandalaClient) -> Self {
        let inner = HashMap::new();
        let inner = Arc::new(RwLock::new(inner));

        Self { inner }
    }

    pub async fn watch(
        &self,
        tx: MandalaTransactionProgress,
        external_notifier: Option<SenderChannel<TransactionMessage>>,
        callback: Option<String>,
    ) -> Hash {
        let tx = Transaction::new(tx, external_notifier, callback);
        let tx_id = tx.id();

        self.watch_tx(tx).await;

        tx_id
    }

    pub async fn check(&self, tx_id: &Hash) -> Option<ExtrinsicStatus> {
        let inner = self.inner.read().await;

        let Some(tx) = inner.get(tx_id) else {
            return None;
        };

        Some(tx.status().await)
    }

    pub async fn stop_watching(&self, tx_id: &Hash) {
        let mut inner = self.inner.write().await;
        inner.remove(tx_id);
    }

    async fn watch_tx(&self, tx: Transaction) {
        let mut inner = self.inner.write().await;
        inner.insert(tx.id(), tx);
    }
}
