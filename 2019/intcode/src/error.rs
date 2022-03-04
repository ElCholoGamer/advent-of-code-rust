use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    detail: String,
}

impl From<&str> for Error {
    fn from(detail: &str) -> Self {
        Self {
            detail: String::from(detail)
        }
    }
}

impl From<String> for Error {
    fn from(detail: String) -> Self {
        Self { detail }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.detail)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.detail
    }
}
