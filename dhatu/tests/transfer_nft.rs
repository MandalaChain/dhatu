mod common;

use dhatu::{
    self,
    registrar::key_manager::keypair::PublicAddress,
    tx::extrinsics::{
        prelude::{
            enums::ExtrinsicStatus::{Failed, Pending, Success},
            extrinsics::Transaction,
            ExtrinsicSubmitter,
        },
        transaction_constructor::{
            calldata::Selector, transfer_nft_contract::constructor::TransferNFT,
        },
    },
};
use mandala_node_runner;

use crate::common::DEFAULT_NFT_TOKEN_ID;

async fn should_transfer_nft() {
    const TRANSFER_FUNCTION_SELECTOR: Selector =
        Selector::from_raw("0x84a15da1").expect("static values are valid");

    let (client, _node) = common::setup_node_and_client().await;
    let contract_address = common::setup_dummy_721_contract(&client).await;

    let alice = sp_keyring::Sr25519Keyring::Alice.pair();
    let bob = sp_keyring::Sr25519Keyring::Bob.pair();

    let _ = common::mint(&client, contract_address.clone(), alice.clone()).await;

    let payload = TransferNFT::construct(
        PublicAddress::from(contract_address),
        PublicAddress::from(bob),
        DEFAULT_NFT_TOKEN_ID,
        TRANSFER_FUNCTION_SELECTOR,
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
        Failed(reason) => panic!("transaction shouldn't failed: {:?}", reason),
        Success(_) => assert!(true),
    }
}
