use displaydoc::Display;
use thiserror::Error;

/// Errors of the execution component.
#[non_exhaustive]
#[derive(Display, Error, Debug)]
pub enum ExecutionError {
    /// Channel error
    ChannelError(String),

    /// Join error
    JoinError,

    /// crypto error: {0}
    ModelsError(#[from] massa_models::ModelsError),

    /// time error: {0}
    TimeError(#[from] massa_time::TimeError),

    /// File error
    FileError(String),

    /// Runtime error: {0}
    RuntimeError(String),
}
