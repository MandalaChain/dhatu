use subxt::{tx::SubmittableExtrinsic, OnlineClient, PolkadotConfig};

pub mod error;
pub(crate) mod private;
pub mod registrar;
pub mod tx;

pub type MandalaConfig = PolkadotConfig;
pub(crate) type NodeClient = OnlineClient<MandalaConfig>;
 
pub struct MandalaExtrinsics(pub(crate) SubmittableExtrinsic<NodeClient,MandalaConfig>);

impl MandalaExtrinsics {
    pub(crate) fn new(tx: SubmittableExtrinsic<NodeClient, MandalaConfig>) -> Self {
        Self(tx)
    }

    pub(crate) fn into_inner(self) -> SubmittableExtrinsic<NodeClient,MandalaConfig>{
        self.0
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
