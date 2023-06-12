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
    types::{MandalaTransactionProgress, SenderChannel},
};

use super::enums::{ExtrinsicStatus, Hash};

pub struct TransactionMessage {
    pub(crate) status: ExtrinsicStatus,
    pub(crate) callback: Option<String>,
    pub(crate) id: Hash,
}

impl TransactionMessage {
    pub fn new(status: ExtrinsicStatus, callback: Option<String>, id: Hash) -> Self {
        Self {
            status,
            callback,
            id,
        }
    }

    pub fn inner_status(&self) -> ExtrinsicStatus {
        self.status.clone()
    }

    pub fn callback(&self) -> Option<&String> {
        self.callback.as_ref()
    }

    pub fn id(&self) -> &Hash {
        &self.id
    }
}

#[cfg(feature = "tokio")]
pub struct Transaction {
    id: H256,
    status: Arc<RwLock<ExtrinsicStatus>>,
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
        external_notifier: Option<SenderChannel<TransactionMessage>>,
        callback: Option<String>,
    ) -> Self {
        let hash = tx.0.extrinsic_hash();
        let task_channel = Self::process_transaction(tx, external_notifier, callback);

        let default_status = Self::watch_transaction_status(task_channel);

        Self {
            id: hash,
            status: default_status,
        }
    }

    fn process_transaction(
        tx: MandalaTransactionProgress,
        external_status_notifier: Option<SenderChannel<TransactionMessage>>,
        callback: Option<String>,
    ) -> Receiver<TransactionMessage> {
        let (internal_status_notifier, receiver) = Self::create_channel();

        let task = async move {
            let id = tx.0.extrinsic_hash().into();
            let status = Self::wait(tx).await;

            internal_status_notifier
                .send(status.clone())
                .await
                .expect("there should be only 1 message sent");

            if let Some(external_status_notifier) = external_status_notifier {
                let msg = TransactionMessage::new(status, callback, id);
                external_status_notifier.send(msg).await;
            }
        };
        tokio::task::spawn(task);
        receiver
    }

    pub async fn wait(tx: MandalaTransactionProgress) -> ExtrinsicStatus {
        let status = tx.0.wait_for_finalized_success().await;

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
