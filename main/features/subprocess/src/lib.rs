//! `swe-edge-egress-subprocess` — subprocess execution primitive.
//!
//! Provides the [`SubprocessRunner`] trait backed by [`tokio::process::Command`].
//! Consumers call [`SubprocessSvc::runner`] and receive `impl SubprocessRunner`; the
//! concrete type stays `pub(crate)` in `core/`.
//!
//! # Quick start
//!
//! ```rust,no_run
//! use swe_edge_egress_subprocess::{SubprocessSvc, SubprocessArgs, SubprocessResult, SubprocessRunner};
//!
//! # #[tokio::main]
//! # async fn main() {
//! let runner = SubprocessSvc::runner();
//!
//! let args = SubprocessArgs::builder()
//!     .argv(vec!["echo".into(), "hello".into()])
//!     .allow_commands(vec!["echo".into()])
//!     .timeout_ms(5_000)
//!     .build();
//!
//! match runner.run(args).await {
//!     SubprocessResult::Completed { exit_code, stdout, .. } => {
//!         println!("exit={exit_code} stdout={stdout}");
//!     }
//!     SubprocessResult::Denied { command } => eprintln!("denied: {command}"),
//!     SubprocessResult::TimedOut { timeout_ms } => eprintln!("timed out after {timeout_ms}ms"),
//!     SubprocessResult::SpawnFailed { reason } => eprintln!("spawn failed: {reason}"),
//!     SubprocessResult::IsolationFailed { profile, reason } => {
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
