use subxt::{
    tx::{SubmittableExtrinsic, TxProgress},
    OnlineClient, SubstrateConfig, PolkadotConfig,
};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::error::MandalaClientErorr;

pub(crate) type MandalaConfig = PolkadotConfig;
pub(crate) type NodeClient = OnlineClient<MandalaConfig>;
pub(crate) type Extrinsic = SubmittableExtrinsic<MandalaConfig, NodeClient>;
pub(crate) type TransactionProgress = TxProgress<MandalaConfig, NodeClient>;

#[cfg(feature = "tokio")]
pub type ReceiverChannel<Message> = UnboundedReceiver<Message>;

#[cfg(feature = "tokio")]
pub type SenderChannel<Message> = UnboundedSender<Message>;

#[cfg(feature = "tokio")]
pub struct InternalChannels<Message> {
    // we're using unbounded channels for practical reasons
    // will consider using buffered channels in the future.
    receiver: Option<ReceiverChannel<Message>>,
    sender: SenderChannel<Message>,
}

impl<Message> From<SenderChannel<Message>> for InternalChannels<Message> {
    fn from(value: SenderChannel<Message>) -> Self {
        Self {
            receiver: None,
            sender: value,
        }
    }
}

impl<Message> Default for InternalChannels<Message> {
    fn default() -> Self {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel::<Message>();
        Self {
            receiver: Some(receiver),
            sender,
        }
    }
}

impl<Message> InternalChannels<Message> {
    pub fn new() -> Self {
        Default::default()
    }

    /// must be called only once, will panic if called twice
    pub fn get_receiver(&mut self) -> ReceiverChannel<Message> {
        self.receiver.take().expect("should be called only once")
    }

    #[must_use]
    pub fn is_receiver_taken(&self) -> bool {
        self.receiver.is_none()
    }

    pub fn sender(&self) -> &SenderChannel<Message> {
        &self.sender
    }
}

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


impl MandalaClient {
    pub(crate) fn inner_internal(&self) -> &OnlineClient<MandalaConfig> {
        &self.0
    }

    #[cfg(feature = "subxt")]
    pub fn inner(&self) -> &OnlineClient<MandalaConfig> {
        &self.0
    }

    pub async fn new(node_url: &str) -> Result<Self, crate::error::Error> {
        let client = OnlineClient::<MandalaConfig>::from_url(node_url)
            .await
            .map_err(MandalaClientErorr::from)?;

        Ok(Self(client))
    }

    pub async fn dev() -> Result<Self, crate::error::Error> {
        let client = OnlineClient::<MandalaConfig>::new()
            .await
            .map_err(MandalaClientErorr::from)?;

        Ok(Self(client))
    }
}
