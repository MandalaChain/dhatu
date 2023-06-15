use subxt::{tx::SubmittableExtrinsic, SubstrateConfig};

use crate::{tx::extrinsics::{prelude::{
     extrinsics::TransactionMessage,
}, transaction_constructor::transfer_nft_contract::constructor::NftTransferPayload}, types::{SenderChannel, ReceiverChannel, MandalaConfig}};


pub type MigrationTask<T> = std::pin::Pin<Box<dyn futures::Future<Output = T>>>;
pub type MigrationTransactionPayload = NftTransferPayload;
pub type MigrationTransactionResultNotifier =
    SenderChannel<TransactionMessage>;
pub type MigrationTransactionResultReceiver =
    ReceiverChannel<TransactionMessage>;
pub type MigrationTransaction = SubmittableExtrinsic<MandalaConfig, crate::types::NodeClient>;
