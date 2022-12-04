use std::fmt::{Display, Formatter};

pub type BoxedError = Box<dyn std::error::Error>;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    MissingSID,
    InvalidSID,
    InputUnavailable,
    TestInputNotFound,
    Misc(String)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingSID => write!(f, "missing SID_COOKIE environment variable"),
            Self::InvalidSID => write!(f, "invalid session cookie"),
            Self::InputUnavailable => write!(f, "input unavailable"),
            Self::TestInputNotFound => write!(f, "test input not found"),
            Self::Misc(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for Error {}
