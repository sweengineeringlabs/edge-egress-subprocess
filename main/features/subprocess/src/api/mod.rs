//! Public API — traits, types, and errors for subprocess execution.

pub mod allow;
pub mod default;
pub mod error;
pub mod extension;
pub mod swe;
pub mod traits;
pub mod types;

pub use error::{Error, IsolationError};
pub use traits::{
    AllowList, IsolationProfile, ProcessRunner, Processor, SweEdgeEgressProcess, Validator,
};
pub use types::{
    ProcessArgs, ProcessArgsBuilder, ProcessConfig, ProcessConfigBuilder, ProcessResult, ProcessSvc,
};
