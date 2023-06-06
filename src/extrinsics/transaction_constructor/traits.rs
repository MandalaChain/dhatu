use std::str::FromStr;

use subxt::{
    ext::scale_value::Composite,
    tx::DynamicTxPayload,
    utils::{AccountId32, MultiAddress},
    OnlineClient, PolkadotConfig,
};

use crate::extrinsics::prelude::{BlockchainClient, GenericError};

use super::{
    calldata::CallData,
    transfer_nft_contract::{constructor::TransferNFT, types::ContractTransactionPayload},
};

pub trait ToContractPayload<T = ContractCall> {
    fn to_payload(
        self,
        address: &str,
        client: BlockchainClient,
    ) -> Result<ContractTransactionPayload<T>, GenericError>;
}

pub trait ValidateHash {
    fn call_hash(client: BlockchainClient) -> Option<[u8; 32]> {
        client
            .metadata()
            .call_hash(Self::pallet_name(), Self::function_name())
    }

    fn pallet_name() -> &'static str;

    fn function_name() -> &'static str;
}
