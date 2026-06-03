//! Default runner interface — the api/ counterpart to `core::default::runner`.
//!
//! `core::default::runner::DefaultSubprocessRunner` implements
//! [`SubprocessRunner`], which the SAF `runner()` factory returns through this
//! mirror path. The trait is defined canonically in
//! `api/traits/subprocess/subprocess_runner.rs`.

pub use crate::api::traits::subprocess::subprocess_runner::SubprocessRunner;
