



use crate::tx::extrinsics::prelude::{};

use super::{
    calldata::{ContractCall},
    transfer_nft_contract::{types::ContractTransactionPayload},
};

pub trait ToContractPayload<T = ContractCall>: ValidateHash {
    fn to_payload(
        self,
        address: &str,
    ) -> Result<ContractTransactionPayload<T>, crate::error::Error>;
}

pub trait ValidateHash {
    fn pallet_name() -> &'static str;

    fn function_name() -> &'static str;
}

pub trait ScaleEncodeable {
    fn encode(self) -> Vec<u8>;
}
