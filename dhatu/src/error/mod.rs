/// the base error enum. this is the expected error type that will be returned.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// error when generating password for keypair phrase.
    #[error("password generation error : {0}")]
    PasswordGenError(String),

    /// error when generating keypair.
    #[error("keypair generation error : ")]
    KeypairGenError(#[from] KeypairGenerationError),

    /// error associated with the underlying blockchain rpc client.
    #[error("mandala client error :")]
    Client(#[from] MandalaClientErorr),

    /// error associated with reserve funds transactions.
    #[error("reserve error : ")]
    ReserveError(#[from] FundsReserveError),

    /// error when executing transaction http callback.
    #[error("callback executor error : ")]
    CallbackError(#[from] CallbackExecutorError),

    /// error when submitting transaction to rpc client.
    #[error("error when submitting transaction : {0}")]
    TransactionSubmitError(#[from] subxt::Error),

    /// error when encoding calldata to contract pallet payload.
    #[error("error when converting to payload : {0}")]
    PayloadError(#[from] ToPayloadError),

    /// error when signing transaction.
    #[error("error when signing transaction : {0}")]
    SignTransactionError(#[from] TxBuilderError),
}

/// all error that can happen when generating and parsing keypair related stuff.
#[derive(thiserror::Error, Debug)]
pub enum KeypairGenerationError {
    /// error parsing keypair.
    #[error("{0}")]
    PublicAddress(String),

    /// error parsing mnemonic phrase.
    #[error("fail to generate mnemonic phrase with {0}")]
    MnemonicPhrase(String),

    /// error parsing private key.
    #[error("{0}")]
    PrivateKey(String),

    /// error recovering keypair.
    #[error("{0}")]
    Recover(String),
}

/// error that happens on the underlying blockchain rpc client.
#[derive(thiserror::Error, Debug)]
pub enum MandalaClientErorr {

    /// error associating with the node connection
    #[error("connection Error : {0}")]
    Connection(#[from] subxt::Error),
}

/// reserve funds transaction error
#[derive(thiserror::Error, Debug)]
pub enum FundsReserveError {
    /// rpc related error
    #[error("{0}")]
    RpcError(#[from] subxt::error::Error),

    /// account does not exist, happens when trying 
    /// to transfer funds to an account that does not exist.
    #[error("account does not exist!")]
    NonExistentAccount,
}

/// errors related with executing transaction http callback.
#[derive(thiserror::Error, Debug)]
pub enum CallbackExecutorError {
    /// failed to parse url.
    #[error("{0}")]
    InvalidUrl(String),
}

/// errors that can happen when encoding calldata to pallet contrats payload.
#[derive(thiserror::Error, Debug)]
pub enum ToPayloadError {
    /// failed to parse account address.
    #[error("{0}")]
    AddressError(String),
}

/// errors that can happen when signing transaction.
#[derive(thiserror::Error, Debug)]
pub enum TxBuilderError {
    #[error("{0}")]
    SignErorr(#[from] subxt::Error),
}
