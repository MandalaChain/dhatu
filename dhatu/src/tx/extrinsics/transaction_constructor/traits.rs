





use super::{
    calldata::{ContractCall},
    transfer_nft_contract::{},
};

pub(crate) trait ToContractPayload: ValidateHash {
    fn to_payload(
        self,
        address: &str,
    ) -> Result<subxt::tx::Payload<ContractCall>, crate::error::Error>;
}

pub(crate) trait WrappedExtrinsic<T>{
    fn into_inner(self) -> subxt::tx::Payload<T>;
}

pub trait ValidateHash {
    fn pallet_name() -> &'static str;

    fn function_name() -> &'static str;
}

pub trait ScaleEncodeable {
    fn encode(self) -> Vec<u8>;
}
