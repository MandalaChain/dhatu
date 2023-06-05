use std::{collections::HashMap, sync::Arc};

use tokio::sync::{RwLock};

use super::prelude::{enums::ExtrinsicStatus, TransactionId};

pub type ExtrinsicResultTempStorage = HashMap<TransactionId, ExtrinsicStatus>;
pub type InnerStorage = Arc<RwLock<ExtrinsicResultTempStorage>>;

pub struct ExtrinsicReportStorage {
    inner: InnerStorage,
}

impl Clone for ExtrinsicReportStorage {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl ExtrinsicReportStorage {
    pub fn new() -> Self {
        let inner = HashMap::new();
        let inner = Arc::new(RwLock::new(inner));

        Self { inner }
    }

    pub async fn set_status(&self, tx_id: &TransactionId, status: ExtrinsicStatus) {
        let mut inner = self.inner.write().await;
        inner.insert(*tx_id, status);
    }

    pub async fn get_status(&self, tx_id: &TransactionId) -> Option<ExtrinsicStatus> {
        let inner = self.inner.read().await;
        inner.get(tx_id).cloned()
    }

    pub async fn remove(&self, tx_id: &TransactionId) {
        let mut inner = self.inner.write().await;
        inner.remove(tx_id);
    }
}
