use std::str::FromStr;

use subxt::{
    ext::scale_value::Composite,
    tx::DynamicTxPayload,
    utils::{AccountId32, MultiAddress},
    OnlineClient, PolkadotConfig,
};

use crate::extrinsics::prelude::{BlockchainClient, GenericError};

use super::{
    calldata::{CallData, ContractCall},
    transfer_nft_contract::{constructor::TransferNFT, types::ContractTransactionPayload},
};

pub trait ToContractPayload<T = ContractCall>: ContractValidateHash {
    fn to_payload(
        self,
        address: &str,
        client: BlockchainClient,
    ) -> Result<ContractTransactionPayload<T>, GenericError>;
}

pub trait ContractValidateHash {
    fn call_hash(client: BlockchainClient) -> [u8; 32] {
        client
            .metadata()
            .call_hash(Self::pallet_name(), Self::function_name())
            .expect("static values must be valid, this should not happen")
    }

    fn pallet_name() -> &'static str {
        "Contract"
    }

    fn function_name() -> &'static str {
        "Call"
    }
}

pub trait ScaleEncodeable {
    fn encode(self) -> Vec<u8>;
}
