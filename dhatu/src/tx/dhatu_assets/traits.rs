use std::{collections::HashMap, sync::Arc};

use sp_core::H256;
use tokio::sync::RwLock;



pub(crate) type AssetDatabaseId = i64;
pub(crate) type UserEmail = String;
pub(crate) type TransactionId = H256;



pub trait Asset {
    fn contract_address(&self) -> &str;

    fn token_id(&self) -> i64;

    fn function_selector(&self) -> &str;
}
