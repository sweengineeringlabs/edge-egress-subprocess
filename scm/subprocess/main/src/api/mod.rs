//! Public API — traits, types, and errors for subprocess execution.

pub mod command;
pub mod default;
pub mod error;
pub mod extension;
pub mod traits;
pub mod types;

pub use error::{Error, IsolationError};
pub use traits::{
    AllowList, IsolationProfile, Processor, SubprocessRunner, SubprocessRunnerExtension,
    SweEdgeEgressProcess, Validator,
};
pub use types::subprocess::{
    SubprocessArgs, SubprocessArgsBuilder, SubprocessConfig, SubprocessConfigBuilder,
    SubprocessResult,
};
pub use types::SubprocessSvc;
