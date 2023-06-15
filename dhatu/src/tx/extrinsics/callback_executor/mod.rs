use std::str::FromStr;

use serde::Serialize;

use super::prelude::enums::{ExtrinsicStatus, Hash};
use crate::error::{CallbackExecutorError, Error};

/// http callback executor for extrinsics transaction.
#[cfg(feature = "tokio")]
pub struct Executor {
    /// in-mmeory http connection pool.
    http_connection_pool: reqwest::Client,
}

/// http callback url.
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

    /// execute http callback given the callback url and the extrinsics status.
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

    /// infer callback body given the extrinsics status.
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

/// general callback body.
/// will consider to customize callbackbody in the future.
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
