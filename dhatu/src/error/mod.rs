use crate::{MandalaClientErorr, registrar::key_manager::prelude::KeypairGenerationError};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("password generation error : {0}")]
    PasswordGenError(String),
    #[error("keypair generation error : ")]
    KeypairGenError(#[from] KeypairGenerationError),
    #[error("mandala client error :")]
    MandalaClient(#[from] MandalaClientErorr),
}
