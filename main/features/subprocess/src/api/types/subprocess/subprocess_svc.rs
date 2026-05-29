//! `SubprocessSvc` — factory type for subprocess execution primitives.

/// Factory for subprocess execution primitives.
///
/// All construction entry points live as associated functions on this type.
/// Callers use `SubprocessSvc::runner()`, `SubprocessSvc::config_builder()`, etc.
pub struct SubprocessSvc;
