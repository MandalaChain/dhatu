use std::str::FromStr;

use subxt::{
    ext::scale_value::Composite,
    tx::DynamicTxPayload,
    utils::{AccountId32, MultiAddress},
    OnlineClient, PolkadotConfig,
};

use crate::extrinsics::prelude::{BlockchainClient, GenericError};

use super::{calldata::CallData, transfer_nft_contract::types::ContractTransactionPayload};

const DEFAULT_PROOF_SIZE: u64 = 1_000_000_000;
const DEFAULT_REF_TIME: u64 = 1_000_000_000;

const DEFAULT_TX_VALUE: u128 = 0;
const DEFAULT_DEPOSIT_LIMIT: u128 = 0;

#[derive(subxt::ext::codec::Decode, subxt::ext::codec::Encode, Debug)]
struct GasLimit {
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

#[derive(subxt::ext::codec::Decode, subxt::ext::codec::Encode, Debug)]
struct ContractCall {
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

    fn new_with_arbitrary_args(dest: MultiAddress<AccountId32, ()>, args: CallData) -> Self {
        Self {
            dest: contract_address,
            value: DEFAULT_TX_VALUE,
            gas_limit: GasLimit::default(),
            storage_deposit_limit: None,
            data: args.into(),
        }
    }
}


pub trait ToContractPayload<T> {
    fn to_payload(
        self,
        address: &str,
        client: BlockchainClient,
    ) -> Result<ContractTransactionPayload<T>, GenericError>;
}

impl ToContractPayload for CallData {
    fn to_payload(
        self,
        address: &str,
        client: BlockchainClient,
    ) -> Result<ContractTransactionPayload, GenericError> {
        client.metadata().call_hash(pallet, function);
        let address = MultiAddress::Id(AccountId32::from_str(address)?);

        let fields = ContractCall::new_with_arbitrary_args(address, self);

        let tx = subxt::tx::StaticTxPayload::new(pallet_name, call_name, call_data, validation_hash);

        Ok(tx)
    }
}
