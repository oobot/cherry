# Cherry 🍒

Cherry is a Rust async ORM, build on top of [SQLx](https://github.com/launchbadge/sqlx),
support for MySQL, PostgreSQL and SQLite. It's designed to be easy to use.

## required features

### Database
At least one of the features must be enabled: `sqlite`, `postgres`, `mysql`.

### Async runtime
* async-std: `runtime-async-std-native-tls`, `runtime-async-std-rustls`.
* tokio: `runtime-tokio-native-tls`, `runtime-tokio-rustls`.
* actix: `runtime-actix-native-tls`, `runtime-actix-rustls`.

> The async runtime is only work for `mysql` and `postgres`. 

For example, assume you pick `sqlite`, `mysql` and async-std runtime `runtime-async-std-rustls`, 
the toml dependency should look like:

```toml
[dependencies]
cherry = { version = "0.4.0", features = ["sqlite", "mysql", "runtime-async-std-rustls"] }
```

## Example

```rust
use cherry::{Cherry, QueryExecutor};
use cherry::clause::Where;
use cherry::sqlite::SqlitePool;
// use cherry::mysql::MySqlPool;

#[derive(Cherry)]
struct User {
    id: u32,
    name: String,
}

async fn example() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    // let pool = MySqlPool::connect("mysql://username:password@localhost/test").await.unwrap();

    let user = User { id: 100, name: "Joe" };
    let result = user.insert().execute(&pool).await.unwrap();
    assert_eq!(1, result.rows_affected);

    let result = User::select().and_eq("id", &user.id).one(&pool).await.unwrap();
    assert!(result.is_some());
    assert_eq!(user.name, result.unwrap().name);
}
```

### Insert
```
let users = vec![ /*...*/];
User::insert_bulk(&users)


```


### Select
### Update
### Delete
### Transaction
### More


