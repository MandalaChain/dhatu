use std::{collections::HashMap, sync::Arc};

use serde_json::Value;
use tokio::sync::RwLock;

use crate::extrinsics::prelude::{TransactionId};

pub struct CallbackExecutor {
    result: Arc<RwLock<HashMap<TransactionId, Result<Value, reqwest::Error>>>>,
    http_connection_pool: reqwest::Client,
}

impl Clone for CallbackExecutor {
    fn clone(&self) -> Self {
        Self {
            result: self.result.clone(),
            http_connection_pool: self.http_connection_pool.clone(),
        }
    }
}

impl CallbackExecutor {
    pub fn new() -> Self {
        Self {
            http_connection_pool: reqwest::Client::new(),
            result: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn execute(&self, tx_id: TransactionId, body: Value, callback: String) {
        let client = self.http_connection_pool.clone();
        let callback_result_clone = self.result.clone();

        let task = async move {
            let response = client.post(callback).json(&body).send().await;
            let response = async move {
                match response {
                    Ok(res) => Ok(res.json::<Value>().await.unwrap_or_default()),
                    Err(e) => Err(e),
                }
            }
            .await;

            let mut callback_result = callback_result_clone.write().await;

            callback_result.insert(tx_id, response);
        };

        tokio::task::spawn(task);
    }
}
