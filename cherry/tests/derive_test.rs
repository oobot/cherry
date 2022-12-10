use sqlx::{Arguments, SqlitePool, Executor};
use sqlx::sqlite::SqliteArguments;

use cherry::Cherry;
use cherry_derive::Cherry;

#[derive(Cherry)]
struct User {
    id: u32,
    name: String,
}

#[test]
fn test() {
    let table = User::table();
    assert_eq!("user", table);
    assert_eq!(vec![("id", "id"), ("name", "name")], User::columns());
}

#[async_std::test]
async fn test_arguments() {
    // SqlitePool::connect()
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    pool.execute(include_str!("migrations.sql")).await.unwrap();
    // sqlx::migrate!("migrations.sql").run(&pool).await.unwrap();
    let user = User { id: 100, name: "The user name".to_string() };

    let mut arguments = SqliteArguments::default();
    arguments.add(&user.id);
    arguments.add(&user.name);

    let a = sqlx::query_with("insert into user (id, name) values (?, ?)", arguments)
        .execute(&pool).await.unwrap();
    assert_eq!(1, a.rows_affected());
}