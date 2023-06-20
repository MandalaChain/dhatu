use dhatu::{self, registrar::key_manager::keypair::PublicAddress, tx, types::MandalaClient};
use mandala_node_runner;
use parity_scale_codec::Encode;
use sp_core::sr25519::Pair;
use subxt::{tx::PairSigner, utils::AccountId32, PolkadotConfig};

use self::test_types::api::{
    contracts::events::Instantiated, runtime_types::sp_weights::weight_v2::Weight,
};
mod test_types;

pub const DEFAULT_NFT_TOKEN_ID: u32 = 0;
pub const STATIC_GAS_LIMIT: Weight = Weight {
    ref_time: u64::MAX,
    proof_size: u64::MAX,
};

pub async fn setup_node_and_client() -> (
    dhatu::types::MandalaClient,
    mandala_node_runner::SubstrateNode,
) {
    let madya_bin_path = std::env::var("MADYA_BIN_PATH")
        .unwrap_or("/mnt/c/Users/ASUS/Documents/GitHub/stuff/madya/target/release".to_string());

    let mut node_builder = mandala_node_runner::SubstrateNodeBuilder::new();

    node_builder.binary_path(madya_bin_path);

    let node = node_builder
        .spawn()
        .expect("should spawn a new development node!");

    let node_url = format!("127.0.0.1:{}", node.ws_port());

    let client = MandalaClient::new(&node_url)
        .await
        .expect("should create a new client instance!");

    (client, node)
}

pub async fn setup_dummy_721_contract(client: &MandalaClient) -> subxt::utils::AccountId32 {
    let contract_code = std::fs::read("tests/common/erc721.wasm").expect("should read wasm file");
    let mut contract_salt = Vec::new();

    for i in 0..32 {
        contract_salt.push(rand::random::<u8>());
    }

    let tx_payload = test_types::api::tx().contracts().instantiate_with_code(
        0,                // transfered value
        STATIC_GAS_LIMIT, // static gas limit
        None,             // storage deposit limit
        contract_code,    // actual contract code
        vec![],           // empty data
        contract_salt,    // contract salt for address derivation
    );

    let signer = sp_keyring::Sr25519Keyring::Alice.pair();
    let signer: PairSigner<PolkadotConfig, sp_core::sr25519::Pair> = PairSigner::new(signer);

    let deploy_result = client
        .inner()
        .tx()
        .sign_and_submit_then_watch_default(&tx_payload, &signer)
        .await
        .expect("should deploy a new dummy contract transaction successfuly! ")
        .wait_for_finalized_success()
        .await
        .expect("should deploy dummy contract successfully");

    let contract = deploy_result
        .find_first::<Instantiated>()
        .expect("should emit instantiated event")
        .expect("should find instantiated event");

    contract.contract
}

pub async fn mint(client: &MandalaClient, address: AccountId32, to: Pair) {
    const MINT_FUNCTION_SELECTOR: &str = "0xcfdd9aa2";

    let contract_arguments = (MINT_FUNCTION_SELECTOR, DEFAULT_NFT_TOKEN_ID).encode();

    let payload = test_types::api::tx().contracts().call(
        subxt::utils::MultiAddress::Id(address),
        0,                // default value to trf to contract
        STATIC_GAS_LIMIT, // static gas limit
        None,
        contract_arguments,
    );
    let signer: PairSigner<PolkadotConfig, Pair> = PairSigner::new(to);

    let tx = client
        .inner()
        .tx()
        .sign_and_submit_then_watch_default(&payload, &signer)
        .await
        .expect("should submit mint transaction successfuly!")
        .wait_for_finalized_success()
        .await
        .expect("should mint successfully!");
}
