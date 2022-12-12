use sqlx::{Arguments, Executor, Sqlite};
use sqlx::sqlite::SqliteArguments;

use cherry::Cherry;
use cherry::crud::select::select::Select;
use cherry::pool::sqlite::SqlitePool;
use cherry_derive::Cherry;

#[derive(Cherry)]
struct User {
    id: u32,
    name: String,
}

#[test]
fn test() {
    let table = <User as Cherry<Sqlite>>::table();
    // let table = User::table();
    assert_eq!("user", table);
    assert_eq!(vec![("id", "id"), ("name", "name")], <User as Cherry<Sqlite>>::columns());
}

#[async_std::test]
async fn test_select() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    pool.inner.execute(include_str!("migrations.sql")).await.unwrap();
    // sqlx::migrate!("migrations.sql").run(&pool).await.unwrap();
    let user = User { id: 100, name: "The user name".to_string() };

    let mut arguments = SqliteArguments::default();
    arguments.add(&user.id);
    arguments.add(&user.name);

    let a = sqlx::query_with("insert into user (id, name) values (?, ?)", arguments)
        .execute(&pool.inner).await.unwrap();
    assert_eq!(1, a.rows_affected());

    // let mut sql = String::new();
    let user: Option<User> = Select::new(&mut String::new()).by_id(100).one(&pool.inner).await.unwrap();
    assert!(user.is_some());
    assert_eq!(100, user.unwrap().id);
}

#[async_std::test]
async fn test_arguments() {
    // SqlitePool::connect()
    let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
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