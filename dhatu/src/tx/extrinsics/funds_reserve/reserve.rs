use crate::error::{Error, FundsReserveError};
use crate::registrar::key_manager::prelude::PublicAddress;
use crate::registrar::signer::WrappedExtrinsic;
use crate::tx::extrinsics::prelude::{
    enums::ExtrinsicStatus, extrinsics::Transaction, transfer_balance::constructor::BalanceTransfer,
};
use crate::types::{MandalaClient, Unit};

use subxt::dynamic::{At, DecodedValueThunk};
use subxt::tx::PairSigner;

use crate::registrar::key_manager::prelude::PrivateKey;

/// funds reserve. used to reserve funds for extrinsics gas fees.
/// intended to use on migration transactions.
#[derive(Clone)]
pub struct FundsReserve {
    reserve: PrivateKey,
    client: MandalaClient,
}

impl FundsReserve {
    /// create a new funds reserve instance
    pub fn new(reserve_key: PrivateKey, client: MandalaClient) -> Self {
        Self {
            reserve: reserve_key,
            client,
        }
    }
}

impl FundsReserve {
    /// get the reserve signer.
    pub fn reserve_signer(&self) -> &PrivateKey {
        &self.reserve
    }

    ///    get the reserve address.
    pub fn reserve_address(&self) -> PublicAddress {
        self.reserve.public_address()
    }

    /// get the client instance.
    pub fn client(&self) -> &MandalaClient {
        &self.client
    }

    /// set a new reserve signer.
    pub fn set_signer(&mut self, signer: PrivateKey) {
        self.reserve = signer;
    }
}

impl FundsReserve {
    const SYSTEM_PALLET: &'static str = "System";
    const SYSTEM_PALLET_ACCOUNT_STORAGE_ENTRY: &'static str = "Account";

    ///  check if the account has enough funds to pay for the transaction.
    pub async fn check_funds(&self, account: PublicAddress, value: Unit) -> Result<bool, Error> {
        let client = self.client().inner_internal();

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

        let value = value.as_u128();
        match account_balance.cmp(&value) {
            std::cmp::Ordering::Less => Ok(false),
            _ => Ok(true),
        }
    }

    /// decode the balance from the storage result.
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
    /// transfer funds to the account.
    pub async fn transfer_funds(
        &self,
        account: PublicAddress,
        value: Unit,
    ) -> Result<ExtrinsicStatus, Error> {
        let client = self.client().inner_internal();

        let signer = PairSigner::new(self.reserve_signer().0.to_owned());

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
    /// check if the account has enough funds to pay for the transaction, then transfer the funds if it's not enough.
    ///
    /// will return `None` if the funds is sufficient
    ///
    /// `threshold` : the account balance quotas to compare against
    ///
    ///
    /// `value` : what the transaction will cost
    pub async fn check_and_transfer(
        &self,
        account: PublicAddress,
        threshold: Unit,
        value: Unit,
    ) -> Result<Option<ExtrinsicStatus>, Error> {
        let is_balance_enough = self.check_funds(account.clone(), threshold).await?;

        match is_balance_enough {
            false => Ok(Some(self.transfer_funds(account, value).await?)),
            true => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::registrar::key_manager::KeyManager;

    use super::*;

    async fn mock_funds_reserve() -> FundsReserve {
        let private_key = PrivateKey::from(sp_keyring::Sr25519Keyring::Bob.pair());
        let client = MandalaClient::dev().await.unwrap();
        FundsReserve::new(private_key, client)
    }

    async fn mock_insufficient_address() -> PublicAddress {
        KeyManager::new_without_password()
            .keypair()
            .to_owned()
            .into()
    }

    fn mock_sufficient_address() -> PublicAddress {
        PublicAddress::from(sp_keyring::Sr25519Keyring::Alice.pair())
    }

    #[tokio::test]
    async fn test_check_funds_enough_balance() {
        let value = Unit::new("0.1", None).expect("static conversion should not fail");

        let result = mock_funds_reserve()
            .await
            .check_funds(mock_sufficient_address(), value)
            .await;

        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_check_funds_insufficient_balance() {
        let value = Unit::new("0.1", None).expect("static conversion should not fail");

        let address = mock_insufficient_address().await;

        let result = mock_funds_reserve()
            .await
            .transfer_funds(address.clone(), value)
            .await
            .unwrap();

        let value = Unit::new("1", None).expect("static conversion should not fail");

        let result = mock_funds_reserve().await.check_funds(address, value).await;

        let result = result.unwrap();
        assert_eq!(result, false)
    }

    #[tokio::test]
    async fn test_transfer_funds_success() {
        let value = Unit::new("0.1", None).expect("static conversion should not fail");

        let result = mock_funds_reserve()
            .await
            .transfer_funds(mock_insufficient_address().await, value)
            .await
            .unwrap();

        match &result {
            ExtrinsicStatus::Pending => println!("transaction is pending"),
            ExtrinsicStatus::Failed(_) => panic!(),
            ExtrinsicStatus::Success(res) => {
                let hash_str = res.hash();
                println!("{:?}", hash_str);
            }
        }
    }

    #[tokio::test]
    async fn test_check_and_transfer_sufficient() {
        let threshold = Unit::new("4500", None).expect("static conversion should not fail");
        let value = Unit::new("5000", None).expect("static conversion should not fail");

        let result = mock_funds_reserve()
            .await
            .check_and_transfer(mock_sufficient_address(), threshold, value)
            .await;

        let result = result.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_check_and_transfer_if_insufficient() {
        let value = Unit::new("0.1", None).expect("static conversion should not fail");

        let address = mock_insufficient_address().await;

        let result = mock_funds_reserve()
            .await
            .transfer_funds(address.clone(), value)
            .await
            .unwrap();

        let threshold = Unit::new("4500", None).expect("static conversion should not fail");
        let value = Unit::new("5000", None).expect("static conversion should not fail");

        let result = mock_funds_reserve()
            .await
            .check_and_transfer(address, threshold, value)
            .await;

        let result = result.unwrap();
        assert!(result.is_some());
    }
}
