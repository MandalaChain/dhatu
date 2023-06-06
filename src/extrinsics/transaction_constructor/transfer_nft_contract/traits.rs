use subxt::tx::TxPayload;

use crate::extrinsics::prelude::{calldata::CallData, BlockchainClient, GenericError};

pub trait NftTransferTransactionConstructor<T: TxPayload> {
    fn construct(
        address: &str,
        to: &str,
        token_id: i64,
        function_selector: &str,
        client: BlockchainClient,
    ) -> Result<T, GenericError>;
}

pub(super) trait ContractCallDataEncoder {
    fn encode_calldata(
        to: &str,
        token_id: i64,
        function_selector: &str,
    ) -> Result<CallData, GenericError>;
}
