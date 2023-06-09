use crate::{
    registrar::key_manager::prelude::KeypairGenerationError,
    tx::extrinsics::prelude::{reserve::FundsReserveError, CallbackExecutorError}, MandalaClientErorr,
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
    CallbackError(#[from] CallbackExecutorError)
}
