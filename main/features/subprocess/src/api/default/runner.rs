//! Default runner interface — the public contract for `core::default::runner`.
//!
//! [`ProcessRunner`] and [`Processor`] are the traits that
//! `DefaultProcessRunner` implements.

pub use crate::api::traits::processor::Processor;
pub use crate::api::traits::process_runner::ProcessRunner;
