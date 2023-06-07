use subxt::tx::TxPayload;

use crate::tx::extrinsics::prelude::{calldata::CallData, GenericError};

pub trait NftTransferTransactionConstructor<T: TxPayload> {
    fn construct(
        address: &str,
        to: &str,
        token_id: i64,
        function_selector: String,
    ) -> Result<T, GenericError>;
}

pub(super) trait ContractCallDataEncoder<T> {
    fn encode_calldata(
        to: &str,
        token_id: i64,
        function_selector: String,
    ) -> Result<CallData<T>, GenericError>;
}
