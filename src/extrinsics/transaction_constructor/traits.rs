use std::str::FromStr;


use subxt::{utils::AccountId32};

use crate::extrinsics::prelude::GenericError;

use super::{ calldata::CallData, transfer_nft_contract::types::ContractTransactionPayload};


pub trait ToContractPayload {
    fn to_payload(self, address: &str) -> Result<ContractTransactionPayload, GenericError>;
}

impl ToContractPayload for CallData {
    fn to_payload(self, address: &str) -> Result<ContractTransactionPayload, GenericError> {
        let dest = subxt::utils::MultiAddress::Id(AccountId32::from_str(address)?);
        let value = Default::default();

        let storage_deposit_limit = Default::default();
        // set it as max, ideally we want to estimate the gas limit
        let gas_limit = runtime_types::api::runtime_types::sp_weights::weight_v2::Weight {
            proof_size: 1_000_000_000,
            ref_time: 1_000_000_000,
        };

        // create tx payload
        let tx = runtime_types::api::tx().contracts().call(
            dest,
            value,
            gas_limit,
            storage_deposit_limit,
            self.into(),
        );

        Ok(tx)
    }
}
