use crate::errors::FuzzerError;
use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

#[derive(Clone)]
pub enum TargetFunctionType {
    Sync(fn(&[u8]) -> Result<(), FuzzerError>),
    Async(
        Arc<
            dyn Fn(&[u8]) -> Pin<Box<dyn Future<Output = Result<(), FuzzerError>> + Send>>
                + Send
                + Sync,
        >,
    ),
}

#[derive(Clone)]
pub struct TargetFunction {
    pub name: String,
    pub func_type: TargetFunctionType,
}

impl TargetFunction {
    pub fn new_sync(name: &str, func: fn(&[u8]) -> Result<(), FuzzerError>) -> Self {
        TargetFunction {
            name: name.to_string(),
            func_type: TargetFunctionType::Sync(func),
        }
    }

    pub fn new_async<F>(name: &str, func: F) -> Self
    where
        F: Fn(&[u8]) -> Pin<Box<dyn Future<Output = Result<(), FuzzerError>> + Send>>
            + Send
            + Sync
            + 'static,
    {
        TargetFunction {
            name: name.to_string(),
            func_type: TargetFunctionType::Async(Arc::new(func)),
        }
    }
}

#[async_trait]
pub trait Executable {
    async fn execute(&self, input: &[u8]) -> Result<(), FuzzerError>;
}

#[async_trait]
impl Executable for TargetFunction {
    async fn execute(&self, input: &[u8]) -> Result<(), FuzzerError> {
        match &self.func_type {
            TargetFunctionType::Sync(func) => func(input),
            TargetFunctionType::Async(func) => (func)(input).await,
        }
    }
}
