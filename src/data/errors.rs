use thiserror::Error;

#[derive(Debug, Error)]
pub enum DataError{
    #[error("Failed database query:  {0}")]
    Query(#[from] sqlx::Error),
    #[error("Failed to query database: {0}")]
    FailedQuery(String),
    #[error("Internal error: {0}")]
    Internal(String),
    #[error("Failed to hash password: {0}")]
    Bcrypt(#[from] bcrypt::BcryptError),
    #[error("Failed to convert from utf8: {0}")]
    Utf8Conversion(#[from] std::string::FromUtf8Error)
}