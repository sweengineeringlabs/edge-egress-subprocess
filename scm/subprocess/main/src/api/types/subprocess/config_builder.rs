//! `SubprocessConfigBuilder` — fluent builder for [`SubprocessConfig`].

use std::collections::HashMap;
use std::path::PathBuf;

use crate::api::types::subprocess::config::SubprocessConfig;

/// Fluent builder for [`SubprocessConfig`].
#[derive(Debug, Default)]
pub struct SubprocessConfigBuilder {
    allow_commands: Vec<String>,
    timeout_ms: Option<u64>,
    output_bytes_cap: Option<u64>,
    cwd: Option<PathBuf>,
    env: HashMap<String, String>,
    cpu_time_ms: Option<u64>,
    memory_bytes: Option<u64>,
}

impl SubprocessConfigBuilder {
    /// Set the list of allowed command basenames.
    pub fn allow_commands(mut self, commands: Vec<String>) -> Self {
        self.allow_commands = commands;
        self
    }

    /// Set the wall-clock timeout in milliseconds.
    pub fn timeout_ms(mut self, ms: u64) -> Self {
        self.timeout_ms = Some(ms);
        self
    }

    /// Set the combined stdout + stderr byte cap.
    pub fn output_bytes_cap(mut self, cap: u64) -> Self {
        self.output_bytes_cap = Some(cap);
        self
    }

    /// Set the working directory for the child process.
    pub fn cwd(mut self, cwd: impl Into<PathBuf>) -> Self {
        self.cwd = Some(cwd.into());
        self
    }

    /// Replace the environment map for the child process.
    pub fn env(mut self, env: HashMap<String, String>) -> Self {
        self.env = env;
        self
    }

    /// Set the CPU time limit in milliseconds. `0` = unlimited.
    pub fn cpu_time_ms(mut self, ms: u64) -> Self {
        self.cpu_time_ms = Some(ms);
        self
    }

    /// Set the maximum virtual address space in bytes. `0` = unlimited.
    pub fn memory_bytes(mut self, bytes: u64) -> Self {
        self.memory_bytes = Some(bytes);
        self
    }

    /// Consume the builder and return a [`SubprocessConfig`].
    pub fn build(self) -> SubprocessConfig {
        let defaults = SubprocessConfig::default();
        SubprocessConfig {
            allow_commands: self.allow_commands,
            timeout_ms: self.timeout_ms.unwrap_or(defaults.timeout_ms),
            output_bytes_cap: self.output_bytes_cap.unwrap_or(defaults.output_bytes_cap),
            cwd: self.cwd,
            env: self.env,
            cpu_time_ms: self.cpu_time_ms,
            memory_bytes: self.memory_bytes,
        }
    }
}
