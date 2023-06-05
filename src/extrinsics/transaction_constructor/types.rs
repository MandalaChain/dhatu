use subxt::tx::{TxPayload};

pub type TransactionPayload = Box<dyn TxPayload>;

