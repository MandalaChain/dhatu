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
                extrinsics::Transaction,
                reserve::FundsReserve,
                ExtrinsicSubmitter,
            },
            transaction_constructor::{
                calldata::Selector, transfer_nft_contract::constructor::TransferNFT,
            },
        },
    },
};
use mandala_node_runner;

use crate::common::DEFAULT_NFT_TOKEN_ID;

#[ignore]
#[tokio::test]
async fn should_transfer_nft() {
    let client = common::setup_node_and_client().await;
    let contract_address_native = common::setup_dummy_721_contract(&client).await;
    // TODO : should implement Fron<subxt::utils::AccountId32> for PublicAddress
    let contract_address = PublicAddress::from_str(&contract_address_native.to_string())
        .expect("static values are valid");

    let alice = sp_keyring::Sr25519Keyring::Alice.pair();
    let bob = sp_keyring::Sr25519Keyring::Bob.pair();

    let _ = common::mint(
        &client,
        contract_address.clone(),
        alice.clone(),
        DEFAULT_NFT_TOKEN_ID,
    )
    .await;

    assert_eq!(
        contract_address.to_string(),
        contract_address_native.to_string()
    );

    println!(
        "contract address : {}",
        contract_address.clone().to_string()
    );

    let transfer_function_selector: Selector =
        Selector::from_raw("84a15da1").expect("static values are valid");

    let payload = TransferNFT::construct(
        contract_address,
        PublicAddress::from(bob),
        DEFAULT_NFT_TOKEN_ID,
        transfer_function_selector,
    )
    .expect("static values are valid");

    let tx = dhatu::registrar::signer::TxBuilder::signed(&client, alice, payload)
        .await
        .expect("static values are valid");

    let progress = ExtrinsicSubmitter::submit(tx)
        .await
        .expect("static values are valid");

    let progress = Transaction::wait(progress).await;

    match progress {
        Pending => panic!("transaction should not be pending"),
        Failed(e) => {
            panic!("transaction shouldn't failed: {}", e.inner());
        }
        Success(_) => assert!(true),
    }
}
