use std::{str::FromStr, time::Duration};

mod common;

use dhatu::{
    tx::extrinsics::prelude::{extrinsics::TransactionMessage, facade::ExtrinsicFacade, Url},
    types::{InternalChannels, MandalaExtrinsics},
};

#[ignore]
#[tokio::test]
async fn should_track_and_watch_extrinsics() {
    let mut server = mockito::Server::new();

    let mock = server.mock("POST", "/").create();

    let blockchain_client = common::setup_node_and_client().await;

    let channels = InternalChannels::<TransactionMessage>::new();

    let facade = ExtrinsicFacade::new(channels);

    //  example transaction
    let tx = common::get_dummy_deploy_contract_payload();
    let signer = common::mock_alice_pair();
    let tx = common::initiate_deploy_contract_txs(&blockchain_client, tx, signer).await;

    let callback = Url::from_str(&server.url()).expect("static string is valid url");
    println!("callback url: {}", callback.to_string());

    let result = facade.submit(tx.into(), Some(callback)).await.unwrap();

    loop {
        println!("checking transaction status");
        let status = facade.watcher().check(&result).await;

        if status.is_none() {
            mock.assert();
            break;
        } else if let Some(status) = status {
            println!("transaction status: {:?}", status);
        }

        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}
