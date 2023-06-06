use subxt::tx::{StaticTxPayload, DynamicTxPayload};

pub type ContractTransactionPayload<T> = StaticTxPayload<T>;
