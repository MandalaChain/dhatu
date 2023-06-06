use std::{marker::PhantomData, str::FromStr};
use subxt::utils::{AccountId32, MultiAddress};

use crate::extrinsics::prelude::{BlockchainClient, GenericError};

use super::{
    traits::{ScaleEncodeable, ToContractPayload, ValidateHash},
    transfer_nft_contract::{
        constructor::{NftTransferAgrs, TransferNFT},
        types::ContractTransactionPayload,
    },
};

pub struct CallData<T = ()>(Vec<u8>, PhantomData<T>);

impl CallData {
    pub fn get(&self) -> &Vec<u8> {
        self.0.as_ref()
    }

    pub fn to_vec(self) -> Vec<u8> {
        self.0
    }
}

impl From<Vec<u8>> for CallData {
    fn from(val: Vec<u8>) -> Self {
        CallData(val, PhantomData)
    }
}

impl<T> From<CallData<T>> for Vec<u8> {
    fn from(value: CallData<T>) -> Self {
        value.0
    }
}

impl From<NftTransferAgrs> for CallData<TransferNFT> {
    fn from(value: NftTransferAgrs) -> Self {
        CallData(value.encode(), PhantomData)
    }
}

impl<T> ToContractPayload for CallData<T> {
    fn to_payload(self, address: &str) -> Result<ContractTransactionPayload, GenericError> {
        let address = MultiAddress::Id(AccountId32::from_str(address)?);

        let args = ContractCall::new_with_arbitrary_args(address, self);

        let tx = subxt::tx::Payload::new(Self::pallet_name(), Self::function_name(), args);

        Ok(tx)
    }
}

impl<T> ValidateHash for CallData<T> {
    fn pallet_name() -> &'static str {
        "Contract"
    }

    fn function_name() -> &'static str {
        "call"
    }
}

//

const DEFAULT_PROOF_SIZE: u64 = 1_000_000_000;
const DEFAULT_REF_TIME: u64 = 1_000_000_000;

const DEFAULT_TX_VALUE: u128 = 0;
const DEFAULT_DEPOSIT_LIMIT: u128 = 0;

#[derive(
    :: subxt :: ext :: codec :: Decode,
    :: subxt :: ext :: codec :: Encode,
    :: subxt :: ext :: scale_decode :: DecodeAsType,
    :: subxt :: ext :: scale_encode :: EncodeAsType,
    Debug,
)]
#[codec (crate = :: subxt :: ext :: codec)]
#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
pub struct GasLimit {
    pub ref_time: u64,
    pub proof_size: u64,
}

impl Default for GasLimit {
    fn default() -> Self {
        Self {
            proof_size: DEFAULT_PROOF_SIZE,
            ref_time: DEFAULT_REF_TIME,
        }
    }
}

#[derive(
    :: subxt :: ext :: codec :: Decode,
    :: subxt :: ext :: codec :: Encode,
    :: subxt :: ext :: scale_decode :: DecodeAsType,
    :: subxt :: ext :: scale_encode :: EncodeAsType,
    Debug,
)]
#[codec (crate = :: subxt :: ext :: codec)]
#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
pub struct ContractCall {
    pub dest: MultiAddress<AccountId32, ()>,
    pub value: u128,
    pub gas_limit: GasLimit,
    pub storage_deposit_limit: Option<u128>,
    pub data: Vec<u8>,
}

impl ContractCall {
    fn new(
        dest: MultiAddress<AccountId32, ()>,
        value: u128,
        gas_limit: GasLimit,
        storage_deposit_limit: Option<u128>,
        data: Vec<u8>,
    ) -> Self {
        Self {
            dest,
            value,
            gas_limit,
            storage_deposit_limit,
            data,
        }
    }

    fn new_with_arbitrary_args<T>(dest: MultiAddress<AccountId32, ()>, args: CallData<T>) -> Self {
        Self {
            dest,
            value: DEFAULT_TX_VALUE,
            gas_limit: GasLimit::default(),
            storage_deposit_limit: None,
            data: args.into(),
        }
    }
}
