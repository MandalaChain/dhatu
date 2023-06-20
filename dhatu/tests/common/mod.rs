use dhatu::{self, types::MandalaClient};
use mandala_node_runner;
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

pub async fn setup_dummy_721_contract(client:MandalaClient) {
    let tx_payload = test_types::api::tx().contracts().instantiate_with_code(
        value,
        gas_limit,
        storage_deposit_limit,
        code,
        data,
        salt,
    );

    client.


}
