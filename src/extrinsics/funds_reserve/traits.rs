use std::{pin::Pin, str::FromStr};

use futures::Future;
use sp_core::sr25519::Pair;
use subxt::{tx::PairSigner, utils::AccountId32};

use crate::extrinsics::prelude::{
    enums::ExtrinsicStatus, extrinsics::Transaction, BlockchainClient, GenericError,
};

// TODO : make this a macro

pub trait FundsReserveAtributes {
    fn reserve_signer(&self) -> &sp_core::sr25519::Pair;

    fn reserve_adress(&self) -> String;

    fn client(&self) -> &BlockchainClient;

    fn change_signer(&mut self, pair: Pair);
}

pub trait FundsReserveTask: FundsReserveAtributes {
    fn check_funds(
        &self,
        account: &str,
        value: u128,
    ) -> Pin<Box<dyn Future<Output = Result<bool, GenericError>>>> {
        // we need this outside of the async block to avoid lifetime's issues`
        let client = self.client().clone();
        let account = String::from(account);

        let task = async move {
            let account = AccountId32::from_str(&account)?;
            let address = runtime_types::api::storage().system().account(account);
            let result = client
                .storage()
                .at(None)
                .await
                .unwrap()
                .fetch(&address)
                .await?;

            let account = result.unwrap();
            let account_balance = account.data.free;

            match account_balance.cmp(&value) {
                std::cmp::Ordering::Less => Ok(false),
                _ => Ok(true),
            }
        };

        Box::pin(task)
    }

    fn transfer_funds(
        &self,
        account: &str,
        value: u128,
    ) -> Pin<Box<dyn Future<Output = Result<ExtrinsicStatus, GenericError>> + Send>> {
        // we need this outside of the async block to avoid lifetime's issues
        let client = self.client().clone();
        let signer = PairSigner::new(self.reserve_signer().to_owned());
        let account = String::from(account);

        let task = async move {
            let dest = subxt::utils::MultiAddress::Id(AccountId32::from_str(&account)?);
            let payload = runtime_types::api::tx()
                .balances()
                .transfer_keep_alive(dest, value);

            let tx = client
                .tx()
                .sign_and_submit_then_watch_default(&payload, &signer)
                .await?;

            let status = Transaction::wait(tx).await;

            Ok(status)
        };

        Box::pin(task)
    }

    // threshold should be the account balance quotas to compare against,
    // value should be what the transaction will cost
    fn check_and_transfer(
        &self,
        account: &str,
        threshold: u128,
        value: u128,
    ) -> Pin<Box<dyn Future<Output = Result<Option<ExtrinsicStatus>, GenericError>>>> {
        let check_balance = self.check_funds(account, threshold);
        let transfer = self.transfer_funds(account, value);

        let task = async move {
            let balance_result = check_balance.await?;

            match balance_result {
                true => Ok(Some(transfer.await?)),
                false => Ok(None),
            }
        };

        Box::pin(task)
    }
}

pub trait FundsReserveTraits: FundsReserveAtributes + FundsReserveTask + Clone {}
