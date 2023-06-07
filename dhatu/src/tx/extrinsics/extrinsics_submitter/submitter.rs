use crate::tx::extrinsics::{
    prelude::TransactionId,
    types::{Extrinsic, ExtrinsicTracker, GenericError},
};

pub struct ExtrinsicSubmitter;

impl ExtrinsicSubmitter {
    pub async fn submit(tx: Extrinsic) -> Result<(ExtrinsicTracker, TransactionId), GenericError> {
        let tracker = tx.submit_and_watch().await?;
        let tx_id = tracker.extrinsic_hash();
        Ok((tracker, tx_id))
    }
}
