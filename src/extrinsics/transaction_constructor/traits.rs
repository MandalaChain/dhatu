use std::str::FromStr;

use subxt::{
    ext::scale_value::Composite,
    utils::{AccountId32, MultiAddress},
    OnlineClient, PolkadotConfig,
};

use crate::extrinsics::prelude::{BlockchainClient, GenericError};

use super::{
    calldata::{CallData, ContractCall},
    transfer_nft_contract::{constructor::TransferNFT, types::ContractTransactionPayload},
};

pub trait ToContractPayload<T = ContractCall>: ValidateHash {
    fn to_payload(
        self,
        address: &str,
    ) -> Result<ContractTransactionPayload<T>, GenericError>;
}

pub trait ValidateHash {
    fn pallet_name() -> &'static str;

    fn function_name() -> &'static str;
}

pub trait ScaleEncodeable {
    fn encode(self) -> Vec<u8>;
}
