//! SweEdgeEgressProcess interface — public contract for the swe service layer.

pub mod config_builder;
pub mod default;

pub use crate::api::traits::swe_edge_egress_process::SweEdgeEgressProcess;
pub use crate::api::traits::validator::Validator;
