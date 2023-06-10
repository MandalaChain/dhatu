use std::sync::Arc;

use sp_core::H256;

use tokio::sync::{
    mpsc::{Receiver, Sender},
    Mutex,
};

use crate::{
    tx::extrinsics::{
        prelude::{NotificationMessage, TransactionId},
        types::ExtrinsicTracker,
    },
    types::MandalaTransactionProgress,
};

use super::enums::ExtrinsicStatus;

pub struct Transaction {
    id: H256,
    status: Arc<Mutex<ExtrinsicStatus>>,
    transaction_notifier: tokio::sync::mpsc::UnboundedSender<NotificationMessage>,
}

impl Transaction {
    pub fn id(&self) -> TransactionId {
        self.id
    }

    pub async fn status(&self) -> ExtrinsicStatus {
        let status = self.status.lock().await;

        status.clone()
    }
}

impl Transaction {
    pub fn new(
        tx: MandalaTransactionProgress,
        external_notifier: tokio::sync::mpsc::UnboundedSender<NotificationMessage>,
        callback: Option<String>,
    ) -> Self {
        let hash = tx.extrinsic_hash();
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

            internal_status_notifier.send(status.clone()).await.unwrap();

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
            Ok(tx) => ExtrinsicStatus::Success(tx.extrinsic_hash().into()),
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
    ) -> Arc<Mutex<ExtrinsicStatus>> {
        let default_status = Arc::new(Mutex::new(ExtrinsicStatus::default()));
        let status_arc_clone = default_status.clone();

        let watcher = async move {
            let Some(new_status) = task_channel.recv().await else {
            return ;
        };
            let mut status = status_arc_clone.lock().await;
            *status = new_status;
        };

        tokio::task::spawn(watcher);

        default_status
    }
}
