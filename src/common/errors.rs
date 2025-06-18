use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ProxyError {
    ConnectionFailed(reqwest::Error),
    AuthenticationFailed,
    Timeout,
    ProtocolError(String),
    InvalidResponse(String),
    UnsupportedProxyType,
}

impl fmt::Display for ProxyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConnectionFailed(err) => write!(f, "Connection failed: {}", err),
            Self::AuthenticationFailed => write!(f, "Authentication failed"),
            Self::Timeout => write!(f, "Connection timed out"),
            Self::ProtocolError(msg) => write!(f, "Protocol error: {}", msg),
            Self::InvalidResponse(msg) => write!(f, "Invalid response: {}", msg),
            Self::UnsupportedProxyType => write!(f, "Unsupported proxy type"),
        }
    }
}

impl Error for ProxyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ConnectionFailed(err) => Some(err),
            _ => None,
        }
    }
}
