//! Extension interface — the contract that `core::extension` implements against.

pub mod runner;

pub use crate::api::traits::process_runner::ProcessRunner;
pub use crate::api::traits::process_runner_extension::ProcessRunnerExtension;
