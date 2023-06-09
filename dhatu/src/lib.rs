use subxt::{tx::SubmittableExtrinsic, OnlineClient, PolkadotConfig};
use types::{MandalaConfig, NodeClient, Extrinsic};

pub mod error;
pub(crate) mod private;
pub mod registrar;
pub mod tx;
pub mod types;


pub struct MandalaExtrinsics(pub(crate) SubmittableExtrinsic<MandalaConfig, NodeClient>);

impl MandalaExtrinsics {
    pub(crate) fn new(tx: SubmittableExtrinsic<MandalaConfig, NodeClient>) -> Self {
        Self(tx)
    }

    pub(crate) fn into_inner(self) -> SubmittableExtrinsic<MandalaConfig, NodeClient> {
        self.0
    }
}

impl  From<Extrinsic> for MandalaExtrinsics {
    fn from(value: Extrinsic) -> Self {
        Self(value)
    }
}


#[derive(Clone)]
pub struct MandalaClient(pub(crate) OnlineClient<MandalaConfig>);

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
