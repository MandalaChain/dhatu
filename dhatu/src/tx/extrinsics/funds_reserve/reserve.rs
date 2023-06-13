


use crate::error::Error;
use crate::registrar::key_manager::prelude::PublicAddress;
use crate::registrar::signer::WrappedExtrinsic;
use crate::tx::extrinsics::prelude::{
    enums::ExtrinsicStatus, extrinsics::Transaction,
    transfer_balance::constructor::BalanceTransfer,
};
use crate::types::MandalaClient;


use subxt::dynamic::{At, DecodedValueThunk};
use subxt::{tx::PairSigner};

use crate::{registrar::key_manager::prelude::PrivateKey};

#[derive(thiserror::Error, Debug)]
pub enum FundsReserveError {
    #[error("{0}")]
    RpcError(#[from] subxt::error::Error),

    #[error("account does not exist!")]
    NonExistentAccount,
}

#[derive(Clone)]
pub struct FundsReserve {
    reserve: PrivateKey,
    client: MandalaClient,
}

impl FundsReserve {
    pub fn new(reserve_key: PrivateKey, client: MandalaClient) -> Self {
        Self {
            reserve: reserve_key,
            client,
        }
    }
}

impl FundsReserve {
    pub fn reserve_signer(&self) -> &PrivateKey {
        &self.reserve
    }

    pub fn reserve_address(&self) -> PublicAddress {
        self.reserve.public_key()
    }

    pub fn client(&self) -> &MandalaClient {
        &self.client
    }

    pub fn set_signer(&mut self, signer: PrivateKey) {
        self.reserve = signer;
    }
}

impl FundsReserve {
    const SYSTEM_PALLET: &'static str = "System";
    const SYSTEM_PALLET_ACCOUNT_STORAGE_ENTRY: &'static str = "Account";

    pub async fn check_funds(&self, account: PublicAddress, value: u128) -> Result<bool, Error> {
        let client = self.client().inner();

        let address = subxt::dynamic::storage(
            Self::SYSTEM_PALLET,
            Self::SYSTEM_PALLET_ACCOUNT_STORAGE_ENTRY,
            vec![subxt::dynamic::Value::from(account)],
        );

        let result = client
            .storage()
            .at_latest()
            .await
            .map_err(FundsReserveError::RpcError)?
            .fetch(&address)
            .await
            .map_err(FundsReserveError::RpcError)?;

        let account_balance = Self::infer_balance(result)?;

        match account_balance.cmp(&value) {
            std::cmp::Ordering::Less => Ok(false),
            _ => Ok(true),
        }
    }

    fn infer_balance(result: Option<DecodedValueThunk>) -> Result<u128, Error> {
        result
            .ok_or(FundsReserveError::NonExistentAccount.into())
            .map(|v| v.to_value().expect("subxt dynamic values are valid"))
            .map(|acc| {
                acc.at("data")
                    .at("free")
                    .expect("subxt dynamic values are valid")
                    .to_owned()
            })
            .map(|balance| balance.as_u128().expect("subxt dynamic values are valid"))
    }
}

impl FundsReserve {
    pub async fn transfer_funds(
        &self,
        account: PublicAddress,
        value: u128,
    ) -> Result<ExtrinsicStatus, Error> {
        let client = self.client().inner();

        let signer = PairSigner::new(self.reserve_signer().inner().to_owned());

        let payload = BalanceTransfer::construct(account, value).into_inner();

        let tx = client
            .tx()
            .sign_and_submit_then_watch_default(&payload, &signer)
            .await
            .map_err(FundsReserveError::RpcError)?;

        let status = Transaction::wait(tx.into()).await;

        Ok(status)
    }
}

impl FundsReserve {
    // threshold should be the account balance quotas to compare against,
    // value should be what the transaction will cost
    pub async fn check_and_transfer(
        &self,
        account: PublicAddress,
        threshold: u128,
        value: u128,
    ) -> Result<Option<ExtrinsicStatus>, Error> {
        let is_balance_low = self.check_funds(account.clone(), threshold).await?;

        match is_balance_low {
            true => Ok(Some(self.transfer_funds(account, value).await?)),
            false => Ok(None),
        }
    }
}
