//! `swe-edge-egress-process` — subprocess execution primitive.
//!
//! Provides the [`ProcessRunner`] trait backed by [`tokio::process::Command`].
//! Consumers call [`process_runner`] and receive `impl ProcessRunner`; the
//! concrete type stays `pub(crate)` in `core/`.
//!
//! # Quick start
//!
//! ```rust,no_run
//! use swe_edge_egress_subprocess::{ProcessSvc, ProcessArgs, ProcessResult, ProcessRunner};
//!
//! # #[tokio::main]
//! # async fn main() {
//! let runner = ProcessSvc::runner();
//!
//! let args = ProcessArgs::builder()
//!     .argv(vec!["echo".into(), "hello".into()])
//!     .allow_commands(vec!["echo".into()])
//!     .timeout_ms(5_000)
//!     .build();
//!
//! match runner.run(args).await {
//!     ProcessResult::Completed { exit_code, stdout, .. } => {
//!         println!("exit={exit_code} stdout={stdout}");
//!     }
//!     ProcessResult::Denied { command } => eprintln!("denied: {command}"),
//!     ProcessResult::TimedOut { timeout_ms } => eprintln!("timed out after {timeout_ms}ms"),
//!     ProcessResult::SpawnFailed { reason } => eprintln!("spawn failed: {reason}"),
//!     ProcessResult::IsolationFailed { profile, reason } => {
//!         eprintln!("isolation failed ({profile}): {reason}");
//!     }
//! }
//! # }
//! ```

#![warn(missing_docs)]
#![deny(unsafe_code)]

mod api;
mod core;
mod gateway;
mod saf;
mod spi;

pub use gateway::*;
