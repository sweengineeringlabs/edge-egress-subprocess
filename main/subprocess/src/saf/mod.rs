//! SAF layer — public factory surface for subprocess execution.

mod subprocess_svc;

pub use crate::api::{
    AllowList, Error, IsolationError, IsolationProfile, Processor, SubprocessArgs,
    SubprocessArgsBuilder, SubprocessConfig, SubprocessConfigBuilder, SubprocessResult,
    SubprocessRunner, SubprocessRunnerExtension, SubprocessSvc, SweEdgeEgressProcess, Validator,
};
pub use futures::future::BoxFuture;
