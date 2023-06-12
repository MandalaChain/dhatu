use std::str::FromStr;

use serde::Serialize;
use serde_json::Value;

use super::prelude::enums::{ExtrinsicStatus, Hash};
use crate::error::Error;

#[cfg(feature = "tokio")]
pub struct Executor {
    http_connection_pool: reqwest::Client,
}

#[derive(thiserror::Error, Debug)]
pub enum CallbackExecutorError {
    #[error("{0}")]
    InvalidUrl(String),
}

pub struct Url(pub(crate) reqwest::Url);

impl FromStr for Url {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url = reqwest::Url::from_str(s)
            .map_err(|e| CallbackExecutorError::InvalidUrl(e.to_string()))?;

        Ok(Self(url))
    }
}

impl Clone for Executor {
    fn clone(&self) -> Self {
        Self {
            http_connection_pool: self.http_connection_pool.clone(),
        }
    }
}

impl Executor {
    pub fn new() -> Self {
        Self {
            http_connection_pool: reqwest::Client::new(),
        }
    }

    #[cfg(feature = "tokio")]
    #[cfg(feature = "serde")]
    pub fn execute(&self, status: ExtrinsicStatus, callback_url: &str) -> Result<(), Error> {
        let client = self.http_connection_pool.clone();

        let body = Self::infer_callback_body(status);

        let callback = Url::from_str(callback_url)?;
        let task = client.post(callback.0).json(&body).send();

        tokio::task::spawn(task);

        Ok(())
    }

    fn infer_callback_body(status: ExtrinsicStatus) -> CallBackBody<Hash> {
        match status {
            ExtrinsicStatus::Pending => CallBackBody::new(false, String::from("pending"), None),
            ExtrinsicStatus::Failed(reason) => CallBackBody::new(
                false,
                format!("failed with reason : {}", reason.inner()),
                None,
            ),
            ExtrinsicStatus::Success(result) => {
                CallBackBody::new(true, String::from("success"), Some(result.hash()))
            }
        }
    }
}

#[cfg(feature = "serde")]
#[derive(Serialize)]
pub struct CallBackBody<Data: Serialize> {
    status: bool,
    message: String,
    data: Option<Data>,
}

#[cfg(feature = "serde")]
impl<Data: Serialize> CallBackBody<Data> {
    pub fn new(status: bool, message: String, data: Option<Data>) -> Self {
        Self {
            status,
            message,
            data,
        }
    }
}
