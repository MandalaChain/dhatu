use subxt::tx::{StaticTxPayload, DynamicTxPayload};

pub type ContractTransactionPayload = DynamicTxPayload<'static>;
