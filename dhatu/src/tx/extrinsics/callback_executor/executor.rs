use std::{collections::HashMap, str::FromStr, sync::Arc};

use serde_json::Value;
use tokio::sync::RwLock;

use crate::{error::Error, tx::extrinsics::prelude::TransactionId};

pub struct CallbackExecutor {
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

impl Clone for CallbackExecutor {
    fn clone(&self) -> Self {
        Self {
            http_connection_pool: self.http_connection_pool.clone(),
        }
    }
}

impl CallbackExecutor {
    pub fn new() -> Self {
        Self {
            http_connection_pool: reqwest::Client::new(),
        }
    }

    pub fn execute(&self, body: Value, callback: String) {
        let client = self.http_connection_pool.clone();
        let task = client.post(callback).json(&body).send();

        tokio::task::spawn(task);
    }
}
