use anyhow::Result;
use datafusion::datasource::CsvReadOptions;
use datafusion::prelude::ExecutionContext;
use std::sync::Arc;
use datafusion_query_proxy::datafusion::DataFusionEngine;
use datafusion_query_proxy::protocol::server;
use datafusion_query_proxy::protocol::server::BindOptions;


async fn new_engine() -> DataFusionEngine {
	let mut ctx = ExecutionContext::new();
	ctx.register_csv(
		"test_100_4buckets",
		"/Users/sylar/workspace/opensource/convergence/convergence-arrow/data/100_4buckets.csv",
		CsvReadOptions::new(),
	)
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
