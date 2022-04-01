use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum RError {
    #[error("with protocol: <https://github.com>")]
    ParseError,
    #[error("reqwest: {0}")]
    Reqwest(String),
    #[error("tokio join error: {0}")]
    TokioJoinError(String),
    #[error("{0}: invalid HTTP response")]
    InvalidHttpResponse(String),
    #[error("{0}: file error")]
    FileError(String)
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

impl std::convert::From<std::io::Error> for RError {
    fn from(err: std::io::Error) -> Self {
        RError::FileError(err.to_string())
    }
}
