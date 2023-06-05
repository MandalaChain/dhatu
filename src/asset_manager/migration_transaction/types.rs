use subxt::{tx::SubmittableExtrinsic, OnlineClient, PolkadotConfig};

use crate::extrinsics::prelude::{transfer_nft_contract::types::ContractTransactionPayload, NotificationMessage};

pub type MigrationTask<T> = std::pin::Pin<Box<dyn futures::Future<Output = T>>>;
pub type MigrationTransactionPayload = ContractTransactionPayload;
pub type MigrationTransactionResultNotifier =
    tokio::sync::mpsc::UnboundedSender<NotificationMessage>;
pub type MigrationTransactionResultReceiver =
    tokio::sync::mpsc::UnboundedReceiver<NotificationMessage>;
pub type MigrationTransaction = SubmittableExtrinsic<PolkadotConfig, OnlineClient<PolkadotConfig>>;
