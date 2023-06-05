use std::str::FromStr;

use subxt::utils::AccountId32;

use crate::extrinsics::prelude::GenericError;

use super::{calldata::CallData, transfer_nft_contract::types::ContractTransactionPayload};

#[derive(subxt::ext::codec::Encode, subxt::ext::codec::Decode)]
struct GasLimit {
    pub proof_size: u128,
    pub ref_time: u128,
}

const DEFAULT_PROOF_SIZE: u128 = 1_000_000_000;
const DEFAULT_REF_TIME: u128 = 1_000_000_000;

const DEFAULT_TX_VALUE: u128 = 0;
const DEFAULT_DEPOSIT_LIMIT: u128 = 0;

fn gas_limit() -> FieldsCallData {
    let fields = vec![
        ("proof_size", DEFAULT_PROOF_SIZE),
        ("ref_time", DEFAULT_REF_TIME),
    ];

    FieldsCallData::from(fields)
}

use subxt::dynamic::Value as FieldsCallData;

pub trait ToContractPayload {
    fn to_payload(self, address: &str) -> Result<ContractTransactionPayload, GenericError>;
}

impl ToContractPayload for CallData {
    fn to_payload(self, address: &str) -> Result<ContractTransactionPayload, GenericError> {
        // check for address validity
        let _ = AccountId32::from_str(address)?;

        let fields = vec![
            FieldsCallData::from(String::from(address)),
            FieldsCallData::from(DEFAULT_TX_VALUE),
            FieldsCallData::from(gas_limit()),
            FieldsCallData::from(DEFAULT_DEPOSIT_LIMIT),
            FieldsCallData::from_bytes(self.to_vec()),
        ];

        let tx = subxt::tx::dynamic("Contract", "Call", fields);

        Ok(tx)
    }
}
