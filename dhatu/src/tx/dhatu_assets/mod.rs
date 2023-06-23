//! this module is intended for managing mandala blockchain nft assets.

/// abstractions over migration transactions
pub mod facade;
#[doc(hidden)]
pub(crate) mod migration_transaction;
/// traits necessary for extrinsics abstraction
pub mod traits;
