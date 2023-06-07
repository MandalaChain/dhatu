use subxt::tx::TxPayload;

use crate::tx::extrinsics::prelude::GenericError;

pub trait TransferBalanceTransactionConstructor<T: TxPayload> {
    fn construct_tx(
        &self,
        address: &str,
        from: &str,
        to: &str,
        token_id: i64,
        function_selector: &str,
    ) -> Result<T, GenericError>;
}
