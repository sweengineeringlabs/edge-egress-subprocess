//! Default runner interface — the public contract for `core::default::runner`.
//!
//! [`SubprocessRunner`] and [`Processor`] are the traits that
//! `DefaultSubprocessRunner` implements.

pub use crate::api::traits::processor::Processor;
pub use crate::api::traits::subprocess::subprocess_runner::SubprocessRunner;
