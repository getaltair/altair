use altaird::testing::TestDb;

#[tokio::test]
async fn migrations_apply_to_a_fresh_database() {
    let db = TestDb::new().await;
    let tables: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM information_schema.tables WHERE table_schema = 'public'",
    )
    .fetch_one(&db.pool)
    .await
    .expect("query");
    assert!(tables > 0, "migrations produced no tables");
}

#[tokio::test]
async fn each_test_gets_its_own_database() {
    let (a, b) = (TestDb::new().await, TestDb::new().await);
    sqlx::query("CREATE TABLE only_in_a (x int)")
        .execute(&a.pool)
        .await
        .expect("create in a");
    let exists: bool = sqlx::query_scalar("SELECT to_regclass('only_in_a') IS NOT NULL")
        .fetch_one(&b.pool)
        .await
        .expect("query b");
    assert!(!exists, "databases are not isolated");
}
