use std::str::FromStr;

use parity_scale_codec::Encode;
use subxt::utils::AccountId32;

use crate::{
    error::ToPayloadError,
    registrar::{key_manager::prelude::PublicAddress, signer::WrappedExtrinsic},
    tx::extrinsics::{
        prelude::calldata::CallData,
        transaction_constructor::{
            calldata::{ContractCall, Selector},
            traits::{ScaleEncodeable, ToContractPayload},
        },
    },
};

/// NFT transfer function arguments
pub struct NftTransferAgrs {
    function_selector: Selector,
    to: PublicAddress,
    id: u32,
}

impl NftTransferAgrs {
    fn new(function_selector: Selector, to: PublicAddress, id: u32) -> Self {
        Self {
            function_selector,
            to,
            id,
        }
    }
}

impl ScaleEncodeable for NftTransferAgrs {
    fn encode(self) -> Vec<u8> {
        let selector = self.function_selector.to_string();
        let to = AccountId32::from(self.to);

        (selector, to, self.id).encode()
    }
}

/// NFT transfer extrinsic constructor.
pub struct TransferNFT;

impl TransferNFT {
    /// encode payload calldata.
    fn encode_calldata(
        to: PublicAddress,
        token_id: u32,
        function_selector: Selector,
    ) -> Result<CallData<NftTransferAgrs>, crate::error::Error> {
        let args = NftTransferAgrs::new(function_selector, to, token_id);

        Ok(args.into())
    }

    /// construct nft transfer extrinsic payload.
    pub fn construct(
        address: PublicAddress,
        to: PublicAddress,
        token_id: u32,
        function_selector: Selector,
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
