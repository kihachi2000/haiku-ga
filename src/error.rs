use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    DotEnvNotFound,
    VariableNotFound(String),
    FileWriteError(String),
    ConnectionError,
    ParseError,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::DotEnvNotFound => write!(f, ".env not found"),
            Self::VariableNotFound(key) => write!(f, r#"variable "{}" not found"#, key),
            Self::FileWriteError(file_name) => write!(f, "failed to write to {}", file_name),
            Self::ConnectionError => write!(f, "failed to get response from Gemini"),
            Self::ParseError => write!(f, "failed to parse response"),
        }
    }
}

impl std::error::Error for Error {}
