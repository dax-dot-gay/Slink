use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Clone, Debug, Serialize, Deserialize)]
pub enum Error {
    #[error("An unexpected error occurred: {0}")]
    Unexpected(String),

    #[error("A process runner ({scope}:{runner}) with ID {id} failed: {reason}")]
    RunnerError {
        scope: String,
        runner: String,
        id: String,
        reason: String
    }
}

impl Error {
    pub fn unexpected<T>(err: impl std::error::Error) -> Res<T> {
        Err(Self::Unexpected(err.to_string()))
    }
}

pub type Res<T> = Result<T, Error>;
