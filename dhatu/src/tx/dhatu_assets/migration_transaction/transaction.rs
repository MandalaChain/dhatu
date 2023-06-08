use sp_core::{sr25519::Pair, Pair as PairTraits};

use crate::{
    registrar::signer::TxBuilder,
    tx::extrinsics::{
        funds_reserve::traits::FundsReserveTraits,
        prelude::{
            extrinsics,
            submitter::ExtrinsicSubmitter,
            transfer_nft_contract::{
                constructor::TransferNFT, traits::NftTransferTransactionConstructor,
            },
            BlockchainClient,
        },
    },
};

use super::{
    traits::{MigrationTransaction as Transaction, MigrationTransactionAttributes},
    types::{
        MigrationTransaction as SubmittableTransaction, MigrationTransactionPayload,
        MigrationTransactionResultNotifier,
    },
};

const STATIC_NFT_TRANSFER_FEE: u128 = 9_000_000_000; // 9  mili units (9mU)

pub(crate) struct MigrationTransaction<Reserve: FundsReserveTraits> {
    signer: Pair,
    notifier: MigrationTransactionResultNotifier,
    reserve: Reserve,
    client: BlockchainClient,
    payload: Option<MigrationTransactionPayload>,
    inner_tx: Option<SubmittableTransaction>,
}

impl<Reserve: FundsReserveTraits> MigrationTransaction<Reserve> {
    pub fn new(
        signer: Pair,
        notifier: MigrationTransactionResultNotifier,
        reserve: Reserve,
        client: BlockchainClient,
        payload: Option<MigrationTransactionPayload>,
        inner_tx: Option<SubmittableTransaction>,
    ) -> Self {
        Self {
            signer,
            notifier,
            reserve,
            client,
            payload,
            inner_tx,
        }
    }

    pub fn construct_payload(
        mut self,
        address: &str,
        to: &str,
        token_id: i64,
        function_selector: &str,
    ) -> Self {
        let tx =
            TransferNFT::construct(address, to, token_id, function_selector.to_string()).unwrap();

        self.payload = Some(tx);

        self
    }

    pub async fn sign(mut self) -> Self {
        let client = self.client.clone();
        let acc = self.signer.clone();
        let payload = self
            .payload
            .take()
            .expect("migration payload not constructed");

        let tx = TxBuilder::signed(&client, acc, &payload)
            .await
            .expect("should sign transaction");

        self.inner_tx = Some(tx);

        self
    }

    pub async fn ensure_enough_gas(self) -> Self {
        let account = self.signer.public().to_string();

        // future implementation will dynamically check the threshold and then transfer.
        // currently this automatically transfer funds regardless of quota threshold
        self.reserve
            .transfer_funds(account, STATIC_NFT_TRANSFER_FEE)
            .await
            .unwrap();

        self
    }

    pub async fn submit(mut self) -> crate::tx::extrinsics::prelude::extrinsics::Transaction {
        let tx = self
            .inner_tx
            .take()
            .expect("inner transaction should have been built");

        let (progress, _) = ExtrinsicSubmitter::submit(tx).await.unwrap();
        let notifier_channel = self.notifier.clone();

        extrinsics::Transaction::new(progress, notifier_channel, None)
    }
}

impl<Reserve: FundsReserveTraits> MigrationTransactionAttributes<Reserve>
    for MigrationTransaction<Reserve>
{
    fn signer(&self) -> &Pair {
        &self.signer
    }

    fn notifier(&self) -> &MigrationTransactionResultNotifier {
        &self.notifier
    }

    fn reserve(&self) -> &Reserve {
        &self.reserve
    }

    fn client(&self) -> &BlockchainClient {
        &self.client
    }

    fn payload(&self) -> Option<&MigrationTransactionPayload> {
        self.payload.as_ref()
    }

    fn inner_tx(&self) -> Option<&SubmittableTransaction> {
        self.inner_tx.as_ref()
    }
}

impl<Reserve: FundsReserveTraits> Transaction<Reserve> for MigrationTransaction<Reserve> {}
