use sp_core::Pair;

use crate::{
    registrar::key_manager::prelude::SecretKey, tx::extrinsics::prelude::BlockchainClient,
    MandalaClient,
};

use super::traits::{FundsReserveAtributes, FundsReserveTask, FundsReserveTraits};

#[derive(Clone)]
pub struct FundsReserve {
    reserve: sp_core::sr25519::Pair,
    client: BlockchainClient,
}

impl FundsReserve {
    pub fn new(reserve_key: SecretKey, client: MandalaClient) -> Self {
        Self {
            reserve: reserve_key.into(),
            client: client.inner().to_owned(),
        }
    }
}

impl FundsReserveAtributes for FundsReserve {
    fn reserve_signer(&self) -> &sp_core::sr25519::Pair {
        &self.reserve
    }

    fn reserve_adress(&self) -> String {
        self.reserve.public().to_string()
    }

    fn client(&self) -> &BlockchainClient {
        &self.client
    }

    fn change_signer(&mut self, pair: sp_core::sr25519::Pair) {
        self.reserve = pair;
    }
}

impl FundsReserveTask for FundsReserve {}

impl FundsReserveTraits for FundsReserve {}
