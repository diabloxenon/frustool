#[derive(Debug, Clone)]
pub enum Error {
    InvalidParams(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidParams(message) => write!(f, "{}", message),
        }
    }
}
