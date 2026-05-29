//! Subprocess process types — args, config, result, and factory.

pub mod process_args;
pub mod process_args_builder;
pub mod process_config;
pub mod process_config_builder;
pub mod process_result;
pub mod process_svc;

pub use process_args::ProcessArgs;
pub use process_args_builder::ProcessArgsBuilder;
pub use process_config::ProcessConfig;
pub use process_config_builder::ProcessConfigBuilder;
pub use process_result::ProcessResult;
pub use process_svc::ProcessSvc;
