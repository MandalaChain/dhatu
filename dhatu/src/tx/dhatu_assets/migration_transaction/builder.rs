use crate::{tx::extrinsics::prelude::reserve::FundsReserve, types::NodeClient};

use super::{traits::MigrationTransactionBuilder, transaction::MigrationTransaction};

/// builder for [MigrationTransaction]
pub(crate) struct MigrationTransactionBuilderStruct {
    signer: Option<subxt::ext::sp_core::sr25519::Pair>,
    notifier: Option<super::types::MigrationTransactionResultNotifier>,
    reserve: Option<FundsReserve>,
    client: Option<NodeClient>,
}

impl MigrationTransactionBuilder for MigrationTransactionBuilderStruct {
    /// create new builder
    fn new() -> Self {
        Self {
            signer: None,
            notifier: None,
            reserve: None,
            client: None,
        }
    }

    fn set_signer(&mut self, signer: subxt::ext::sp_core::sr25519::Pair) -> &mut Self {
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

    fn set_client(&mut self, client: NodeClient) -> &mut Self {
        self.client = Some(client);

        self
    }

    /// build [MigrationTransaction] from builder
    fn build(&mut self) -> MigrationTransaction {
        let signer = self.signer.take().expect("signer should be set");
        let notifier = self.notifier.take().expect("notifier should be set");
        let reserve = self.reserve.take().expect("funds reserve should be set");
        let client = self.client.take().expect("blockchain client should be set");

        MigrationTransaction::new(signer, notifier, reserve, client, None, None)
    }
}
