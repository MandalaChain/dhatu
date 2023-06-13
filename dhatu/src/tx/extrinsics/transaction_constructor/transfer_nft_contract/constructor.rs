use std::str::FromStr;

use parity_scale_codec::Encode;
use subxt::utils::AccountId32;

use crate::tx::extrinsics::{
    prelude::{calldata::{CallData, ToPayloadError}, },
    transaction_constructor::traits::{ScaleEncodeable, ToContractPayload},
};

use super::{
    traits::{ContractCallDataEncoder, NftTransferTransactionConstructor},
    types::ContractTransactionPayload,
};

pub struct TransferNFT;

pub struct NftTransferAgrs {
    pub function_selector: String,
    pub to: AccountId32,
    pub id: u32,
}

impl NftTransferAgrs {
    fn new(function_selector: String, to: AccountId32, id: u32) -> Self {
        Self {
            function_selector,
            to,
            id,
        }
    }
}

impl ScaleEncodeable for NftTransferAgrs {
    fn encode(self) -> Vec<u8> {
        (self.function_selector, self.to, self.id).encode()
    }
}

impl ContractCallDataEncoder<TransferNFT> for TransferNFT {
    fn encode_calldata(
        to: &str,
        token_id: i64,
        function_selector: String,
    ) -> Result<CallData<TransferNFT>, crate::error::Error> {
        // convert rust types to substrate primitives
        let to = AccountId32::from_str(to).map_err(|e| ToPayloadError::AddressError(e.to_string()))?;
        let id = token_id as u32;

        // build call data
        // TODO : make this standardized by using traits
        let args = NftTransferAgrs::new(function_selector, to, id);

        Ok(args.into())
    }
}

impl NftTransferTransactionConstructor<ContractTransactionPayload> for TransferNFT {
    fn construct(
        address: &str,
        to: &str,
        token_id: i64,
        function_selector: String,
    ) -> Result<ContractTransactionPayload, crate::error::Error> {
        Self::encode_calldata(to, token_id, function_selector)?.to_payload(address)
    }
}
