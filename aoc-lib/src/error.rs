use std::fmt::{Display, Formatter};

pub type BoxedError = Box<dyn std::error::Error>;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    MissingSID,
    InvalidSID,
    InputUnavailable,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingSID => write!(f, "Could not find SID_COOKIE environment variable"),
            Self::InvalidSID => write!(f, "The provided session cookie is invalid"),
            Self::InputUnavailable => write!(f, "The requested input is unavailable"),
        }
    }
}

impl std::error::Error for Error {}
