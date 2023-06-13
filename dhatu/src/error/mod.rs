use crate::{
    registrar::{key_manager::prelude::KeypairGenerationError, signer::TxBuilderError},
    tx::extrinsics::prelude::{
        calldata::ToPayloadError, reserve::FundsReserveError, CallbackExecutorError,
    },
    types::MandalaClientErorr,
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("password generation error : {0}")]
    PasswordGenError(String),

    #[error("keypair generation error : ")]
    KeypairGenError(#[from] KeypairGenerationError),

    #[error("mandala client error :")]
    Client(#[from] MandalaClientErorr),

    #[error("reserve error : ")]
    ReserveError(#[from] FundsReserveError),

    #[error("callback executor error : ")]
    CallbackError(#[from] CallbackExecutorError),

    #[error("error when submitting transaction : {0}")]
    TransactionSubmitError(#[from] subxt::Error),

    #[error("error when converting to payload : {0}")]
    PayloadError(#[from] ToPayloadError),

    #[error("error when signing transaction : {0}")]
    SignTransactionError(#[from] TxBuilderError),
}
