use crate::{
    registrar::key_manager::prelude::KeypairGenerationError,
    tx::extrinsics::prelude::reserve::FundsReserveError, MandalaClientErorr,
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("password generation error : {0}")]
    PasswordGenError(String),

    #[error("keypair generation error : ")]
    KeypairGenError(#[from] KeypairGenerationError),

    #[error("mandala client error :")]
    MandalaClient(#[from] MandalaClientErorr),

    #[error("reserve error : ")]
    ReserveError(#[from] FundsReserveError),
}
