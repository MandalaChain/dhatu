use std::sync::Arc;

use sp_core::H256;

use tokio::sync::{
    mpsc::{Receiver, Sender},
    Mutex, RwLock,
};

use crate::{
    tx::extrinsics::{
        prelude::{NotificationMessage, TransactionId},
        types::ExtrinsicTracker,
    },
    types::MandalaTransactionProgress,
};

use super::enums::{ExtrinsicStatus, Hash};

#[cfg(feature = "tokio")]
pub struct Transaction {
    id: H256,
    status: Arc<RwLock<ExtrinsicStatus>>,
    transaction_notifier: tokio::sync::mpsc::UnboundedSender<NotificationMessage>,
}

impl Transaction {
    pub fn id(&self) -> Hash {
        self.id.into()
    }

    pub async fn status(&self) -> ExtrinsicStatus {
        let status = self.status.read().await;

        status.clone()
    }
}

impl Transaction {
    pub fn new(
        tx: MandalaTransactionProgress,
        external_notifier: tokio::sync::mpsc::UnboundedSender<NotificationMessage>,
        callback: Option<String>,
    ) -> Self {
        let hash = tx.0.extrinsic_hash();
        let task_channel = Self::process_transaction(tx, external_notifier.clone(), callback);

        let default_status = Self::watch_transaction_status(task_channel);

        Self {
            transaction_notifier: external_notifier,
            id: hash,
            status: default_status,
        }
    }

    fn process_transaction(
        tx: ExtrinsicTracker,
        external_status_notifier: tokio::sync::mpsc::UnboundedSender<NotificationMessage>,
        callback: Option<String>,
    ) -> Receiver<ExtrinsicStatus> {
        let (internal_status_notifier, receiver) = Self::create_channel();

        let task = async move {
            let hash = tx.extrinsic_hash();

            let status = Self::wait(tx).await;

            internal_status_notifier
                .send(status.clone())
                .await
                .expect("there should be only 1 message sent");

            external_status_notifier
                .send((hash, status.clone(), callback))
                .unwrap();
        };
        tokio::task::spawn(task);
        receiver
    }

    pub async fn wait(tx: ExtrinsicTracker) -> ExtrinsicStatus {
        let status = tx.wait_for_finalized_success().await;

        match status {
            Ok(tx) => ExtrinsicStatus::Success(tx.into()),
            Err(e) => ExtrinsicStatus::Failed(e.to_string().into()),
        }
    }

    fn create_channel() -> (Sender<ExtrinsicStatus>, Receiver<ExtrinsicStatus>) {
        // only 1 message will ever be sent so we don't need buffer size more than 1
        let default_buffer_size = 1_usize;
        tokio::sync::mpsc::channel::<ExtrinsicStatus>(default_buffer_size)
    }

    fn watch_transaction_status(
        mut task_channel: Receiver<ExtrinsicStatus>,
    ) -> Arc<RwLock<ExtrinsicStatus>> {
        let default_status = Arc::new(RwLock::new(ExtrinsicStatus::default()));
        let status_arc_clone = default_status.clone();

        let watcher = async move {
            let Some(new_status) = task_channel.recv().await else {
            return ;
        };
            let mut status = status_arc_clone.write().await;
            *status = new_status;
        };

        tokio::task::spawn(watcher);

        default_status
    }
}
