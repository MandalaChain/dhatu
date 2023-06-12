use crate::{
    error::Error,
    types::{MandalaExtrinsics, MandalaTransactionProgress},
};

pub struct ExtrinsicSubmitter;

impl ExtrinsicSubmitter {
    pub async fn submit(tx: MandalaExtrinsics) -> Result<MandalaTransactionProgress, Error> {
        let result = tx.into_inner()
            .submit_and_watch()
            .await
            .map_err(Error::TransactionSubmitError)?
            .into();

        Ok(result)
    }
}
