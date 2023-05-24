use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ApiError(#[from] ErrorKind),

    #[error(transparent)]
    ClientError(#[from] json_api::error::Error),
}

#[derive(Error, Debug)]
pub enum ErrorKind {
    #[error("Feature not found with name: {0}")]
    FeatureNotFound(String),
}

pub type Result<T> = std::result::Result<T, Error>;
