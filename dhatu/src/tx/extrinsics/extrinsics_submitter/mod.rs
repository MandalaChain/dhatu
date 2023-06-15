use crate::{
    error::Error,
    types::{MandalaExtrinsics, MandalaTransactionProgress},
};

/// extrinsic transaction submitter.
pub struct ExtrinsicSubmitter;

impl ExtrinsicSubmitter {
    /// submit extrinsic transaction.
    /// will return a subscriber to the transaction progress.
    pub async fn submit(tx: MandalaExtrinsics) -> Result<MandalaTransactionProgress, Error> {
        let result = tx.into_inner()
            .submit_and_watch()
            .await
            .map_err(Error::Transaction)?
            .into();

        Ok(result)
    }
}
