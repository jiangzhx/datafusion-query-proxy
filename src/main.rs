use anyhow::Result;
use datafusion::prelude::{CsvReadOptions, ExecutionContext};
use datafusion_query_proxy::datafusion::DataFusionEngine;
use datafusion_query_proxy::protocol::postgresql::server;
use datafusion_query_proxy::protocol::postgresql::server::BindOptions;
use std::sync::Arc;

async fn new_engine() -> DataFusionEngine {
	let mut ctx = ExecutionContext::new();
	ctx.register_csv("test_100_4buckets", "data/100_4buckets.csv", CsvReadOptions::new())
		.await
		.expect("failed to register csv");

	DataFusionEngine::new(ctx)
}

#[tokio::main]
async fn main() -> Result<()> {
	server::run(BindOptions::new().with_port(5432), Arc::new(|| Box::pin(new_engine())))
		.await
		.unwrap();
	Ok(())
}
