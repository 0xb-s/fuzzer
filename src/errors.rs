use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Deserialize, Serialize)]
pub enum FuzzerError {
    #[error("Input generation error: {0}")]
    InputGenerationError(String),

    #[error("Mutation error: {0}")]
    MutationError(String),

    #[error("Execution error: {0}")]
    ExecutionError(String),

    #[error("Timeout occurred")]
    TimeoutError,

    #[error("Custom error: {0}")]
    CustomError(String),

    #[error("Reproduction failled: {0}")]
    ReproductionFailed(String),
}
