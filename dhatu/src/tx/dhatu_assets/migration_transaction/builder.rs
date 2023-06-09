use crate::tx::extrinsics::prelude::{reserve::FundsReserve, BlockchainClient};

use super::{traits::MigrationTransactionBuilder, transaction::MigrationTransaction};

pub(crate) struct MigrationTransactionBuilderStruct {
    signer: Option<sp_core::sr25519::Pair>,
    notifier: Option<super::types::MigrationTransactionResultNotifier>,
    reserve: Option<FundsReserve>,
    client: Option<BlockchainClient>,
}

impl MigrationTransactionBuilder for MigrationTransactionBuilderStruct {
    fn new() -> Self {
        Self {
            signer: None,
            notifier: None,
            reserve: None,
            client: None,
        }
    }

    fn set_signer(&mut self, signer: sp_core::sr25519::Pair) -> &mut Self {
        self.signer = Some(signer);

        self
    }

    fn set_notifier(
        &mut self,
        notifier: super::types::MigrationTransactionResultNotifier,
    ) -> &mut Self {
        self.notifier = Some(notifier);

        self
    }

    fn set_gas_reserve(&mut self, reserve: FundsReserve) -> &mut Self {
        self.reserve = Some(reserve);

        self
    }

    fn set_client(
        &mut self,
        client: crate::tx::extrinsics::prelude::BlockchainClient,
    ) -> &mut Self {
        self.client = Some(client);

        self
    }

    fn build(&mut self) -> MigrationTransaction {
        let signer = self.signer.take().expect("signer should be set");
        let notifier = self.notifier.take().expect("notifier should be set");
        let reserve = self.reserve.take().expect("funds reserve should be set");
        let client = self.client.take().expect("blockchain client should be set");

        MigrationTransaction::new(signer, notifier, reserve, client, None, None)
    }
}
