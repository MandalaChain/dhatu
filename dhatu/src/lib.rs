//! dhatu core libraries. aims to abstract away the complexity of interacting to substrate blockchain.
//! for now, is meant to be used with mandala based blockchains. but in future, it will be extended to support other substrate based blockchains.

//! # Re-exports
//! ```
//! #[cfg(feature = "unstable_sp_core")]
//! pub use sp_core;
//! #[cfg(feature = "subxt")]
//! pub use subxt;
//! ```
//! due to the unstable nature of sp_core and subxt, they are not re-exported by default.
//! if you want to interact with the some of the low level feature of dhatu and use the raw substrate primitive types,
//! we reccomend you to enable `unstable_sp_core`,`subxt`  or both with `unstable` feature flag to properly interact with the low level modules.
//!
//! see [ext] for more details.


/// error associated with dhatu
pub mod error;
/// crate private modules
pub(crate) mod private;
/// identity registrar, consist of types and modules regarding blockchain identity.
/// i.e keypair.
pub mod registrar;
/// transaction module, consist of extrinsics abstraction. 
pub mod tx;
/// global crate level types, code inside this modules is meant to be used globally.
pub mod types;
/// re export external libraries that makes up dhatu.
pub mod ext;
