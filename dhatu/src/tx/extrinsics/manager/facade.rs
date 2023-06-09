use serde::Serialize;

use sp_core::H256;

use crate::tx::extrinsics::prelude::{NotificationMessage, TransactionId};

use super::super::{
    callback_executor::Executor,
    extrinsic_reporting::ExtrinsicReportStorage,
    extrinsics_tracker::{enums::ExtrinsicStatus, tracker::ExtrinsicWatcher},
    prelude::{submitter::ExtrinsicSubmitter, GenericError},
    types::{BlockchainClient, Extrinsic},
};

#[doc(hidden)]
type Task = tokio::task::JoinHandle<()>;

pub type TransactionWatcherInstance = ExtrinsicWatcher;
pub type CallbackExecutorInstance = Executor;

// temporary callback body
#[doc(hidden)]
#[derive(Serialize)]
pub struct Body {
    hash: H256,
}

impl Body {
    pub fn new(hash: H256) -> Self {
        Self { hash }
    }
}

pub struct ExtrinsicFacade {
    transaction_watcher: TransactionWatcherInstance,
    callback_executor: CallbackExecutorInstance,
    transaction_sender_channel: tokio::sync::mpsc::UnboundedSender<NotificationMessage>,
    notif_bus_handle: Task,
    report_storage: ExtrinsicReportStorage,
}

impl ExtrinsicFacade {
    pub fn new(client: BlockchainClient) -> Self {
        let (tx_sender_channel, tx_receiver_channel) = Self::create_channel();

        let callback_executor = Executor::new();

        let tx_watcher = ExtrinsicWatcher::new(client, tx_sender_channel.clone());

        let report_storage = ExtrinsicReportStorage::new();

        let rcv_handle = Self::initialize_receive_task(
            tx_watcher.clone(),
            callback_executor.clone(),
            tx_receiver_channel,
            report_storage.clone(),
        );

        Self {
            transaction_watcher: tx_watcher,
            callback_executor,
            transaction_sender_channel: tx_sender_channel,
            notif_bus_handle: rcv_handle,
            report_storage,
        }
    }

    fn initialize_receive_task(
        tx_watcher: TransactionWatcherInstance,
        callback_executor: CallbackExecutorInstance,
        mut tx_receiver_channel: tokio::sync::mpsc::UnboundedReceiver<NotificationMessage>,
        report_storage: ExtrinsicReportStorage,
    ) -> Task {
        let recv = async move {
            loop {
                let (id, status, callback) = tx_receiver_channel.recv().await.unwrap();

                tx_watcher.stop_watching(&id).await;
                report_storage.set_status(&id, status.clone()).await;

                if let Some(callback) = callback {
                    // TODO : customize body
                    callback_executor
                        .execute(serde_json::to_value(Body::new(id)).unwrap(), &callback);
                }
            }
        };

        tokio::task::spawn(recv)
    }

    pub async fn submit(
        &self,
        tx: Extrinsic,
        callback: Option<String>,
    ) -> Result<TransactionId, GenericError> {
        let (tx, _id) = ExtrinsicSubmitter::submit(tx).await?;
        let tx = self.transaction_watcher.watch(tx, callback).await;

        Ok(tx)
    }

    pub async fn get_status(&self, tx_id: &TransactionId) -> Option<ExtrinsicStatus> {
        if let Some(status) = self.transaction_watcher.check(tx_id).await {
            Some(status)
        } else if let Some(status) = self.report_storage.get_status(tx_id).await {
            return Some(status);
        } else {
            return None;
        }
    }

    pub fn create_channel() -> (
        tokio::sync::mpsc::UnboundedSender<NotificationMessage>,
        tokio::sync::mpsc::UnboundedReceiver<NotificationMessage>,
    ) {
        tokio::sync::mpsc::unbounded_channel::<NotificationMessage>()
    }

    // TODO : figure out when to remove the transaction result
    pub async fn remove(&mut self, tx_id: &TransactionId) {
        self.report_storage.remove(tx_id).await
    }
}
