use subxt::tx::TxPayload;

use crate::extrinsics::prelude::{calldata::CallData, GenericError};

pub trait NftTransferTransactionConstructor<T: TxPayload> {
    fn construct(
        address: &str,
        to: &str,
        token_id: i64,
        function_selector: &str,
    ) -> Result<T, GenericError>;
}

pub(super) trait ContractCallDataEncoder {
    fn encode_calldata(
        to: &str,
        token_id: i64,
        function_selector: &str,
    ) -> Result<CallData, GenericError>;
}
