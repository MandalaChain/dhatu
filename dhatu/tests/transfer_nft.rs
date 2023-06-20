mod common;

use dhatu::{
    self, tx::extrinsics::transaction_constructor::transfer_nft_contract::constructor::TransferNFT,
};
use mandala_node_runner;

#[tokio::test]
async fn should_transfer_nft() {
    let (client, _node) = common::setup_node_and_client().await;
    let contract = common::setup_dummy_721_contract(&client).await;
    let payload = TransferNFT::construct(address, to, token_id, function_selector);
}
