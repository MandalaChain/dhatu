//! for now we only use the [attribute traits](MigrationTransactionAttributes)
//! due to other traits requiring async operation and it would be a maintain hell to maintain
//! async traits right now.

use sp_core::sr25519::Pair;

use crate::extrinsics::{
    funds_reserve::traits::FundsReserveTraits,
    prelude::{extrinsics::Transaction as RunningTransaction, BlockchainClient},
};

use super::types::{
    MigrationTask as Task, MigrationTransaction as Transaction, MigrationTransactionPayload,
    MigrationTransactionResultNotifier,
};

pub(crate) trait MigrationTask {
    fn construct_payload(
        &'static mut self,
        address: &str,
        to: &str,
        token_id: i64,
        function_selector: &str,
    ) -> &mut Self;

    fn sign(&'static mut self) -> Task<&mut Self>;

    fn ensure_enough_gas(&'static mut self) -> Task<&mut Self>;

    fn submit(&'static mut self) -> Task<RunningTransaction>;
}
pub(crate) trait MigrationProcess {
    fn start(&mut self);
}

pub(crate) trait MigrationTransactionBuilder<
    Reserve: FundsReserveTraits,
    Transaction: MigrationTransaction<Reserve>,
>
{
    fn new() -> Self;

    fn set_signer(&mut self, signer: Pair) -> &mut Self;

    fn set_notifier(&mut self, notifier: MigrationTransactionResultNotifier) -> &mut Self;

    fn set_gas_reserve(&mut self, reserve: Reserve) -> &mut Self;

    fn set_client(&mut self, client: BlockchainClient) -> &mut Self;

    fn build(&mut self) -> Transaction;
}

pub(crate) trait MigrationTransactionAttributes<Reserve: FundsReserveTraits> {
    fn signer(&self) -> &Pair;

    fn notifier(&self) -> &MigrationTransactionResultNotifier;

    fn reserve(&self) -> &Reserve;

    fn client(&self) -> &BlockchainClient;

    fn payload(&self) -> Option<&MigrationTransactionPayload>;

    fn inner_tx(&self) -> Option<&Transaction>;
}

pub(crate) trait MigrationTransaction<Reserve: FundsReserveTraits>:
    MigrationTransactionAttributes<Reserve>
{
}
