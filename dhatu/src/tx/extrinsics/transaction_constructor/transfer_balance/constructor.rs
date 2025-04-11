use subxt::utils::{AccountId32, MultiAddress};

use crate::{
    registrar::{key_manager::prelude::PublicAddress, signer::WrappedExtrinsic},
    tx::extrinsics::transaction_constructor::traits::ValidateHash,
    types::Unit,
};

// pallet balance transfer function arguments
#[doc(hidden)]
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
    pub(crate) dest: MultiAddress<AccountId32, ()>,
    pub(crate) value: u128,
}

impl BalanceTransferArgs {
    fn new(dest: MultiAddress<AccountId32, ()>, value: u128) -> Self {
        Self { dest, value }
    }
}

/// Balance transfer extrinsic constructor.
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

/// Balance transfer extrinsic payload.
pub struct BalanceTransferPayload(subxt::tx::Payload<BalanceTransferArgs>);

impl WrappedExtrinsic<BalanceTransferArgs> for BalanceTransferPayload {
    fn into_inner(self) -> subxt::tx::Payload<BalanceTransferArgs> {
        self.0
    }
}

impl BalanceTransferPayload {
    /// construct a balance transfer extrinsic payload
    fn new(args: BalanceTransferArgs) -> Self {
        Self(BalanceTransfer::generate_payload(args))
    }

    /// get the inner payload
    #[cfg(feature = "unstable_sp_core")]
    pub fn inner(&self) -> &subxt::tx::Payload<BalanceTransferArgs> {
        &self.0
    }
}

impl BalanceTransfer {
    /// construct a balance transfer extrinsic payload
    pub fn construct(to: PublicAddress, value: Unit) -> BalanceTransferPayload {
        let dest = subxt::utils::MultiAddress::Id(to.into());
        let args = BalanceTransferArgs::new(dest, value.as_u128());

        BalanceTransferPayload::new(args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use sp_keyring::sr25519::sr25519;
    use std::str::FromStr;
    use subxt::ext::sp_core::{crypto::Ss58Codec, Pair};

    fn mock_pair() -> sr25519::Pair {
        sp_keyring::Sr25519Keyring::Alice.pair()
    }

    fn mock_id() -> AccountId32 {
        let pair = mock_pair();
        let pair_public = pair.public();
        AccountId32::from_str(&pair_public.to_ss58check()).unwrap()
    }

    #[test]
    fn test_balance_transfer_args_new() {
        let id = mock_id();
        let dest = MultiAddress::Id(id.clone());
        let value = 100;

        let args = BalanceTransferArgs::new(dest, value);

        assert_eq!(args.dest, MultiAddress::Id(id));
        assert_eq!(args.value, 100);
    }

    #[test]
    fn test_balance_transfer_generate_payload() {
        let id = mock_id();
        let dest = MultiAddress::Id(id);
        let value = 100;
        let args = BalanceTransferArgs::new(dest.clone(), value);

        let payload = BalanceTransfer::generate_payload(args);

        assert_eq!(payload.call_data().dest, dest);
        assert_eq!(payload.call_data().value, value);
    }

    #[test]
    fn test_balance_transfer_construct() {
        let to = PublicAddress::from(mock_pair());
        let value = Unit::new("0.1", None).expect("static values are valid");

        let payload = BalanceTransfer::construct(to, value.clone());

        assert_eq!(payload.0.call_data().dest, MultiAddress::Id(mock_id()));
        assert_eq!(payload.0.call_data().value, value.as_u128());
    }
}
