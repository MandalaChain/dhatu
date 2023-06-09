use sp_core::Pair as PairTraits;
use std::{pin::Pin, str::FromStr};

use crate::registrar::key_manager::prelude::PublicAddress;
use crate::tx::extrinsics::prelude::{
    enums::ExtrinsicStatus, extrinsics::Transaction,
    transfer_balance::constructor::BalanceTransfer, BlockchainClient, GenericError,
};
use futures::Future;
use sp_core::sr25519::Pair;
use subxt::dynamic::At;
use subxt::{tx::PairSigner, utils::AccountId32};

use crate::{
    registrar::key_manager::prelude::PrivateKey, tx::extrinsics::prelude::BlockchainClient,
    MandalaClient,
};

use super::traits::{FundsReserveAtributes, FundsReserveTask, FundsReserveTraits};

#[derive(Clone)]
pub struct FundsReserve {
    reserve: sp_core::sr25519::Pair,
    client: BlockchainClient,
}

impl FundsReserve {
    pub fn new(reserve_key: PrivateKey, client: MandalaClient) -> Self {
        Self {
            reserve: reserve_key.into(),
            client: client.inner().to_owned(),
        }
    }
}

impl FundsReserve {
    pub fn reserve_signer(&self) -> &sp_core::sr25519::Pair {
        &self.reserve
    }

    pub fn reserve_adress(&self) -> String {
        self.reserve.public().to_string()
    }

    pub fn client(&self) -> &BlockchainClient {
        &self.client
    }

    pub fn change_signer(&mut self, pair: sp_core::sr25519::Pair) {
        self.reserve = pair;
    }
}

impl FundsReserve {
    pub async fn check_funds(&self, account: PublicAddress, value: u128) -> Result<bool, GenericError> {
        // we need this outside of the async block to avoid lifetime's issues`
        let client = self.client().clone();
        let account = String::from(account);

        let account = AccountId32::from_str(&account)?;
        let account = subxt::dynamic::Value::from_bytes(account);

        let address = subxt::dynamic::storage("System", "Account", vec![account]);
        let result = client.storage().at_latest().await?.fetch(&address).await?;

        let account = result.unwrap().to_value()?;
        let account_balance = account.at("data").at("free").unwrap().to_owned();
        let account_balance = account_balance.as_u128().unwrap();

        match account_balance.cmp(&value) {
            std::cmp::Ordering::Less => Ok(false),
            _ => Ok(true),
        }
    }
}

impl FundsReserve {
    pub async fn transfer_funds(
        &self,
        account: String,
        value: u128,
    ) -> Result<ExtrinsicStatus, GenericError> {
        // we need this outside of the async block to avoid lifetime's issues
        let client = self.client().clone();
        let signer = PairSigner::new(self.reserve_signer().to_owned());

        let payload = BalanceTransfer::construct(&account, value)?;

        let tx = client
            .tx()
            .sign_and_submit_then_watch_default(&payload, &signer)
            .await?;

        let status = Transaction::wait(tx).await;

        Ok(status)
    }
}

impl FundsReserve {
    // threshold should be the account balance quotas to compare against,
    // value should be what the transaction will cost
    pub async fn check_and_transfer(
        &self,
        account: String,
        threshold: u128,
        value: u128,
    ) -> Result<Option<ExtrinsicStatus>, GenericError> {
        let check_balance = self.check_funds(&account, threshold);
        let transfer = self.transfer_funds(account, value);

        let balance_result = check_balance.await?;

        match balance_result {
            true => Ok(Some(transfer.await?)),
            false => Ok(None),
        }
    }
}
