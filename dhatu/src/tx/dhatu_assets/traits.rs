

use sp_core::H256;




pub(crate) type AssetDatabaseId = i64;
pub(crate) type UserEmail = String;
pub(crate) type TransactionId = H256;



pub trait Asset {
    fn contract_address(&self) -> &str;

    fn token_id(&self) -> i64;

    fn function_selector(&self) -> &str;
}
