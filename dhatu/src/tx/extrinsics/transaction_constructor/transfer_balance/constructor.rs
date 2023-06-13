use subxt::utils::{AccountId32, MultiAddress};

use crate::{
    registrar::key_manager::prelude::PublicAddress,
    tx::extrinsics::transaction_constructor::traits::ValidateHash,
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
pub(crate) struct BalanceTransferArgs {
    pub(crate) dest: MultiAddress<AccountId32, ()>,
    pub(crate) value: u128,
}

impl BalanceTransferArgs {
    fn new(dest: MultiAddress<AccountId32, ()>, value: u128) -> Self {
        Self { dest, value }
    }
}

pub struct BalanceTransfer;

impl BalanceTransfer {
    fn generate_payload(args: BalanceTransferArgs) -> subxt::tx::Payload<BalanceTransferArgs> {
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

pub struct BalanceTransferPayload(subxt::tx::Payload<BalanceTransferArgs>);

impl BalanceTransferPayload {
    fn new(args: BalanceTransferArgs) -> Self {
        Self(BalanceTransfer::generate_payload(args))
    }

    pub(crate) fn into_inner(self) -> subxt::tx::Payload<BalanceTransferArgs> {
        self.0
    }

    #[cfg(feature = "unstable_sp_core")]
    pub fn inner(&self) -> &subxt::tx::Payload<BalanceTransferArgs> {
        &self.0
    }
}

impl BalanceTransfer {
    pub fn construct(to: PublicAddress, value: u128) -> BalanceTransferPayload {
        let dest = subxt::utils::MultiAddress::Id(to.into());
        let args = BalanceTransferArgs::new(dest, value);
        
        BalanceTransferPayload::new(args)
    }
}
