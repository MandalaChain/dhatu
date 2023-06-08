#[derive(thiserror::Error, Debug)]
pub enum DhatuError {
    #[error("password length must be at least 32 characters long!")]
    PasswordGenError,
    #[error("{0}")]
    KeypairGenError(String),
}
