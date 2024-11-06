pub mod allocator;
pub mod analysis;
pub mod config;
pub mod coverage;
pub mod crash_analysis;
pub mod distributed;
pub mod errors;
pub mod fuzz_engine;
pub mod input;
pub mod logger;
pub mod mutator;
pub mod mutator_options;

pub mod reproducer;
pub mod target;
pub mod utils;
pub use crate::config::FuzzerConfig;
pub use crate::errors::FuzzerError;
pub use crate::fuzz_engine::Fuzzer;
pub use crate::target::{TargetFunction, TargetFunctionType};
