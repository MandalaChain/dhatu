/// callback executor module. used to execute http callback after extrinsics transaction is completed.
pub mod callback_executor;
/// extrinsics submitter module. used to submit extrinsics to the blockchain.
pub mod extrinsics_submitter;
/// extrinsics tracker module. used to track extrinsics status.
pub mod extrinsics_tracker;
/// funds reserve module. used for reserve funds for extrinsics transaction ( used to supply gas fees ).
pub mod funds_reserve;
/// extrinsics manager module. used to manage extrinsics transaction.
/// this also act as a facade or abstraction for all extrinsics module.
/// user is expected to mainly use this module.
pub mod manager;
/// transaction constructor module. used to construct extrinsics transaction.
/// there are various transaction constructor for various extrinsics.
pub mod transaction_constructor;

/// re export all extrinsics modules for convinience sake.
pub mod prelude {
    use super::*;

    pub use callback_executor::*;
    pub use extrinsics_submitter::*;
    pub use extrinsics_tracker::*;
    pub use funds_reserve::*;
    pub use manager::*;
    pub use transaction_constructor::*;
}
