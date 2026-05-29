//! `ProcessSvc` — factory type for subprocess execution primitives.

/// Factory for subprocess execution primitives.
///
/// All construction entry points live as associated functions on this type.
/// Callers use `ProcessSvc::runner()`, `ProcessSvc::config_builder()`, etc.
pub struct ProcessSvc;
