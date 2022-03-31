use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum RError {
    #[error("Usage: tricoder <kerkour.com>")]
    ParseError,
    #[error("Reqwest: {0}")]
    Reqwest(String),
    #[error("tokio join error: {0}")]
    TokioJoinError(String),
    #[error("{0}: Invalid HTTP response")]
    InvalidHttpResponse(String),
}

impl std::convert::From<reqwest::Error> for RError {
    fn from(err: reqwest::Error) -> Self {
        RError::Reqwest(err.to_string())
    }
}

impl std::convert::From<tokio::task::JoinError> for RError {
    fn from(err: tokio::task::JoinError) -> Self {
        RError::TokioJoinError(err.to_string())
    }
}

impl std::convert::From<url::ParseError> for RError {
    fn from(err: url::ParseError) -> Self {
        RError::ParseError
    }
}
