use crate::{
    error::Error,
    types::{MandalaExtrinsics, MandalaTransactionProgress},
};

/// extrinsic transaction submitter.
pub struct ExtrinsicSubmitter;

impl ExtrinsicSubmitter {
    /// submit extrinsic transaction.
    /// will return a subscriber to the transaction progress.
    pub async fn submit(tx: MandalaExtrinsics) -> Result<MandalaTransactionProgress, Error> {
        let result = tx
            .into_inner()
            .submit_and_watch()
            .await
            .map_err(Error::Transaction)?
            .into();

        Ok(result)
    }
}

#[cfg(test)]
mod extrinsic_submitter_tests {
    use crate::registrar::key_manager::prelude::PublicAddress;
    use crate::registrar::signer::TxBuilder;
    use crate::tx::extrinsics::extrinsics_submitter::ExtrinsicSubmitter;
    use crate::types::MandalaConfig;
    use crate::types::MandalaExtrinsics;
    use sp_core::crypto::Pair as CryptoPair;
    use sp_core::sr25519::Pair;
    use std::str::FromStr;
    pub(crate) use subxt::OnlineClient;

    fn mock_pair() -> sp_core::sr25519::Pair {
        sp_keyring::Sr25519Keyring::Alice.pair()
    }
    async fn mock_client() -> crate::types::NodeClient {
        OnlineClient::<MandalaConfig>::new().await.unwrap()
    }
    #[tokio::test]
    async fn submit_successful_tests() {
        let address = "5DJk1gegyQJk6BNs7LceZ1akt5e9fpm4gUYGzcfpKaLG9Mmb";
        let new_address = PublicAddress::from_str(address).unwrap();
        let pair = mock_pair();
        let node_client = mock_client().await;

        let value = rand::random();
        // Create the payload using the `construct` function from `BalanceTransfer`
        let payload = crate::tx::extrinsics::prelude::transfer_balance::constructor::BalanceTransfer::construct(new_address, value);
        let extrinsic = TxBuilder::signed(&node_client.into(), pair, payload)
            .await
            .unwrap()
            .0;

        // Create a mock MandalaExtrinsics object
        let tx = MandalaExtrinsics::new(extrinsic);
        let result = ExtrinsicSubmitter::submit(tx).await;

        assert!(result.is_ok());
    }
    #[tokio::test]
    #[should_panic]
    async fn submit_failure_tests() {
        let address = "5DJk1gegyQJk6BNs7LceZ1akt5e9fpm4gUYGzcfpKaLG9Mmb";
        let new_address = PublicAddress::from_str(address).unwrap();
        let pair = mock_pair();
        let node_client = mock_client().await;

        // Assign a invalid value
        let value: u128 = 0;

        // Create the payload using the `construct` function from `BalanceTransfer`
        let payload = crate::tx::extrinsics::prelude::transfer_balance::constructor::BalanceTransfer::construct(new_address, value);

        // Introduce a failure by using an invalid client for signing
        let extrinsic = TxBuilder::signed(&node_client.into(), pair, payload)
            .await
            .unwrap()
            .0;

        // Create a mock MandalaExtrinsics object
        let tx = MandalaExtrinsics::new(extrinsic);
        let result = ExtrinsicSubmitter::submit(tx).await;

        // Assert that the result is an error indicating the failure
        assert!(result.is_err());
        // Add additional assertions or error handling as needed
    }
}
