use crate::{
    tx::extrinsics::{
        prelude::TransactionId,
        types::{Extrinsic, ExtrinsicTracker, GenericError},
    },
    MandalaExtrinsics,
};

pub struct ExtrinsicSubmitter;

impl ExtrinsicSubmitter {
    pub async fn submit(
        tx: MandalaExtrinsics,
    ) -> Result<(ExtrinsicTracker, TransactionId), GenericError> {
        let tracker = tx.into_inner().submit_and_watch().await?;
        let tx_id = tracker.extrinsic_hash();
        Ok((tracker, tx_id))
    }
}
