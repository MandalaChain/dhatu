mod common;

use std::str::FromStr;

use dhatu::{
    self,
    registrar::key_manager::keypair::PublicAddress,
    tx::{
        dhatu_assets::{facade::DhatuAssetsFacade, traits::Asset},
        extrinsics::{
            prelude::{
                enums::ExtrinsicStatus::{Failed, Pending, Success},
                extrinsics::{Transaction, TransactionMessage},
                reserve::FundsReserve,
                ExtrinsicSubmitter,
            },
            transaction_constructor::{
                calldata::Selector, transfer_nft_contract::constructor::TransferNFT,
            },
        },
    },
    types::InternalChannels,
};
use mandala_node_runner;

use crate::common::DEFAULT_NFT_TOKEN_ID;

// still fail, dont run this!
#[ignore]
#[tokio::test]
async fn should_migrate_asset() {
    let client = common::setup_node_and_client().await;
    let contract_address_native = common::setup_dummy_721_contract(&client).await;
    // TODO : should implement Fron<subxt::utils::AccountId32> for PublicAddress
    let contract_address = PublicAddress::from_str(&contract_address_native.to_string())
        .expect("static values are valid");

    let alice = sp_keyring::Sr25519Keyring::Alice.pair();
    let bob = sp_keyring::Sr25519Keyring::Bob.pair();

    let amount_minted = 3;

    let assets = common::batch_mint(&client, contract_address, alice.clone(), amount_minted).await;

    let reserve = FundsReserve::new(bob.clone().into(), client.clone());

    let facade = DhatuAssetsFacade::new(client);
    let mut channels = InternalChannels::<TransactionMessage>::new();

    let mut recv = channels.get_receiver();
    let notifier = channels.sender().clone();

    facade.migrate(assets, alice.clone().into(), bob.into(), &reserve, notifier);

    // wait till all transactions are completed and fail if one of them fails
    while let Some(tx) = recv.recv().await {
        match tx.inner_status() {
            Pending => panic!("transaction should not be pending"),
            Failed(reason) => panic!("transaction shouldn't failed: {}", reason.inner()),
            Success(res) => println!("transaction success"),
        }
    }
}
