use std::{str::FromStr};

use serde_json::Value;


use crate::{error::Error};

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

    #[cfg(feature = "tokio", feature = "serde")]
    pub fn execute(&self, body: Value, callback: &str) -> Result<(), Error> {
        let client = self.http_connection_pool.clone();

        let callback = Url::from_str(callback)?;
        let task = client.post(callback.0).json(&body).send();

        tokio::task::spawn(task);

        Ok(())
    }
}
