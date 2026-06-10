//! Subprocess value objects — invocation inputs, policy, and outcomes.

pub mod args;
pub mod args_builder;
pub mod config;
pub mod config_builder;
pub mod result;

pub use args::SubprocessArgs;
pub use args_builder::SubprocessArgsBuilder;
pub use config::SubprocessConfig;
pub use config_builder::SubprocessConfigBuilder;
pub use result::SubprocessResult;
