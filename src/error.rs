use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    DotEnvNotFound,
    VariableNotFound(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::DotEnvNotFound => write!(f, ".env not found"),
            Self::VariableNotFound(key) => write!(f, r#"variable "{}" not found"#, key),
        }
    }
}

impl std::error::Error for Error {}
