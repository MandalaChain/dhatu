use subxt::tx::TxPayload;

use crate::extrinsics::prelude::{calldata::CallData, BlockchainClient, GenericError};

pub trait NftTransferTransactionConstructor<T: TxPayload> {
    fn construct(
        address: &str,
        to: &str,
        token_id: i64,
        function_selector: String,
        client: BlockchainClient,
    ) -> Result<T, GenericError>;
}

pub(super) trait ContractCallDataEncoder<T> {
    fn encode_calldata(
        to: &str,
        token_id: i64,
        function_selector: String,
    ) -> Result<CallData<T>, GenericError>;
}
