use cherry::Cherry;
use cherry::sqlite::SqlitePool;
use cherry::sqlx::{Arguments, Executor, Sqlite};
use cherry::sqlx::sqlite::SqliteArguments;
use cherry_derive::Cherry;

#[derive(Cherry, sqlx::FromRow)]
struct User {
    id: u32,
    name: String,
    age: u8,
}

#[test]
fn test() {
    let table = <User as Cherry<Sqlite>>::table();
    assert_eq!("user", table);
    assert_eq!(vec![("id", "id"), ("name", "name"), ("age", "age")], <User as Cherry<Sqlite>>::columns());
}

#[async_std::test]
async fn test_correct() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    pool.execute(include_str!("migrations.sql")).await.unwrap();
    // sqlx::migrate!("migrations.sql").run(&pool).await.unwrap();
    let user = User { id: 100, name: "The user name".to_string(), age: 25, };

    let mut arguments = SqliteArguments::default();
    arguments.add(&user.id);
    arguments.add(&user.name);
    arguments.add(&user.age);

    let a = sqlx::query_with("insert into user (id, name, age) values (?, ?, ?)", arguments)
        .execute(&pool).await.unwrap();
    assert_eq!(1, a.rows_affected());
}
