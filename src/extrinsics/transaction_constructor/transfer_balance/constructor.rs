use std::str::FromStr;

use subxt::{
    tx::Payload,
    utils::{AccountId32, MultiAddress},
};

use crate::extrinsics::{
    prelude::{BlockchainClient, GenericError},
    transaction_constructor::traits::ValidateHash,
};

#[derive(
    :: subxt :: ext :: codec :: Decode,
    :: subxt :: ext :: codec :: Encode,
    :: subxt :: ext :: scale_decode :: DecodeAsType,
    :: subxt :: ext :: scale_encode :: EncodeAsType,
    Debug,
)]
#[codec (crate = :: subxt :: ext :: codec)]
#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
pub struct BalanceTransferArgs {
    pub dest: MultiAddress<AccountId32, ()>,
    pub value: u128,
}

impl BalanceTransferArgs {
    fn new(dest: MultiAddress<AccountId32, ()>, value: u128) -> Self {
        Self { dest, value }
    }
}

pub struct BalanceTransfer;

impl BalanceTransfer {
    fn generate_payload(args: BalanceTransferArgs) -> BalanceTransferPayload {
        subxt::tx::Payload::new(Self::pallet_name(), Self::function_name(), args)
    }
}

impl ValidateHash for BalanceTransfer {
    fn pallet_name() -> &'static str {
        "Balances"
    }

    fn function_name() -> &'static str {
        "transfer_keep_alive"
    }
}

type BalanceTransferPayload = subxt::tx::Payload<BalanceTransferArgs>;

impl BalanceTransfer {
    pub fn construct(to: &str, value: u128) -> Result<BalanceTransferPayload, GenericError> {
        let dest = subxt::utils::MultiAddress::Id(AccountId32::from_str(to)?);

        let args = BalanceTransferArgs::new(dest, value);

        let payload = Self::generate_payload(args);

        Ok(payload)
    }
}
