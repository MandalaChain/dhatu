#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("password length must be at least 32 characters long!")]
    PasswordGenError,
    #[error("{0}")]
    KeypairGenError(String),
}
