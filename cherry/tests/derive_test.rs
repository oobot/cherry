use sqlx::{Executor, Sqlite};

use cherry::arguments::sqlite::SqliteArguments;
use cherry::Cherry;
use cherry::pool::sqlite::SqlitePool;
use cherry::query::r#where::Where;
use cherry::query::select::select::Select;
use cherry_derive::Cherry;

#[derive(Cherry)]
struct User {
    id: u32,
    name: String,
    age: u8,
}

#[test]
fn test() {
    let table = <User as Cherry<'_, Sqlite, SqliteArguments<'_>>>::table();
    assert_eq!("user", table);
    assert_eq!(vec![("id", "id"), ("name", "name"), ("age", "age")], <User as Cherry<Sqlite, SqliteArguments>>::columns());
}

#[async_std::test]
async fn test_select() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    pool.inner.execute(include_str!("migrations.sql")).await.unwrap();
    // sqlx::migrate!("migrations.sql").run(&pool).await.unwrap();
    let user = User { id: 100, name: "The user name".to_string(), age: 25, };

    let mut arguments = SqliteArguments::new();
    arguments.add(&user.id);
    arguments.add(&user.name);
    arguments.add(&user.age);

    let a = sqlx::query_with("insert into user (id, name, age) values (?, ?, ?)", arguments)
        .execute(&pool.inner).await.unwrap();
    assert_eq!(1, a.rows_affected());

    let user: Option<User> = Select::new(&mut String::new())
        .and_eq("id", user.id)
        .and_eq("name", user.name)
        .one(&pool.inner).await.unwrap();

    assert!(user.is_some());
    assert_eq!(100, user.unwrap().id);
}

#[async_std::test]
async fn test_arguments() {
    // SqlitePool::connect()
    let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
    pool.execute(include_str!("migrations.sql")).await.unwrap();
    // sqlx::migrate!("migrations.sql").run(&pool).await.unwrap();
    let user = User { id: 100, name: "The user name".to_string(), age: 25 };

    let mut arguments = SqliteArguments::new();
    arguments.add(&user.id);
    arguments.add(&user.name);

    let a = sqlx::query_with("insert into user (id, name) values (?, ?)", arguments)
        .execute(&pool).await.unwrap();
    assert_eq!(1, a.rows_affected());
}