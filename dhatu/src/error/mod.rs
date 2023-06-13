use crate::registrar::signer::TxBuilderError;

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

#[derive(thiserror::Error, Debug)]
pub enum KeypairGenerationError {
    #[error("{0}")]
    PublicAddress(String),

    #[error("fail to generate mnemonic phrase with {0}")]
    MnemonicPhrase(String),

    #[error("{0}")]
    PrivateKey(String),

    #[error("{0}")]
    Recover(String),
}

#[derive(thiserror::Error, Debug)]
pub enum MandalaClientErorr {
    #[error("connection Error : {0}")]
    Connection(#[from] subxt::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum FundsReserveError {
    #[error("{0}")]
    RpcError(#[from] subxt::error::Error),

    #[error("account does not exist!")]
    NonExistentAccount,
}

#[derive(thiserror::Error, Debug)]
pub enum CallbackExecutorError {
    #[error("{0}")]
    InvalidUrl(String),
}

#[derive(thiserror::Error, Debug)]
pub enum ToPayloadError {
    #[error("{0}")]
    AddressError(String),
}
