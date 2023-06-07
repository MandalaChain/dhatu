use crate::tx::extrinsics::prelude::calldata::ContractCall;

pub type ContractTransactionPayload<T = ContractCall> = subxt::tx::Payload<T>;
