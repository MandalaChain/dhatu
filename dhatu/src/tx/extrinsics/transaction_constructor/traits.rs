





use super::{
    calldata::{ContractCall},
};

pub(crate) trait ToContractPayload: ValidateHash {
    fn to_payload(
        self,
        address: &str,
    ) -> Result<subxt::tx::Payload<ContractCall>, crate::error::Error>;
}

pub trait ValidateHash {
    fn pallet_name() -> &'static str;

    fn function_name() -> &'static str;
}

pub trait ScaleEncodeable {
    fn encode(self) -> Vec<u8>;
}
