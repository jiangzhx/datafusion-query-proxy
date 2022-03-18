//! Convergence is a crate for writing servers that speak PostgreSQL's wire protocol.
//! Contains types that represent the core Postgres wire protocol.

// this module requires a lot more work to document
// may want to build this automatically from Postgres docs if possible
#![allow(missing_docs)]

pub mod mysql;
pub mod postgresql;
