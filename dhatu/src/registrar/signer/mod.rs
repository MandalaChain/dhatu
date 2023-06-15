/// represent a arbitrary Transaction Signer.
/// all transactions and payload are signed in byte format
/// with a struct wrapper for each transaction type and payload.
/// this enables us to easily treat the payload as a blackbox.
use sp_core::sr25519::Pair;
use subxt::{
    ext::scale_encode::EncodeAsFields,
    tx::{PairSigner},
};

use crate::types::Extrinsic;

pub(crate) trait WrappedExtrinsic<T: EncodeAsFields> {
    fn into_inner(self) -> subxt::tx::Payload<T>;
}

pub(crate) struct TxBuilder;

impl TxBuilder {
    /// create a new unsigned transaction from a transaction payload
    pub fn unsigned<T: EncodeAsFields>(
        client: &crate::types::NodeClient,
        payload: impl WrappedExtrinsic<T>,
    ) -> Result<Extrinsic, crate::error::Error> {
        Ok(client.tx().create_unsigned(&payload.into_inner())?)
    }

    /// create a new signed transaction given a transaction payload
    pub async fn signed<T: EncodeAsFields>(
        client: &crate::types::NodeClient,
        acc: Pair,
        payload: impl WrappedExtrinsic<T>,
    ) -> Result<Extrinsic, crate::error::Error> {
        let signer = PairSigner::new(acc);

        let tx = client
            .tx()
            .create_signed(&payload.into_inner(), &signer, Default::default())
            .await?;

        Ok(tx)
    }
}

// #[cfg(test)]
// mod tests {
//     use std::str::FromStr;

//     use sp_core::{crypto::Ss58Codec, Pair};
//     use subxt::{
//         rpc::types::DryRunError,
//         utils::{AccountId32, MultiAddress},
//     };

//     use super::*;

//     async fn mock_client() -> crate::types::NodeClient {
//         OnlineClient::<PolkadotConfig>::new().await.unwrap()
//     }

//     fn mock_payload(
//         client: &crate::types::NodeClient,
//     ) -> subxt::tx::StaticTxPayload<runtime_types::api::balances::calls::Transfer> {
//         let _metadata = client.metadata();

//         let dest = mock_acc();

//         runtime_types::api::tx().balances().transfer(dest, 0)
//     }

//     fn mock_acc() -> MultiAddress<AccountId32, ()> {
//         let (dest, _) = sp_core::sr25519::Pair::generate();
//         let dest = dest.public();
//         let dest = AccountId32::from_str(&dest.to_ss58check()).unwrap();

//         subxt::utils::MultiAddress::Id(dest)
//     }

//     fn mock_pair() -> sp_core::sr25519::Pair {
//         let (pair, _) = sp_core::sr25519::Pair::generate();

//         pair
//     }

//     #[actix::test]
//     async fn should_create_unsigned_tx() {
//         let node_client = mock_client().await;
//         let payload = mock_payload(&node_client);

//         let extrinsic = TxBuilder::unsigned(&node_client, &payload).unwrap();

//         let dry_run_result = extrinsic.dry_run(None).await.unwrap();
//         let actual_result = extrinsic.submit().await;

//         // should error because the transaction is unsigned and can only be
//         // submitted through OCW
//         // but it should be possible to include the transaction in the block.
//         // that's why instead of validity erorr it's dispatch error
//         if let Err(dry_run_result) = dry_run_result {
//             assert_eq!(dry_run_result, DryRunError::DispatchError);
//         }

//         if let Err(actual_result) = actual_result {
//             println!("{}", actual_result)
//         }
//     }

//     #[actix::test]
//     async fn should_create_signed_tx() {
//         let node_client = mock_client().await;
//         let payload = mock_payload(&node_client);

//         let pair = mock_pair();
//         let extrinsic = TxBuilder::signed(&node_client, pair, &payload)
//             .await
//             .unwrap();

//         let dry_run_result = extrinsic.dry_run(None).await.unwrap();
//         let actual_result = extrinsic.submit().await;

//         // shoould error because the caller does not have enough balance
//         if let Err(dry_run_result) = dry_run_result {
//             assert_eq!(dry_run_result, DryRunError::TransactionValidityError);
//         }

//         if let Err(actual_result) = actual_result {
//             println!("{}", actual_result)
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use subxt::{PolkadotConfig, SubstrateConfig, error::DispatchError};
    pub(crate) use subxt::OnlineClient;
    use std::str::FromStr;

    use sp_core::{crypto::Ss58Codec, Pair};
    use subxt::{
        utils::{AccountId32, MultiAddress},
        rpc::types::DryRunResult
    };

    use super::*;

    async fn mock_client() -> crate::types::NodeClient {
        OnlineClient::<SubstrateConfig>::new().await.unwrap()
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

        MockWrappedExtrinsic(polkadot::tx().balances().transfer(dest, 10_000))
    }

    fn mock_acc() -> MultiAddress<AccountId32, ()> {
        let (dest, _) = sp_core::sr25519::Pair::generate();
        let dest = dest.public();
        let dest = AccountId32::from_str(&dest.to_ss58check()).unwrap();

        subxt::utils::MultiAddress::Id(dest)
    }

    fn mock_pair() -> sp_core::sr25519::Pair {
        let (pair, _) = sp_core::sr25519::Pair::generate();

        pair
    }

    #[tokio::test]
    async fn should_create_unsigned_tx() {
        let node_client = mock_client().await;
        let payload = mock_payload(&node_client);

        let extrinsic = TxBuilder::unsigned(&node_client, payload).unwrap();

        let dry_run_result: DryRunResult  = extrinsic.dry_run(None).await.unwrap();
        let actual_result = extrinsic.submit().await;

        // should error because the transaction is unsigned and can only be
        // submitted through OCW
        // but it should be possible to include the transaction in the block.
        // that's why instead of validity erorr it's dispatch error
        if let DryRunResult::DispatchError(err) = dry_run_result {
            assert_eq!(
                format!("{:?}", err),
                format!("{:?}", DispatchError::Other)
            );
        }


        if let Err(actual_result) = actual_result {
            println!("{}", actual_result)
        }
    }

    // #[actix::test]
    // async fn should_create_signed_tx() {
    //     let node_client = mock_client().await;
    //     let payload = mock_payload(&node_client);

    //     let pair = mock_pair();
    //     let extrinsic = TxBuilder::signed(&node_client, pair, &payload)
    //         .await
    //         .unwrap();

    //     let dry_run_result = extrinsic.dry_run(None).await.unwrap();
    //     let actual_result = extrinsic.submit().await;

    //     // shoould error because the caller does not have enough balance
    //     if let Err(dry_run_result) = dry_run_result {
    //         assert_eq!(dry_run_result, DryRunError::TransactionValidityError);
    //     }

    //     if let Err(actual_result) = actual_result {
    //         println!("{}", actual_result)
    //     }
    // }
}