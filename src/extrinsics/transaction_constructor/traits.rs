use std::str::FromStr;

use subxt::utils::AccountId32;

use crate::extrinsics::prelude::GenericError;

use super::{calldata::CallData, transfer_nft_contract::types::ContractTransactionPayload};

#[derive(subxt::ext::codec::Encode, subxt::ext::codec::Decode)]
struct GasLimit {
    pub proof_size: u128,
    pub ref_time: u128,
}

const STATIC_GAS_LIMIT: GasLimit = GasLimit {
    proof_size: 1_000_000_000,
    ref_time: 1_000_000_000,
};

const DEFAULT_TX_VALUE: u128 = 0;
const DEFAULT_DEPOSIT_LIMIT: u128 = 0;

use subxt::dynamic::Value as FieldsCallData;

pub trait ToContractPayload {
    fn to_payload(self, address: &str) -> Result<ContractTransactionPayload, GenericError>;
}

impl ToContractPayload for CallData {
    fn to_payload(self, address: &str) -> Result<ContractTransactionPayload, GenericError> {
        let dest = subxt::utils::MultiAddress::Id(AccountId32::from_str(address)?);

        let fields = vec![
            FieldsCallData::from_bytes(dest),
            FieldsCallData::from(DEFAULT_TX_VALUE),
            FieldsCallData::from(STATIC_GAS_LIMIT),
            FieldsCallData::from(DEFAULT_DEPOSIT_LIMIT),
            FieldsCallData::from_bytes(self.to_vec()),
        ];

        let tx = subxt::tx::dynamic("Contract", "Call", fields);

        Ok(tx)
    }
}
