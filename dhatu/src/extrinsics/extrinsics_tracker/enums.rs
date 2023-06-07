use sp_core::H256;

pub type Reason = String;
pub type Hash = H256;

#[derive(Debug, Clone)]
pub enum ExtrinsicStatus {
    Pending,
    Failed(Reason),
    Success(Hash),
}

impl Default for ExtrinsicStatus {
    fn default() -> Self {
        Self::Pending
    }
}
