//! Public API — traits, types, and errors for subprocess execution.

pub mod error;
pub mod traits;
pub mod types;
pub mod vo;

pub use error::{Error, IsolationError};
pub use traits::{
    AllowList, IsolationProfile, Processor, SubprocessRunner, SubprocessRunnerExtension,
    SweEdgeEgressProcess, Validator,
};
pub use types::SubprocessSvc;
pub use vo::{
    SubprocessArgs, SubprocessArgsBuilder, SubprocessConfig, SubprocessConfigBuilder,
    SubprocessResult,
};
