use crate::{
    registrar::key_manager::prelude::{PrivateKey, PublicAddress},
    types::MandalaClient,
};
use futures::FutureExt;

use crate::tx::extrinsics::prelude::reserve::FundsReserve;

use super::{
    migration_transaction::{
        builder::MigrationTransactionBuilderStruct, traits::MigrationTransactionBuilder,
        types::MigrationTransactionResultNotifier,
    },
    traits::Asset,
};

/// facade for managing mandala blockchain nft assets
pub struct DhatuAssetsFacade {
    client: MandalaClient,
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
    /// you can create the notifier and receiver using [internal channels](crate::types::InternalChannels).
    pub fn migrate(
        &self,
        assets: Vec<impl Asset>,
        from: PrivateKey,
        to: PublicAddress,
        reserve: &FundsReserve,
        notifier: MigrationTransactionResultNotifier,
    ) {
        // TODO : optimize the migration with batch transaction
        // using nonce tracker for the funds reserve and asse owner.
        let mut tx_batch = Vec::new();
        let client = self.client.inner_internal();

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
                    to.clone(),
                    asset.token_id(),
                    asset.function_selector(),
                )
                .sign()
                .then(|tx| async move { tx.ensure_enough_gas().await })
                .then(|tx| async move { tx.submit().await });

            tx_batch.push(tx)
        }

        // TODO : refactor this to executes the futures in pararell
        let transactions = async move {
            for tx in tx_batch {
                tx.await;
            }
        };
        tokio::task::spawn(transactions);
    }
}
