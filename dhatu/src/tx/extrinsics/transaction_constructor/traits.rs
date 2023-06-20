





use crate::registrar::key_manager::prelude::PublicAddress;

use super::{
    calldata::{ContractCall},
};

/// private traits. should not be exposed to the user.
/// 
/// used to encode calldata into a pallet contracts call arguments payload.
pub(crate) trait ToContractPayload: ValidateHash {
    fn to_payload(
        self,
        address: PublicAddress,
    ) -> Result<subxt::tx::Payload<ContractCall>, crate::error::Error>;
}

/// traits used to mark and properly encode arbitrary calldata into a pallet function calldata payload.
pub trait ValidateHash {
    /// get the pallet name.
    fn pallet_name() -> &'static str;

    /// get the function name.
    fn function_name() -> &'static str;
}

/// traits used to mark and properly encode rust data structure into 
/// a scale encoded byte array.
pub trait ScaleEncodeable {
    fn encode(self) -> Vec<u8>;
}
