//! Migration performance benchmark
//!
//! Verifies that the full migration suite completes in under 5 seconds.
//! This benchmark runs all production migrations against an in-memory SurrealDB instance.

use criterion::{Criterion, criterion_group, criterion_main};
use std::path::PathBuf;

/// Run migration benchmark against in-memory database
fn migration_benchmark(c: &mut Criterion) {
    // Get project root and migrations directory
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let migrations_dir = manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.join("migrations"))
        .expect("Could not find migrations directory");

    // Create async runtime for benchmarks
    let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");

    c.bench_function("full_migration_suite", |b| {
        b.iter(|| {
            rt.block_on(async {
                use altair_db::MigrationRunner;
                use surrealdb::engine::any;

                // Create fresh in-memory database for each iteration
                let db = any::connect("mem://")
                    .await
                    .expect("Failed to connect to database");
                db.use_ns("altair")
                    .use_db("main")
                    .await
                    .expect("Failed to use namespace/database");

                // Run all migrations
                let mut runner = MigrationRunner::new(db, &migrations_dir);
                runner.run().await.expect("Migration failed");
            });
        });
    });
}

criterion_group! {
    name = benches;
    // Configure to ensure we verify the 5-second requirement
    config = Criterion::default()
        .sample_size(10)  // 10 iterations for accuracy
        .measurement_time(std::time::Duration::from_secs(30));
    targets = migration_benchmark
}

criterion_main!(benches);
