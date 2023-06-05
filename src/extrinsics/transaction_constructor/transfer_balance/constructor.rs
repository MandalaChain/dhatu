use std::str::FromStr;

use subxt::utils::AccountId32;

use crate::extrinsics::prelude::GenericError;

pub struct BalanceTransfer;

type BalanceTransferPayload =
    subxt::tx::StaticTxPayload<runtime_types::api::balances::calls::TransferKeepAlive>;

impl BalanceTransfer {
    pub fn construct(to: &str, value: u128) -> Result<BalanceTransferPayload, GenericError> {
        let dest = subxt::utils::MultiAddress::Id(AccountId32::from_str(to)?);
        let payload = runtime_types::api::tx()
            .balances()
            .transfer_keep_alive(dest, value);

        Ok(payload)
    }
}
