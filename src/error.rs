use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("API error (status {status}): {message}")]
    Api {
        status: u16,
        message: String,
        timestamp: Option<String>,
    },

    #[error("deserialization error: {0}")]
    Deserialize(#[from] serde_json::Error),

    #[error("URL parse error: {0}")]
    Url(#[from] url::ParseError),
}

impl Error {
    pub fn is_not_found(&self) -> bool {
        matches!(self, Error::Api { status: 404, .. })
    }

    pub fn is_unauthorized(&self) -> bool {
        matches!(self, Error::Api { status: 401, .. })
    }

    pub fn is_forbidden(&self) -> bool {
        matches!(self, Error::Api { status: 403, .. })
    }

    pub fn is_rate_limited(&self) -> bool {
        matches!(self, Error::Api { status: 429, .. } | Error::Api { status: 503, .. })
    }
}

pub type Result<T> = std::result::Result<T, Error>;