use subxt::{tx::SubmittableExtrinsic, OnlineClient, PolkadotConfig, SubstrateConfig};

use crate::{tx::extrinsics::prelude::{
    transfer_nft_contract::types::ContractTransactionPayload, extrinsics::TransactionMessage,
}, types::{SenderChannel, ReceiverChannel}};

pub type MigrationTask<T> = std::pin::Pin<Box<dyn futures::Future<Output = T>>>;
pub type MigrationTransactionPayload = ContractTransactionPayload;
pub type MigrationTransactionResultNotifier =
    SenderChannel<TransactionMessage>;
pub type MigrationTransactionResultReceiver =
    ReceiverChannel<TransactionMessage>;
pub type MigrationTransaction = SubmittableExtrinsic<SubstrateConfig, crate::types::NodeClient>;
