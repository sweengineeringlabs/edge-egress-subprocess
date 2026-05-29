//! `ProcessArgsBuilder` — builder for [`ProcessArgs`].

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use crate::api::traits::isolation_profile::IsolationProfile;
use crate::api::types::process::process_args::ProcessArgs;

/// Builder for [`ProcessArgs`].
#[derive(Debug, Default)]
pub struct ProcessArgsBuilder {
    argv: Vec<String>,
    cwd: Option<PathBuf>,
    env: HashMap<String, String>,
    allow_commands: Vec<String>,
    timeout_ms: Option<u64>,
    output_bytes_cap: Option<u64>,
    cpu_time_ms: Option<u64>,
    memory_bytes: Option<u64>,
    isolation_profile: Option<Arc<dyn IsolationProfile>>,
}

impl ProcessArgsBuilder {
    /// Set `argv`.  `argv[0]` is the binary; remaining entries are arguments.
    pub fn argv(mut self, argv: Vec<String>) -> Self {
        self.argv = argv;
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

    /// Set the OS-level isolation profile.
    pub fn isolation_profile(mut self, profile: Arc<dyn IsolationProfile>) -> Self {
        self.isolation_profile = Some(profile);
        self
    }

    /// Consume the builder and return a [`ProcessArgs`].
    pub fn build(self) -> ProcessArgs {
        ProcessArgs {
            argv: self.argv,
            cwd: self.cwd,
            env: self.env,
            allow_commands: self.allow_commands,
            timeout_ms: self.timeout_ms,
            output_bytes_cap: self.output_bytes_cap,
            cpu_time_ms: self.cpu_time_ms,
            memory_bytes: self.memory_bytes,
            isolation_profile: self.isolation_profile,
        }
    }
}
