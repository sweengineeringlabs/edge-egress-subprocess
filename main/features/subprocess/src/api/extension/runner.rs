//! Interface contract for `core::extension::runner`.
//!
//! [`ExtensionRunner`] implements [`ProcessRunner`] by delegating to a
//! [`ProcessRunnerExtension`] supplied by the downstream consumer.

pub use crate::api::traits::process_runner::ProcessRunner;
pub use crate::api::traits::process_runner_extension::ProcessRunnerExtension;
