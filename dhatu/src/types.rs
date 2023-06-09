use subxt::{
    tx::{SubmittableExtrinsic, TxProgress},
    OnlineClient, PolkadotConfig,
};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub(crate) type MandalaConfig = PolkadotConfig;
pub(crate) type NodeClient = OnlineClient<MandalaConfig>;
pub(crate) type Extrinsic = SubmittableExtrinsic<MandalaConfig, NodeClient>;
pub(crate) type TransactionProgress = TxProgress<MandalaConfig, NodeClient>;

pub(crate) type ReceiverChannel<Message> = UnboundedReceiver<Message>;
pub(crate) type SenderChannel<Message> = UnboundedSender<Message>;

pub(crate) struct InternalChannels<Message> {
    // we're using unbounded channels for for practical reasons
    // will consider using buffered channels in the future.
    receiver: Option<ReceiverChannel<Message>>,
    sender: SenderChannel<Message>,
}

impl<Message> Default for InternalChannels<Message> {
    fn default() -> Self {
        let (receiver, sender) = tokio::sync::mpsc::unbounded_channel::<Message>();
        Self {
            receiver: Some(receiver),
            sender,
        }
    }
}

impl<Message> InternalChannels<Message> {
    pub(crate) fn new() -> Self {
        Default::default()
    }

    /// must be called only once, will panic if called twice
    pub(crate) fn get_receiver(&mut self) -> ReceiverChannel<Message> {
        self.receiver
            .take()
            .expect("internal function. should be called only once")
    }

    #[must_use]
    pub(crate) fn is_receiver_taken(&self) -> bool {
        self.receiver.is_none()
    }

    pub(crate) fn sender(&self) -> &SenderChannel<Message> {
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
