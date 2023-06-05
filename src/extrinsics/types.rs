use serde_json::Value;
use sp_core::H256;
use subxt::{
    tx::{SubmittableExtrinsic, TxProgress},
    OnlineClient, PolkadotConfig,
};

use super::extrinsics_tracker::enums::ExtrinsicStatus;

pub type BlockchainClientConfig = PolkadotConfig;

pub type BlockchainClient = OnlineClient<BlockchainClientConfig>;

pub type Extrinsic = SubmittableExtrinsic<BlockchainClientConfig, BlockchainClient>;

pub type GenericError = Box<dyn std::error::Error>;

pub type ExtrinsicTracker = TxProgress<BlockchainClientConfig, BlockchainClient>;

pub type OngoingTransaction = tokio::task::JoinHandle<ExtrinsicStatus>;

pub type Callback = String;
pub type NotificationMessage = (TransactionId, ExtrinsicStatus, Option<Callback>);

pub type TransactionId = H256;

pub type CallbackBody = Value;
pub type ExecuteCallbackMessage = (TransactionId, CallbackBody);

pub type URL = String;
pub type AddPendingCallbackMessage = (TransactionId, URL);
