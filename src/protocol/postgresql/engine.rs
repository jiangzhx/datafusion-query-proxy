//! Contains core interface definitions for custom SQL engines.

use crate::protocol::postgresql::protocol_ext::DataRowBatch;
use crate::protocol::postgresql::{ErrorResponse, FieldDescription};
use async_trait::async_trait;
use sqlparser::ast::Statement;

/// A Postgres portal. Portals represent a prepared statement with all parameters specified.
///
/// See Postgres' protocol docs regarding the [extended query overview](https://www.postgresql.org/docs/current/protocol-overview.html#PROTOCOL-QUERY-CONCEPTS)
/// for more details.
#[async_trait]
pub trait Portal: Send + Sync {
	/// Fetches the contents of the portal into a [DataRowBatch].
	async fn fetch(&mut self, batch: &mut DataRowBatch) -> Result<(), ErrorResponse>;
}

/// The engine trait is the core of the `convergence` crate, and is responsible for dispatching most SQL operations.
///
/// Each connection is allocated an [Engine] instance, which it uses to prepare statements, create portals, etc.
#[async_trait]
pub trait Engine: Send + Sync + 'static {
	/// The [Portal] implementation used by [Engine::create_portal].
	type PortalType: Portal;

	/// Prepares a statement, returning a vector of field descriptions for the final statement result.
	async fn prepare(&mut self, stmt: &Statement) -> Result<Vec<FieldDescription>, ErrorResponse>;

	/// Creates a new portal for the given statement.
	async fn create_portal(&mut self, stmt: &Statement) -> Result<Self::PortalType, ErrorResponse>;
}
