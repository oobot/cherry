use sqlx::Executor;

use cherry::pool::sqlite::SqlitePool;
use cherry::query::insert::insert::Insert;
use cherry_derive::Cherry;

async fn init() -> SqlitePool {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    pool.inner.execute(include_str!("migrations.sql")).await.unwrap();
    pool
}

#[async_std::test]
async fn test_insert_one() {
    let pool = init().await;
    let user = User { id: 100, name: "test_insert_one".into(), age: 25, };
    let r = Insert::from_one(&user, &mut "".into())
        .execute(&pool.inner).await.unwrap();
    assert_eq!(1, r.rows_affected());

}

#[derive(Cherry)]
struct User {
    id: u32,
    name: String,
    age: u8,
}
