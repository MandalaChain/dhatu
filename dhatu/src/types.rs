use std::ops::Mul;

use rust_decimal::{
    prelude::{FromPrimitive, ToPrimitive},
    Decimal,
};
use subxt::{
    tx::{SubmittableExtrinsic, TxProgress},
    OnlineClient, PolkadotConfig, SubstrateConfig,
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

/// internal channels for communicating between dhatu low levels module.
///
/// # Example
///
/// ```
/// let client = common::setup_node_and_client().await;
/// let facade = DhatuAssetsFacade::new(client);
///
/// let mut channels = InternalChannels::<TransactionMessage>::new();
///
/// let mut recv = channels.get_receiver();
/// let notifier = channels.sender().clone();
///
/// facade.migrate(assets, alice.clone().into(), bob.into(), &reserve, notifier);
/// ```
///
#[cfg(feature = "tokio")]
pub struct InternalChannels<Message> {
    // we're using unbounded channels for practical reasons
    // will consider using buffered channels in the future.
    receiver: Option<ReceiverChannel<Message>>,
    sender: SenderChannel<Message>,
}

impl<Message> From<SenderChannel<Message>> for InternalChannels<Message> {
    /// convert a correct [sender](UnboundedSender) type.
    ///
    /// note that using this will only wraps the sender, and the receiver will be [`None`].
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
    /// creates a new instance of internal channels.
    pub fn new() -> Self {
        Default::default()
    }

    /// must be called only once, will panic if called twice.
    pub fn get_receiver(&mut self) -> ReceiverChannel<Message> {
        self.receiver.take().expect("should be called only once")
    }

    /// check if the receiver channel is taken.
    #[must_use]
    pub fn is_receiver_taken(&self) -> bool {
        self.receiver.is_none()
    }

    /// get this inetnal channels sender
    pub fn sender(&self) -> &SenderChannel<Message> {
        &self.sender
    }
}

/// a wrapped native substrate extrinsics.
/// you would not typically have to interact with this.
///
/// # Example
///
/// ```
/// async fn submit_extrinsic(extrinsic) {
///     let tx = MandalaExtrinsics::new(extrinsic);
///     let tx_progress = ExtrinsicSubmitter::submit(tx).await.unwrap();
/// }
/// ```
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

/// wrapped native substrate node client.
#[derive(Clone)]
pub struct MandalaClient(pub(crate) OnlineClient<MandalaConfig>);

impl From<OnlineClient<MandalaConfig>> for MandalaClient {
    fn from(value: OnlineClient<MandalaConfig>) -> Self {
        Self(value)
    }
}

impl MandalaClient {
    pub(crate) fn inner_internal(&self) -> &OnlineClient<MandalaConfig> {
        &self.0
    }

    #[cfg(feature = "subxt")]
    pub fn inner(&self) -> &OnlineClient<MandalaConfig> {
        &self.0
    }

    /// create a new node client given a node url.
    pub async fn new(node_url: &str) -> Result<Self, crate::error::Error> {
        let client = OnlineClient::<MandalaConfig>::from_url(node_url)
            .await
            .map_err(MandalaClientErorr::from)?;

        Ok(Self(client))
    }

    /// create a new ws client that connects to local node. 
    pub async fn dev() -> Result<Self, crate::error::Error> {
        let client = OnlineClient::<MandalaConfig>::new()
            .await
            .map_err(MandalaClientErorr::from)?;

        Ok(Self(client))
    }
}

/// a blockchain currency unit. abstracts away the pains of dealing with decimals.
/// this struct automatically parses the decimals and amount and converts them to a valid `u128` under the hood.
///
/// use this to mainly deal with currency. e.g transfering, balances, etc.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Unit {
    amount: Decimal,
    decimals: u8,
}

pub const GENERIC_SUBSTRATE_DECIMALS: u8 = 12;

impl Unit {
    /// create a new blockchain currency unit. the decimals will default to
    /// [generic substrate decimals](GENERIC_SUBSTRATE_DECIMALS) if not specified.
    ///
    /// # Example
    ///
    /// ```
    /// let unit = Unit::new("9", None).expect("static values are valid!");
    /// assert_eq!(unit.as_u128(), 9_000_000_000_000);
    ///
    /// let unit = Unit::new("2.1", None).expect("static values are valid!");
    /// assert_eq!(unit.as_u128(), 2_100_000_000_000);
    ///
    /// ```
    pub fn new(amount: &str, decimals: Option<u8>) -> Result<Self, crate::error::Error> {
        let decimals = decimals.unwrap_or(GENERIC_SUBSTRATE_DECIMALS);

        let _decimals = Self::calculate_decimals(decimals);
        let amount = Self::calculate_amount(amount, _decimals)?;

        Ok(Self { amount, decimals })
    }

    /// converts the unit to a valid [u128] value.
    pub fn as_u128(&self) -> u128 {
        self.amount
            .to_u128()
            .expect("valid conversion should not fail")
    }

    /// get the decimals representation of this unit
    pub fn decimals(&self) -> u8 {
        self.decimals
    }

    fn calculate_decimals(decimals: u8) -> Decimal {
        use rust_decimal::prelude::*;
        Self::decimals_multiplier().powu(decimals as u64)
    }

    fn calculate_amount(amount: &str, decimals: Decimal) -> Result<Decimal, crate::error::Error> {
        Ok(Decimal::from_str_exact(amount)?.mul(decimals))
    }

    fn decimals_multiplier() -> Decimal {
        let multiplier = 10;
        Decimal::from_u8(multiplier).expect("static values must be a valid conversion")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit() {
        let unit = Unit::new("9", None).unwrap();
        assert_eq!(unit.as_u128(), 9_000_000_000_000);
    }

    #[test]
    fn test_with_decimals() {
        let unit = Unit::new("0.9", None).unwrap();
        assert_eq!(unit.as_u128(), 900_000_000_000);

        let unit = Unit::new("2.1", None).unwrap();
        assert_eq!(unit.as_u128(), 2_100_000_000_000);

        let unit = Unit::new("2.01", None).unwrap();
        assert_eq!(unit.as_u128(), 2_010_000_000_000);
    }
}
