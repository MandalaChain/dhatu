use crate::{
    error::Error,
    tx::extrinsics::{
        callback_executor::Url, extrinsics_tracker::extrinsics::TransactionMessage,
        prelude::enums::Hash,
    },
    types::{InternalChannels, MandalaExtrinsics, ReceiverChannel, SenderChannel},
};

use super::super::{
    callback_executor::Executor, extrinsics_tracker::tracker::ExtrinsicWatcher,
    prelude::ExtrinsicSubmitter,
};

/// extrinsics facade.
#[cfg(feature = "tokio")]
#[cfg(feature = "serde")]
pub struct ExtrinsicFacade {
    transaction_watcher: ExtrinsicWatcher,
    transaction_sender_channel: SenderChannel<TransactionMessage>,
}

impl ExtrinsicFacade {
    /// create new extrinsics facade.
    pub fn new(mut channels: InternalChannels<TransactionMessage>) -> Self {
        let tx_receiver_channel = channels.get_receiver();

        let callback_executor = Executor::new();
        let tx_watcher = ExtrinsicWatcher::new();

        Self::initialize_receive_task(tx_watcher.clone(), callback_executor, tx_receiver_channel);

        let tx_sender_channel = channels.sender().clone();

        Self {
            transaction_watcher: tx_watcher,
            transaction_sender_channel: tx_sender_channel,
        }
    }

    /// internal function. should not be exposed to the user.
    ///
    /// this will stop watching the transaction and execute the callback if there's any.
    /// this will be executed in a separate tokio task.
    fn initialize_receive_task(
        tx_watcher: ExtrinsicWatcher,
        callback_executor: Executor,
        mut tx_receiver_channel: ReceiverChannel<TransactionMessage>,
    ) {
        let recv = async move {
            loop {
                // its okay to use unwrap
                let msg = tx_receiver_channel.recv().await.unwrap();

                tx_watcher.stop_watching(msg.id()).await;

                if let Some(callback) = msg.callback() {
                    // will fail silently if if there's an error when executing the callback
                    callback_executor.execute(msg.status.clone(), callback.to_owned());
                }
            }
        };

        tokio::task::spawn(recv);
    }

    /// submit a new extrinsics transaction.
    pub async fn submit(
        &self,
        tx: MandalaExtrinsics,
        callback: Option<Url>,
    ) -> Result<Hash, Error> {
        let progress = ExtrinsicSubmitter::submit(tx).await?;
        let tx = self
            .transaction_watcher
            .watch(
                progress,
                Some(self.transaction_sender_channel.clone()),
                callback,
            )
            .await;

        Ok(tx)
    }

    /// inner tx watcher
    pub fn watcher(&self) -> &ExtrinsicWatcher {
        &self.transaction_watcher
    }
}
