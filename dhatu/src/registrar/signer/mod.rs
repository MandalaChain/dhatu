/// represent a arbitrary Transaction Signer.
/// all transactions and payload are signed in byte format
/// with a struct wrapper for each transaction type and payload.
/// this enables us to easily treat the payload as a blackbox.
use sp_core::sr25519::Pair;
use subxt::{ext::scale_encode::EncodeAsFields, tx::PairSigner};

use crate::types::{MandalaExtrinsics};

pub trait WrappedExtrinsic<T: EncodeAsFields> {
    fn into_inner(self) -> subxt::tx::Payload<T>;
}

pub struct TxBuilder;

impl TxBuilder {
    /// create a new unsigned transaction from a transaction payload
    pub fn unsigned<T: EncodeAsFields>(
        client: &crate::types::NodeClient,
        payload: impl WrappedExtrinsic<T>,
    ) -> Result<MandalaExtrinsics, crate::error::Error> {
        Ok(client.tx().create_unsigned(&payload.into_inner())?.into())
    }

    /// create a new signed transaction given a transaction payload
    pub async fn signed<T: EncodeAsFields>(
        client: &crate::types::NodeClient,
        acc: Pair,
        payload: impl WrappedExtrinsic<T>,
    ) -> Result<MandalaExtrinsics, crate::error::Error> {
        let signer = PairSigner::new(acc);

        let tx = client
            .tx()
            .create_signed(&payload.into_inner(), &signer, Default::default())
            .await?
            .into();

        Ok(tx)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    pub(crate) use subxt::OnlineClient;
    use subxt::{error::DispatchError};

    use sp_core::{crypto::Ss58Codec, Pair};
    use subxt::{
        rpc::types::DryRunResult,
        utils::{AccountId32, MultiAddress},
    };

    use crate::types::MandalaConfig;

    use super::*;

    async fn mock_client() -> crate::types::NodeClient {
        OnlineClient::<MandalaConfig>::new().await.unwrap()
    }

    // Generate an interface that we can use from the node's metadata.
    #[subxt::subxt(runtime_metadata_path = "./src/registrar/signer/polkadot_metadata_small.scale")]
    pub mod polkadot {}

    // Mock implementation of `WrappedExtrinsic` for testing
    struct MockWrappedExtrinsic<T: EncodeAsFields>(subxt::tx::Payload<T>);

    impl<T: EncodeAsFields> WrappedExtrinsic<T> for MockWrappedExtrinsic<T> {
        fn into_inner(self) -> subxt::tx::Payload<T> {
            self.0
        }
    }

    fn mock_payload(
        client: &crate::types::NodeClient,
    ) -> MockWrappedExtrinsic<polkadot::balances::calls::types::Transfer> {
        let _metadata = client.metadata();

        let dest = mock_acc();

        MockWrappedExtrinsic(polkadot::tx().balances().transfer(dest, 0))
    }

    fn mock_acc() -> MultiAddress<AccountId32, ()> {
        let dest = sp_keyring::Sr25519Keyring::Bob.pair();
        let dest = dest.public();
        let dest = AccountId32::from_str(&dest.to_ss58check()).unwrap();

        subxt::utils::MultiAddress::Id(dest)
    }

    fn mock_pair() -> sp_core::sr25519::Pair {
        sp_keyring::Sr25519Keyring::Alice.pair()
    }

    #[tokio::test]
    async fn should_create_unsigned_tx() {
        let node_client = mock_client().await;
        let payload = mock_payload(&node_client);

        let extrinsic_result = TxBuilder::unsigned(&node_client, payload);

        assert!(extrinsic_result.is_ok());

        let extrinsic = extrinsic_result.unwrap().0;
        let dry_run_result: DryRunResult = extrinsic.dry_run(None).await.unwrap();
        let actual_result = extrinsic.submit().await;

        // should error because the transaction is unsigned and can only be
        // submitted through OCW
        // but it should be possible to include the transaction in the block.
        // that's why instead of validity erorr it's dispatch error
        if let DryRunResult::DispatchError(err) = dry_run_result {
            assert_eq!(
                format!("{:?}", err),
                format!("{:?}", DispatchError::BadOrigin)
            );
        }

        if let Err(actual_result) = actual_result {
            println!("{}", actual_result)
        }
    }

    #[tokio::test]
    async fn should_create_signed_tx() {
        let node_client = mock_client().await;
        let payload = mock_payload(&node_client);

        let pair = mock_pair();
        let extrinsic = TxBuilder::signed(&node_client, pair, payload)
            .await
            .unwrap()
            .0;

        let _dry_run_result = extrinsic.dry_run(None).await.unwrap();
        let actual_result = extrinsic.submit().await;
        assert!(actual_result.is_ok());
    }
}
