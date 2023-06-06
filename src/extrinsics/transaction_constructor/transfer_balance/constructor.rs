use std::str::FromStr;

use subxt::{tx::DynamicTxPayload, utils::AccountId32};

use crate::extrinsics::prelude::GenericError;

pub struct BalanceTransfer;

type BalanceTransferPayload = DynamicTxPayload<'static>;

impl BalanceTransfer {
    pub fn construct(to: &str, value: u128) -> Result<BalanceTransferPayload, GenericError> {
        let dest = subxt::utils::MultiAddress::Id(AccountId32::from_str(to)?);

        let payload = subxt::dynamic::tx("Balances", call_name, fields);

        let payload = runtime_types::api::tx()
            .balances()
            .transfer_keep_alive(dest, value);

        Ok(payload)
    }
}
