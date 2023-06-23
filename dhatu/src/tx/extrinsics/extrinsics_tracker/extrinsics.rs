use std::sync::Arc;

use sp_core::H256;

use tokio::sync::{
    mpsc::{Receiver, Sender},
    RwLock,
};

use crate::{
    tx::extrinsics::callback_executor::Url,
    types::{MandalaTransactionProgress, SenderChannel},
};

use super::enums::{ExtrinsicStatus, Hash};

/// transaction message.
/// this is what will be sent to external notifier after the transaction is completed
pub struct TransactionMessage {
    pub(crate) status: ExtrinsicStatus,
    pub(crate) callback: Option<Url>,
    pub(crate) id: Hash,
}

impl TransactionMessage {
    /// internal function. should not be exposed to the user.
    /// create new transaction message.
    pub(crate) fn new(status: ExtrinsicStatus, callback: Option<Url>, id: Hash) -> Self {
        Self {
            status,
            callback,
            id,
        }
    }

    /// get transaction status.
    pub fn inner_status(&self) -> ExtrinsicStatus {
        self.status.clone()
    }

    /// get callback url.
    pub fn callback(&self) -> Option<&Url> {
        self.callback.as_ref()
    }

    /// get transaction id.
    pub fn id(&self) -> &Hash {
        &self.id
    }
}

/// transaction wrapper.
/// this wrap raw substrate extrinsics transaction and will track the transaction status.
#[cfg(feature = "tokio")]
pub struct Transaction {
    /// transaction id.
    id: H256,
    /// transaction status.
    status: Arc<RwLock<ExtrinsicStatus>>,
}

impl Transaction {
    /// get transaction id.
    pub fn id(&self) -> Hash {
        self.id.into()
    }

    /// get transaction status.
    pub async fn status(&self) -> ExtrinsicStatus {
        let status = self.status.read().await;

        status.clone()
    }

    fn infer_err(e: subxt::Error) -> ExtrinsicStatus {
        match e {
            subxt::Error::Io(_) => todo!(),
            subxt::Error::Codec(_) => todo!(),
            subxt::Error::Rpc(_) => todo!(),
            subxt::Error::Serialization(_) => todo!(),
            subxt::Error::Metadata(_) => todo!(),
            subxt::Error::MetadataDecoding(_) => todo!(),
            subxt::Error::Runtime(e) => ExtrinsicStatus::Failed(format!("{e}").into()),
            subxt::Error::Decode(_) => todo!(),
            subxt::Error::Encode(_) => todo!(),
            subxt::Error::Transaction(_) => todo!(),
            subxt::Error::Block(_) => todo!(),
            subxt::Error::StorageAddress(_) => todo!(),
            subxt::Error::Unknown(_) => todo!(),
            subxt::Error::Other(_) => todo!(),
            _ => todo!(),
        }
    }
}

impl Transaction {
    /// create new transaction.
    pub fn new(
        tx: MandalaTransactionProgress,
        external_notifier: Option<SenderChannel<TransactionMessage>>,
        callback: Option<Url>,
    ) -> Self {
        let hash: H256 = tx.0.extrinsic_hash();
        let task_channel = Self::process_transaction(tx, external_notifier, callback);

        let default_status = Self::watch_transaction_status(task_channel);

        Self {
            id: hash,
            status: default_status,
        }
    }

    /// watch transaction status. and send notification through channel if provided after the transaction is completed.
    fn process_transaction(
        tx: MandalaTransactionProgress,
        external_status_notifier: Option<SenderChannel<TransactionMessage>>,
        callback: Option<Url>,
    ) -> Receiver<ExtrinsicStatus> {
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
                external_status_notifier.send(msg);
            }
        };
        tokio::task::spawn(task);
        receiver
    }

    /// manually wait for transaction to be completed.
    // we expose this function to user for convenience. this enables manually waiting the transaction to complete.
    pub async fn wait(tx: MandalaTransactionProgress) -> ExtrinsicStatus {
        let status = tx.0.wait_for_finalized_success().await;

        match status {
            Ok(tx) => ExtrinsicStatus::Success(tx.into()),
            Err(e) => Self::infer_err(e),
        }
    }
    /// create channel for sending transaction status.
    fn create_channel() -> (Sender<ExtrinsicStatus>, Receiver<ExtrinsicStatus>) {
        // only 1 message will ever be sent so we don't need buffer size more than 1
        let default_buffer_size = 1_usize;
        tokio::sync::mpsc::channel::<ExtrinsicStatus>(default_buffer_size)
    }

    /// watch transaction status.
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

#[cfg(test)]
mod transaction_tests {
    use super::*;
    use crate::registrar::key_manager::prelude::PublicAddress;
    use crate::registrar::signer::TxBuilder;
    use crate::tx::extrinsics::extrinsics_submitter::ExtrinsicSubmitter;
    use crate::tx::extrinsics::manager::facade::ExtrinsicFacade;
    use crate::tx::extrinsics::prelude::transfer_balance::constructor::BalanceTransfer;
    use crate::types::MandalaConfig;
    use crate::types::MandalaExtrinsics;
    use crate::types::Unit;
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

        let value = Unit::new("0.00001", None).expect("static conversion should not fail");
        // Create the payload using the `construct` function from `BalanceTransfer`
        let payload = BalanceTransfer::construct(new_address, value);
        let extrinsic = TxBuilder::signed(&node_client.into(), pair.into(), payload)
            .await
            .unwrap()
            .0;

        // Create a mock MandalaExtrinsics object
        let tx = MandalaExtrinsics::new(extrinsic);
        let tx_progress = ExtrinsicSubmitter::submit(tx).await.unwrap();

        tx_progress
    }
    // Create a sample MandalaTransactionProgress and other required variables
    #[tokio::test]
    async fn new_transaction_tests() {
        let tx_progress = create_tx_progress().await;
        let callback = "https://example.net/a/b/c.png";

        let reqwest_url = reqwest::Url::parse(callback).expect("Failed to parse the callback URL");
        let url = Url(reqwest_url);

        let extrinsic_hash = tx_progress.0.extrinsic_hash();

        let result = Transaction::new(tx_progress, None, Some(url));

        assert_eq!(result.id, extrinsic_hash);
    }
    #[tokio::test]
    async fn test_process_transaction() {
        // Create a mock MandalaTransactionProgress
        let tx_progress = create_tx_progress().await;
        let callback = "https://example.net/a/b/c.png";
        let reqwest_url = reqwest::Url::parse(callback).expect("Failed to parse the callback URL");
        let url = Url(reqwest_url);

        let mut result_receiver = Transaction::process_transaction(tx_progress, None, Some(url));

        let result = result_receiver.try_recv();

        // Add assertions or further checks as needed
        assert!(
            result.is_err(),
            "Result receiver should not have received any values yet"
        );
    }
    #[tokio::test]
    async fn wait_tests() {
        // Create a mock MandalaTransactionProgress
        let tx_progress = create_tx_progress().await;

        // Call the wait function

        let status = Transaction::wait(tx_progress).await;
        // Assert the expected outcome based on the mock implementation
        match status {
            ExtrinsicStatus::Success(_) => {
                // Handle the success case
                // Add assertions or further checks as needed
                assert!(true, "Extrinsic was finalized successfully");
            }
            ExtrinsicStatus::Failed(reason) => {
                // Handle the failure case
                // Add assertions or further checks as needed
                // assert!(false, "Extrinsic failed to finalize");
                panic!("{:?}", reason);
            }
            ExtrinsicStatus::Pending => {
                // Handle the pending case
                // Add assertions or further checks as needed
                assert!(false, "Extrinsic is still pending");
            }
        }
    }
}
