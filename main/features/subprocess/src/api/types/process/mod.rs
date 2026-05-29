//! Subprocess process types — args, config, result, and factory.

pub mod args;
pub mod args_builder;
pub mod config;
pub mod config_builder;
pub mod outcome;
pub mod svc;

pub use args::ProcessArgs;
pub use args_builder::ProcessArgsBuilder;
pub use config::ProcessConfig;
pub use config_builder::ProcessConfigBuilder;
pub use outcome::ProcessResult;
pub use svc::ProcessSvc;
