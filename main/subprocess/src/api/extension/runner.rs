//! Interface contract for `core::extension::runner`.
//!
//! [`ExtensionRunner`] implements [`SubprocessRunner`] by delegating to a
//! [`SubprocessRunnerExtension`] supplied by the downstream consumer.

pub use crate::api::traits::subprocess::subprocess_runner::SubprocessRunner;
pub use crate::api::traits::subprocess::subprocess_runner_extension::SubprocessRunnerExtension;
