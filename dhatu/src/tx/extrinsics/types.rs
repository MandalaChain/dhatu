use serde_json::Value;
use sp_core::H256;
use subxt::{
    tx::{SubmittableExtrinsic, TxProgress},
    OnlineClient, PolkadotConfig,
};

use super::extrinsics_tracker::enums::ExtrinsicStatus;




pub type GenericError = Box<dyn std::error::Error>;






