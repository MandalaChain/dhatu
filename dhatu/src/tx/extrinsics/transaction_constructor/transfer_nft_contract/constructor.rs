use std::str::FromStr;

use parity_scale_codec::Encode;
use subxt::utils::AccountId32;

use crate::{tx::extrinsics::{
    prelude::calldata::{CallData, },
    transaction_constructor::{
        calldata::ContractCall,
        traits::{ScaleEncodeable, ToContractPayload, },
    },
}, registrar::signer::WrappedExtrinsic, error::ToPayloadError};

/// NFT transfer function arguments
pub struct NftTransferAgrs {
    function_selector: String,
    to: AccountId32,
    id: u32,
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

/// NFT transfer extrinsic constructor.
pub struct TransferNFT;

impl TransferNFT {
    /// encode payload calldata.
    fn encode_calldata(
        to: &str,
        token_id: i64,
        function_selector: String,
    ) -> Result<CallData<NftTransferAgrs>, crate::error::Error> {
        // convert rust types to substrate primitives
        let to =
            AccountId32::from_str(to).map_err(|e| ToPayloadError::AddressError(e.to_string()))?;
        let id = token_id as u32;

        // build call data
        // TODO : make this standardized by using traits
        let args = NftTransferAgrs::new(function_selector, to, id);

        Ok(args.into())
    }

    /// construct nft transfer extrinsic payload.
    pub fn construct(
        address: &str,
        to: &str,
        token_id: i64,
        function_selector: String,
    ) -> Result<NftTransferPayload, crate::error::Error> {
        Self::encode_calldata(to, token_id, function_selector)?
            .to_payload(address)
            .map(|v| v.into())
    }
}

/// NFT transfer extrinsic payload.
pub struct NftTransferPayload(subxt::tx::Payload<ContractCall>);

impl WrappedExtrinsic<ContractCall> for NftTransferPayload {
    fn into_inner(self) -> subxt::tx::Payload<ContractCall> {
        self.0
    }
}

impl From<subxt::tx::Payload<ContractCall>> for NftTransferPayload {
    fn from(value: subxt::tx::Payload<ContractCall>) -> Self {
        Self(value)
    }
}
