use crate::{
    error::Error,
    tx::extrinsics::{
        prelude::TransactionId,
        types::{Extrinsic, ExtrinsicTracker, GenericError},
    },
    types::{MandalaExtrinsics, MandalaTransactionProgress},
};

pub struct ExtrinsicSubmitter;

impl ExtrinsicSubmitter {
    pub async fn submit(tx: MandalaExtrinsics) -> Result<MandalaTransactionProgress, Error> {
        let result = tx.into_inner()
            .submit_and_watch()
            .await
            .map_err(|e| Error::TransactionSubmitError(e))?
            .into();

        Ok(result)
    }
}
