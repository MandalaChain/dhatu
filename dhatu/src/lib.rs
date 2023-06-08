use subxt::{OnlineClient, PolkadotConfig};

pub mod error;
pub(crate) mod private;
pub mod registrar;
pub mod tx;

#[derive(Clone)]
pub struct MandalaClient(OnlineClient<PolkadotConfig>);

#[derive(thiserror::Error, Debug)]
pub enum MandalaClientErorr {
    #[error("connection Error : {0}")]
    Connection(#[from] subxt::Error),
}

impl MandalaClient {
    pub(crate) fn inner(&self) -> &OnlineClient<PolkadotConfig> {
        &self.0
    }

    pub async fn new(node_url: String) -> Result<Self, crate::error::Error> {
        let client = OnlineClient::<PolkadotConfig>::from_url(node_url)
            .await
            .map_err(MandalaClientErorr::from)?;

        Ok(Self(client))
    }

    pub async fn dev() -> Result<Self, crate::error::Error> {
        let client = OnlineClient::<PolkadotConfig>::new()
            .await
            .map_err(MandalaClientErorr::from)?;

        Ok(Self(client))
    }
}
