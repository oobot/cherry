use cherry::pool::sqlite::SqlitePool;

#[async_std::test]
async fn test() {
    let pool = SqlitePool::connect("sqlite://:memory:").await.unwrap();
}