//! Extension interface — the contract that `core::extension` implements against.

pub mod runner;

pub use crate::api::traits::runner::ProcessRunner;
pub use crate::api::traits::runner_extension::ProcessRunnerExtension;
