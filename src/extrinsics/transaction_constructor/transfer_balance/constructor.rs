use std::str::FromStr;

use subxt::{
    tx::{DynamicTxPayload, StaticTxPayload},
    utils::{AccountId32, MultiAddress},
};

use crate::extrinsics::{
    prelude::{BlockchainClient, GenericError},
    transaction_constructor::traits::ValidateHash,
};

#[derive(subxt::ext::codec::Encode, subxt::ext::codec::Decode)]
struct BalanceTransferArgs {
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
    fn generate_payload(
        client: BlockchainClient,
        args: BalanceTransferArgs,
    ) -> BalanceTransferPayload {
        subxt::tx::StaticTxPayload::new(
            Self::pallet_name(),
            Self::function_name(),
            args,
            Self::call_hash(client),
        )
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

type BalanceTransferPayload = StaticTxPayload<BalanceTransferArgs>;

impl BalanceTransfer {
    pub fn construct(
        to: &str,
        value: u128,
        client: BlockchainClient,
    ) -> Result<BalanceTransferPayload, GenericError> {
        let dest = subxt::utils::MultiAddress::Id(AccountId32::from_str(to)?);

        let args = BalanceTransferArgs::new(dest, value);

        let payload = Self::generate_payload(client, args);

        Ok(payload)
    }
}
