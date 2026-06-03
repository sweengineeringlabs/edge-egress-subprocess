//! `DefaultSubprocessRunner` — in-process subprocess runner.
//!
//! Backed by `tokio::process::Command`.  Stateless — safe to share behind
//! `Arc<dyn SubprocessRunner>` across concurrent callers.

use std::process::Stdio;
use std::time::Duration;

use futures::future::BoxFuture;
use tokio::io::AsyncReadExt as _;
use tokio::process::Command;
use tracing::{debug, warn};

use crate::api::traits::processor::Processor;
use crate::api::traits::subprocess::subprocess_runner::SubprocessRunner;
use crate::api::types::subprocess::subprocess_args::{
    SubprocessArgs, DEFAULT_OUTPUT_BYTES_CAP, DEFAULT_TIMEOUT_MS,
};
use crate::api::types::subprocess::subprocess_result::SubprocessResult;
use crate::core::allow::NormalisedCommand;

/// Default implementation of [`SubprocessRunner`] and [`Processor`].
pub(crate) struct DefaultSubprocessRunner;

impl DefaultSubprocessRunner {
    /// Core execution logic shared by [`SubprocessRunner::run`] and [`Processor::process`].
    async fn execute(args: SubprocessArgs) -> SubprocessResult {
        // 1. Require non-empty argv.
        let Some(cmd) = args.argv.first() else {
            return SubprocessResult::Denied {
                command: String::new(),
            };
        };

        // 2. Allow-list check — deny before any spawn attempt.
        if !NormalisedCommand::is_allowed(cmd, &args.allow_commands) {
            let normalised = NormalisedCommand::normalise(cmd);
            debug!(command = %normalised, "subprocess runner denied — command not in allow-list");
            return SubprocessResult::Denied {
                command: normalised,
            };
        }

        let timeout_ms = args.timeout_ms.unwrap_or(DEFAULT_TIMEOUT_MS);
        let cap = args.output_bytes_cap.unwrap_or(DEFAULT_OUTPUT_BYTES_CAP) as usize;

        // 3. Spawn: env_clear() ensures fail-closed env isolation.
        let mut builder = Command::new(&args.argv[0]);
        builder
            .args(&args.argv[1..])
            .env_clear()
            .envs(&args.env)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true);

        if let Some(ref cwd) = args.cwd {
            builder.current_dir(cwd);
        }

        // 4. Resource limits via setrlimit (Linux only, pre-exec).
        #[cfg(unix)]
        Self::apply_rlimits(&mut builder, args.cpu_time_ms, args.memory_bytes);

        // 5. Pre-spawn isolation hook (e.g. seccomp pre_exec on Linux).
        if let Some(ref profile) = args.isolation_profile {
            if let Err(e) = profile.configure(&mut builder) {
                warn!(profile = profile.name(), error = %e, "isolation configure failed");
                return SubprocessResult::IsolationFailed {
                    profile: profile.name().to_owned(),
                    reason: e.to_string(),
                };
            }
        }

        let mut child = match builder.spawn() {
            Ok(c) => c,
            Err(e) => {
                warn!(error = %e, "subprocess runner spawn failed");
                return SubprocessResult::SpawnFailed {
                    reason: e.to_string(),
                };
            }
        };

        // 6. Post-spawn isolation hook (e.g. Windows Job Objects).
        if let Some(ref profile) = args.isolation_profile {
            if let Err(e) = profile.apply(&mut child) {
                warn!(profile = profile.name(), error = %e, "isolation apply failed");
                let _ = child.kill().await;
                return SubprocessResult::IsolationFailed {
                    profile: profile.name().to_owned(),
                    reason: e.to_string(),
                };
            }
        }

        // 7. Take pipe handles before entering the select so they can be read
        //    concurrently with child.wait().  Each stream is capped at `cap`
        //    bytes via AsyncReadExt::take, bounding live memory to 2 × cap
        //    instead of buffering the full output before truncating.
        // Both handles are always Some because Stdio::piped() was set above.
        let Some(stdout_handle) = child.stdout.take() else {
            let _ = child.kill().await;
            return SubprocessResult::SpawnFailed {
                reason: "stdout pipe was not established".to_owned(),
            };
        };
        let Some(stderr_handle) = child.stderr.take() else {
            let _ = child.kill().await;
            return SubprocessResult::SpawnFailed {
                reason: "stderr pipe was not established".to_owned(),
            };
        };

        tokio::select! {
            result = async {
                let (stdout_bytes, stderr_bytes, status) = tokio::join!(
                    async {
                        let mut buf = Vec::new();
                        let _ = stdout_handle.take(cap as u64).read_to_end(&mut buf).await;
                        buf
                    },
                    async {
                        let mut buf = Vec::new();
                        let _ = stderr_handle.take(cap as u64).read_to_end(&mut buf).await;
                        buf
                    },
                    child.wait(),
                );
                (stdout_bytes, stderr_bytes, status)
            } => {
                let (stdout_bytes, stderr_bytes, status) = result;
                let exit_code = status.map(|s| s.code().unwrap_or(-1)).unwrap_or(-1);
                // 8. Apply combined cap: stdout gets first `cap` bytes; stderr
                //    gets the remainder up to `cap - stdout_len`.
                let stdout_end = stdout_bytes.len().min(cap);
                let stderr_end = stderr_bytes.len().min(cap.saturating_sub(stdout_bytes.len()));
                SubprocessResult::Completed {
                    exit_code,
                    stdout: String::from_utf8_lossy(&stdout_bytes[..stdout_end]).into_owned(),
                    stderr: String::from_utf8_lossy(&stderr_bytes[..stderr_end]).into_owned(),
                }
            }
            _ = tokio::time::sleep(Duration::from_millis(timeout_ms)) => {
                let _ = child.kill().await;
                SubprocessResult::TimedOut { timeout_ms }
            }
        }
    }

    /// Register a `pre_exec` hook that calls `setrlimit` for CPU and memory.
    ///
    /// Both limits are applied in the child after `fork()`, before `exec()`.
    /// A value of `0` or `None` leaves the corresponding limit untouched.
    ///
    /// # Safety
    ///
    /// `setrlimit` is async-signal-safe and valid in the post-fork child context.
    /// The `unsafe` block is intentionally minimal — only the two `setrlimit`
    /// calls are inside it.
    #[cfg(unix)]
    #[allow(unsafe_code)]
    fn apply_rlimits(cmd: &mut Command, cpu_time_ms: Option<u64>, memory_bytes: Option<u64>) {
        use std::os::unix::process::CommandExt as _;
        // SAFETY: setrlimit(2) is async-signal-safe and valid in the post-fork child
        // context established by Command::pre_exec. The closure captures only Copy
        // types (u64), so there are no aliasing concerns.
        unsafe {
            cmd.as_std_mut().pre_exec(move || {
                if let Some(ms) = cpu_time_ms.filter(|&v| v > 0) {
                    let secs = (ms + 999) / 1_000; // ceiling division
                    let _ = nix::sys::resource::setrlimit(
                        nix::sys::resource::Resource::RLIMIT_CPU,
                        secs,
                        secs,
                    );
                }
                if let Some(bytes) = memory_bytes.filter(|&v| v > 0) {
                    let _ = nix::sys::resource::setrlimit(
                        nix::sys::resource::Resource::RLIMIT_AS,
                        bytes,
                        bytes,
                    );
                }
                Ok(())
            });
        }
    }
}

impl SubprocessRunner for DefaultSubprocessRunner {
    fn run(&self, args: SubprocessArgs) -> BoxFuture<'_, SubprocessResult> {
        Box::pin(DefaultSubprocessRunner::execute(args))
    }
}

impl Processor for DefaultSubprocessRunner {
    fn process(&self, args: SubprocessArgs) -> BoxFuture<'_, SubprocessResult> {
        Box::pin(DefaultSubprocessRunner::execute(args))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_run_empty_argv_returns_denied_with_empty_command() {
        let runner = DefaultSubprocessRunner;
        let args = SubprocessArgs::builder().build();
        let result = runner.run(args).await;
        let SubprocessResult::Denied { command } = result else {
            panic!("expected Denied, got {result:?}");
        };
        assert!(
            command.is_empty(),
            "empty argv must produce empty command in Denied"
        );
    }
}
