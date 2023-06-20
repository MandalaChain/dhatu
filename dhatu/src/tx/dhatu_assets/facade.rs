use crate::{
    registrar::key_manager::prelude::{PrivateKey, PublicAddress},
    types::MandalaClient,
};
use futures::{future, FutureExt};

use crate::tx::extrinsics::prelude::reserve::FundsReserve;

use super::{
    migration_transaction::{
        builder::MigrationTransactionBuilderStruct,
        traits::MigrationTransactionBuilder,
        types::{MigrationTransactionResultNotifier, MigrationTransactionResultReceiver},
    },
    traits::Asset,
};

/// facade for managing mandala blockchain nft assets
pub struct DhatuAssetsFacade {
    client: MandalaClient,
}

impl DhatuAssetsFacade {
    /// create a new notifier and receiver for migration transaction result.
    /// 
    /// note that the notifier and receiver is unbounded.
    #[cfg(feature = "tokio")]
    pub fn create_channels() -> (
        MigrationTransactionResultNotifier,
        MigrationTransactionResultReceiver,
    ) {
        tokio::sync::mpsc::unbounded_channel()
    }
}

impl DhatuAssetsFacade {
    pub fn new(mandala_client: MandalaClient) -> Self {
        Self {
            client: mandala_client,
        }
    }

    /// migrate known nft assets from one account to another.
    ///
    /// `assets` : list of assets to be migrated.
    ///
    /// `from` : private key of the account that owns the assets.
    ///
    /// `to` : public address of the account that will receive the assets.
    ///
    /// `reserve` : funds reserve for the transaction (used to supply gas fee).
    ///
    /// `notifier` : notifier for the transaction result.
    ///
    /// note that the it will send the transaction result on every transaction instead of waiting
    /// all of the transaction to complete.
    /// 
    /// you can create the notifier and receiver using `DhatuAssetsFacade::create_channels()`.
    pub fn migrate(
        &self,
        assets: Vec<impl Asset>,
        from: PrivateKey,
        to: PublicAddress,
        reserve: &FundsReserve,
        notifier: MigrationTransactionResultNotifier,
    ) {
        // TODO : optimize the migration with queue
        let mut tx_batch = Vec::new();
        let client = self.client.inner();

        for asset in assets {
            let tx = MigrationTransactionBuilderStruct::new()
                .set_signer(from.0.clone())
                .set_notifier(notifier.clone())
                .set_gas_reserve(reserve.clone())
                .set_client(client.clone())
                .build();

            let tx = tx
                .construct_payload(
                    asset.contract_address(),
                    &to.0,
                    asset.token_id(),
                    asset.function_selector(),
                )
                .sign()
                .then(|tx| async move { tx.ensure_enough_gas().await })
                .then(|tx| async move { tx.submit().await });

            tx_batch.push(tx)
        }

        // TODO : refactor this to executes the futures in pararell
        let transactions = future::join_all(tx_batch);
        tokio::task::spawn(transactions);
    }
}
