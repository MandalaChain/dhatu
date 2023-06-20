use dhatu::{self, types::MandalaClient};
use mandala_node_runner;
use subxt::{tx::PairSigner, PolkadotConfig};

use self::test_types::api::contracts::events::Instantiated;
mod test_types;

pub fn setup_node_and_client() -> (
    dhatu::types::MandalaClient,
    mandala_node_runner::SubstrateNode,
) {
    let madya_bin_path = std::env::var("MADYA_BIN_PATH")
        .expect("madya node binary path (MADYA_BIN_PATH) must be set!");
    let mut node = mandala_node_runner::SubstrateNodeBuilder::new();

    let node = node
        .binary_path(madya_bin_path)
        .spawn()
        .expect("should spawn a new development node!");

    let node_url = format!("127.0.0.1:{}", node.ws_port());
}

pub async fn setup_dummy_721_contract(client: MandalaClient) {
    
    let tx_payload = test_types::api::tx().contracts().instantiate_with_code(
        0 , // transfered value
        20_000_000_000, // static gas limit
        None, // storage deposit limit 
        code,
        data,
        salt,
    );

    let signer = sp_keyring::Sr25519Keyring::Alice.pair();
    let signer: PairSigner<PolkadotConfig, sp_core::sr25519::Pair> = PairSigner::new(signer);

    let deploy_result = client
        .inner()
        .tx()
        .sign_and_submit_then_watch_default(call, &signer)
        .await
        .expect("should deploy a new dummy contract transaction successfuly! ")
        .wait_for_finalized_success()
        .await
        .expect("should deploy dummy contract successfully");

    let contract = deploy_result
        .find_first::<Instantiated>()
        .expect("should emit instantiated event")
        .expect("should find instantiated event");


}
