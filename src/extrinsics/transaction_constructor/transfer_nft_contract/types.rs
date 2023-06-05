use subxt::tx::StaticTxPayload;

pub type ContractTransactionPayload = StaticTxPayload<runtime_types::api::contracts::calls::Call>;
