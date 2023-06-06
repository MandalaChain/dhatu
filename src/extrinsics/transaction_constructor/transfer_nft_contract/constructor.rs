use std::str::FromStr;

use subxt::utils::AccountId32;

use crate::extrinsics::{
    prelude::{calldata::CallData, BlockchainClient, GenericError},
    transaction_constructor::traits::ToContractPayload,
};

use super::{
    traits::{ContractCallDataEncoder, NftTransferTransactionConstructor},
    types::ContractTransactionPayload,
};

pub struct TransferNFT;

impl ContractCallDataEncoder for TransferNFT {
    fn encode_calldata(
        to: &str,
        token_id: i64,
        function_selector: &str,
    ) -> Result<CallData, GenericError> {
        // convert rust types to substrate primitives
        let to = AccountId32::from_str(to)?;
        let id = token_id as u32;

        // build call data
        // TODO : make this standardized by using traits
        let args = (function_selector, to, id);
        let args = subxt::ext::codec::Encode::encode(&args);

        Ok(args.into())
    }
}

impl NftTransferTransactionConstructor<ContractTransactionPayload> for TransferNFT {
    fn construct(
        address: &str,
        to: &str,
        token_id: i64,
        function_selector: &str,
        client: BlockchainClient,
    ) -> Result<ContractTransactionPayload, GenericError> {
        Self::encode_calldata(to, token_id, function_selector)?.to_payload(address, client)
    }
}
