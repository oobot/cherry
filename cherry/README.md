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
// Insert one
let user = User { id: 100, name: "Joe" };
let result = user.insert().execute(&pool).await?;
assert_eq!(1, result.rows_affected);

// Insert multiple
let users = vec![ /*...*/];
User::insert_bulk(&users).execute(&pool).await?;

// Insert ignore on conflict
User::insert_bulk(&users).ignore_on_conflict().execute(&pool).await?;

// Insert update columns if the column "id" conflict (valid for sqlite and postgres)
User::insert_bulk(&users).update_on_conflict().conflict_column("id").set_column("name").set_column("age").execute(&pool).await?;

// Insert or replace if the column "id" conflict (valid for sqlite and mysql, only sqlite can 
specify conflict column)
User::insert_bulk(&users).replace_on_conflict().execute(&pool).await?;
```

### Select
```
// Select one
let user: Option<User> = User::select().and_eq("id", 100).one(&pool).await?;

// Select in list
let users: Vec<User> = User::select().and_gt("id", 100).all(&pool).await?;

// Select in tuple
let count: Option<(u64,)> = User::select().column_raw("COUNT(*)").tuple(&pool).await?;

// Wrap conditions
// WHERE (id < 100 OR id > 200) AND age > 25
User::select().and(|c| c.or_lt_ref("id", 100).or_gt_ref("id", 200)).and_gt("age", 25).all(&pool)
await?;
```

### Update
```
User::update().set("name", "Sleepy").and_eq("id", 100).execute(&pool).await?;
```

### Delete
```
User::delete().and_eq("id", 100).execute(&pool).await?;
```

### Transaction
```
let mut tx = pool.begin().await?;
User::insert_bulk(&users).execute(&mut tx).await?;
User::update().set("name", "Sleepy").and_eq("id", 100).execute(&mut tx).await?;
tx.commit().await?;
```

### More

#### Custom table name

```
// Change the default table name "user" to "people"

#[derive(Cherry)]
#[cherry(table = "people")]
struct User {
    // fields..
}

```

#### specify particular database

By default, Cherry will implement all database types for your entity. For some reason, you can 
specify the particular database type for the entity.

Like this, cherry will implement both "mysql" and "postgres" for the "User" entity:

```toml
[dependencies]
cherry = { version = "0.4.0", features = ["postgres", "mysql", "runtime-async-std-rustls"] }
```

This will implement "mysql" only for the "User" entity:

```
#[derive(Cherry)]
#[cherry(database = "mysql")]
struct User {
    // fields..
}

```
