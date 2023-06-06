use subxt::tx::{DynamicTxPayload, StaticTxPayload};

use crate::extrinsics::prelude::calldata::ContractCall;

pub type ContractTransactionPayload<T = ContractCall> = StaticTxPayload<T>;
