use subxt::{tx::SubmittableExtrinsic, OnlineClient, PolkadotConfig};

pub(crate) type MandalaConfig = PolkadotConfig;
pub(crate) type NodeClient = OnlineClient<MandalaConfig>;
pub(crate) type Extrinsic = SubmittableExtrinsic<MandalaConfig, NodeClient>;
