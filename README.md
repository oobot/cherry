# Cherry 🍒

Cherry is an asynchronous ORM, which support MySQL, PostgreSQL, SQLite and SQL Server. 
It's lightweight and build on top of [SQLx](https://github.com/launchbadge/sqlx). 

**WARNING: This crate is under development (mysql is partial tested at the moment).**

## Dependency
enable one of the database features ['mysql', 'postgres', 'sqlite', 'mssql'].

And enable one of the features ['runtime-actix-native-tls', 'runtime-async-std-native-tls', 
'runtime-tokio-native-tls', 'runtime-actix-rustls', 'runtime-async-std-rustls', 
'runtime-tokio-rustls']. More details see 
[SQLx Feature Flags](https://github.com/launchbadge/sqlx#cargo-feature-flags).

```toml
# Cargo.toml
[dependencies]
cherry = { version = "0.2.1", features = ["mysql", "runtime-async-std-rustls"] }
cherry-derive = "0.2.1"
```

## DataSource

You can set multiple DataSources as you need.

```rust
use std::any::Any;
use std::error::Error;

use cherry::connection::{self, PoolConfig};
use cherry::DataSource;

pub async fn setup() -> Result<(), Box<dyn Error>> {
    let config = [
        (Foo.type_id(), PoolConfig {
            url: "mysql://root:12345678@localhost:3306/foo".to_owned(),
            ..Default::default()
        }),
        (Bar.type_id(), PoolConfig {
            url: "mysql://root:12345678@localhost:3306/bar".to_owned(),
            ..Default::default()
        }),
    ];

    // Setup the database connection pools.
    connection::setup_pools(config).await?;

    Ok(())
}

pub struct Foo;
pub struct Bar;

impl DataSource for Foo {}
impl DataSource for Bar {}
```

## Model
```rust
// lib.rs/main.rs need this.
#[macro_use]
extern crate cherry_derive;

#[derive(Cherry)]
#[cherry(table = "my_user")] // Change the default table name.
pub struct User {
    pub id: u64,
    pub name: String,
}

#[derive(Cherry)]
pub struct Book {
    pub id: u64,
    pub name: String,
}
```

## Insert
```rust
async fn insert() -> Result<(), Box<dyn Error>> {
    // Insert one
    let user = User { id: 1, name: "Bob".to_owned(), };
    let result = Foo.insert(&user).execute().await?;
    assert_eq!(result.rows_affected(), 1);

    // Insert multiple
    let users = [
        User { id: 2, name: "Sam".to_owned() },
        User { id: 3, name: "Jack".to_owned() }
    ];
    let result = Foo.insert_bulk(&users).execute().await?;
    assert_eq!(result.rows_affected(), 2);

    Ok(())
}
```
Also support other insertion such as: `insert replace`, `insert ignore`  ...

## Delete

```rust
async fn delete() -> Result<(), Box<dyn Error>> {
    let result = Foo.delete::<User>()
        .and_where_eq("id", 100)
        .execute()
        .await?;

    Ok(())
}
```

## Update

```rust
async fn update() -> Result<(), Box<dyn Error>> {
    let result = Foo.update::<User>()
        .set("name", "New Name")
        .or_where_lt("id", 100)
        .or_where_gt("id", 200)
        .execute()
        .await?;

    Ok(())
}
```

## Select
```rust
async fn select() -> Result<(), Box<dyn Error>> {
    // Select optional one.
    let result: Option<User> = Foo.select()
        .and_where_eq("id", 123)
        .fetch()
        .await?;

    // Select list.
    let result: Vec<User> = Foo.select()
        .and_where_between("id", 100, 200)
        .and_where_ne("name", "Jack")
        .fetch_all()
        .await?;

    Ok(())
}
```

## Transaction

```rust
use cherry::types::Transaction;

async fn transaction() -> Result<(), Box<dyn Error>> {
    let users = [
        User { id: 1, name: "Henry".to_owned() },
        User { id: 2, name: "Jane".to_owned() }
    ];

    let books = [
        Book { id: 1, name: "Book name 1".to_owned() },
        Book { id: 2, name: "Book name 2".to_owned() }
    ];

    // Without transaction
    Foo.insert_bulk(&users).execute().await?;

    // Auto transaction
    Foo.insert_bulk(&users).execute_tx().await?;

    // Manual transaction
    let mut tx: Transaction = Foo.begin().await?;
    Foo.insert_bulk(&users).execute_with(&mut tx).await?;
    Foo.insert_bulk(&books).execute_with(&mut tx).await?;
    tx.commit().await?;
    // Or tx.rollback().await?;

    Ok(())
}
```

## TODO
- [] Custom type without sqlx imported (if possible).
- [] Rename struct field.
- [] improve databases support and more test.
- [] JOIN select.