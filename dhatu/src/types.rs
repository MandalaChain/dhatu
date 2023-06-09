use subxt::{
    tx::{SubmittableExtrinsic, TxProgress},
    OnlineClient, PolkadotConfig,
};

pub(crate) type MandalaConfig = PolkadotConfig;
pub(crate) type NodeClient = OnlineClient<MandalaConfig>;
pub(crate) type Extrinsic = SubmittableExtrinsic<MandalaConfig, NodeClient>;
pub(crate) type TransactionProgress = TxProgress<MandalaConfig, NodeClient>;

pub struct MandalaExtrinsics(pub(crate) Extrinsic);

impl MandalaExtrinsics {
    pub(crate) fn new(tx: Extrinsic) -> Self {
        Self(tx)
    }

    pub(crate) fn into_inner(self) -> Extrinsic {
        self.0
    }
}

impl From<Extrinsic> for MandalaExtrinsics {
    fn from(value: Extrinsic) -> Self {
        Self(value)
    }
}

pub struct MandalaTransactionProgress(pub(crate) TransactionProgress);

impl From<TransactionProgress> for MandalaTransactionProgress {
    fn from(value: TransactionProgress) -> Self {
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
