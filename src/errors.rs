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

// use std::fmt::Display;
//
// #[derive(Debug, Clone)]
// pub enum CrawlerError {
//     /// Url parsing error
//     UrlError(url::ParseError),
// }
//
// impl Display for CrawlerError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             CrawlerError::UrlError(e) => {
//                 write!(f, "{}: <domain.org>", e.to_string())
//             }
//         }
//     }
// }
//
// impl std::error::Error for CrawlerError {}
//
// impl From<url::ParseError> for CrawlerError {
//     fn from(err: url::ParseError) -> Self {
//         CrawlerError::UrlError(err)
//     }
// }

// use std::fmt::Display;
// use url::Url;

// #[derive(Debug)]
// pub enum CrawlerError {
//     UrlError(::url::ParseError),
//     ReqError(::reqwest::Error),
// }
//
// impl Display for CrawlerError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

// impl Display for CrawlerError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             CrawlerError::UrlError(e) => {
//                 write!(f, "Not a valid Url {}", e)
//             },
//             CrawlerError::ReqError(e) => {
//                 write!(f, "Http request error {}", e)
//             },
//         }
//     }
// }

//
// impl std::error::Error for CrawlerError {}
//
// impl From<url::ParseError> for CrawlerError {
//     fn from(err: url::ParseError) -> Self {
//         CrawlerError::UrlError(err)
//     }
// }
//
// impl From<reqwest::Error> for CrawlerError {
//     fn from(err: reqwest::Error) -> Self {
//         CrawlerError::ReqError(err.to_string())
//     }
// }
