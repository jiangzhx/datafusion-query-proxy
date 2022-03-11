use std::sync::Arc;
use datafusion::datasource::CsvReadOptions;
use datafusion::prelude::ExecutionContext;
use convergence::server::{self, BindOptions};
use convergence_arrow::datafusion::DataFusionEngine;
use anyhow::Result;

async fn new_engine() -> DataFusionEngine {
    let mut ctx = ExecutionContext::new();
    ctx.register_csv("test_100_4buckets", "/Users/sylar/workspace/opensource/convergence/convergence-arrow/data/100_4buckets.csv", CsvReadOptions::new())
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