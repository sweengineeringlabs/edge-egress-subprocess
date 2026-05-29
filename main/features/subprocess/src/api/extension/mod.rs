//! Extension interface — the contract that `core::extension` implements against.

pub mod runner;

pub use crate::api::traits::subprocess::subprocess_runner::SubprocessRunner;
pub use crate::api::traits::subprocess::subprocess_runner_extension::SubprocessRunnerExtension;
