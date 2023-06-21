use parity_scale_codec::Encode;
use subxt::{
    tx::Payload,
    utils::{AccountId32, MultiAddress},
};

use crate::{
    registrar::{key_manager::prelude::PublicAddress, signer::WrappedExtrinsic},
    runtime_types::{
        self,
        api::{contracts::calls::types::Call, runtime_types::sp_weights::weight_v2::Weight},
    },
    tx::extrinsics::transaction_constructor::{calldata::Selector, traits::ScaleEncodeable},
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
        let mut calldata = Vec::new();

        calldata.append(&mut self.function_selector.encoded());
        calldata.append(&mut AccountId32::from(self.to).encode());
        calldata.append(&mut self.id.encode());

        calldata
    }
}

/// NFT transfer extrinsic constructor.
pub struct TransferNFT;

const DEFAULT_PROOF_SIZE: u64 = 500_000_000_000;
const DEFAULT_REF_TIME: u64 = 1_000_000_000;

const DEFAULT_TX_VALUE: u128 = 0;
const DEFAULT_DEPOSIT_LIMIT: u128 = 0;

const STATIC_GAS_LIMIT: Weight = Weight {
    ref_time: 500_000_000_000,
    proof_size: 11111111111,
};

impl TransferNFT {
    /// encode payload calldata.
    fn encode_calldata(
        to: PublicAddress,
        token_id: u32,
        function_selector: Selector,
    ) -> Result<Payload<Call>, crate::error::Error> {
        let args = NftTransferAgrs::new(function_selector, to.clone(), token_id);

        let dest = MultiAddress::Id(AccountId32::from(to));

        // unvalidate the payload because the interface most likely won't change but the runtime version will.
        let payload = runtime_types::api::tx()
            .contracts()
            .call(
                dest,
                DEFAULT_TX_VALUE,
                STATIC_GAS_LIMIT,
                None,
                args.encode(),
            )
            .unvalidated();

        Ok(payload)
    }

    /// construct nft transfer extrinsic payload.
    pub fn construct(
        address: PublicAddress,
        to: PublicAddress,
        token_id: u32,
        function_selector: Selector,
    ) -> Result<NftTransferPayload, crate::error::Error> {
        let calldata = Self::encode_calldata(to, token_id, function_selector)?;
        
        Ok(calldata.into())
    }
}

/// NFT transfer extrinsic payload.
pub struct NftTransferPayload(subxt::tx::Payload<Call>);

impl WrappedExtrinsic<Call> for NftTransferPayload {
    fn into_inner(self) -> subxt::tx::Payload<Call> {
        self.0
    }
}

impl From<subxt::tx::Payload<Call>> for NftTransferPayload {
    fn from(value: subxt::tx::Payload<Call>) -> Self {
        Self(value)
    }
}
