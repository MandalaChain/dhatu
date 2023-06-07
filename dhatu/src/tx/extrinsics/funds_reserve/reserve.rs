use sp_core::Pair;

use crate::tx::extrinsics::prelude::BlockchainClient;

use super::traits::{FundsReserveAtributes, FundsReserveTask, FundsReserveTraits};

#[derive(Clone)]
pub struct FundsReserve {
    pair: sp_core::sr25519::Pair,
    client: BlockchainClient,
}

impl FundsReserve {
    pub fn new(pair: sp_core::sr25519::Pair, client: BlockchainClient) -> Self {
        Self { pair, client }
    }
}

impl FundsReserveAtributes for FundsReserve {
    fn reserve_signer(&self) -> &sp_core::sr25519::Pair {
        &self.pair
    }

    fn reserve_adress(&self) -> String {
        self.pair.public().to_string()
    }

    fn client(&self) -> &BlockchainClient {
        &self.client
    }

    fn change_signer(&mut self, pair: sp_core::sr25519::Pair) {
        self.pair = pair;
    }
}

impl FundsReserveTask for FundsReserve {}

impl FundsReserveTraits for FundsReserve {}
