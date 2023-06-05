use actix_web::error::BlockingError;

pub type ManageAssetTask<T = Result<(), BlockingError>> =
    std::pin::Pin<Box<dyn futures::Future<Output = T> + Send>>;
