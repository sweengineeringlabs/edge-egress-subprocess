//! Subprocess runner traits — execution and extension contracts.

pub mod runner;
pub mod runner_extension;

pub use runner::SubprocessRunner;
pub use runner_extension::SubprocessRunnerExtension;
