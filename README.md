# Cherry 🍒

Cherry is an asynchronous ORM, support MySQL/MariaDB, PostgreSQL, SQLite and SQL Server. 
It's lightweight and build on top of [SQLx](https://github.com/launchbadge/sqlx). 

> This crate is under development (only mysql was tested partially).

```toml
[dependencies]
cherry = "0.3.0"
cherry-derive = "0.3.0"
```

```rust
use cherry::{DataSource, PoolConfig};

pub async fn setup() -> Result<(), Box<dyn Error>> {
    let conn = PoolConfig {
        url: "mysql://root:12345678@localhost:3306/foo".to_owned(),
        ..Default::default()
    };

    Foo::setup(conn).await?;

    let result: Option<User> = Foo::select()
        .and_where_eq("id", 123)
        .fetch()
        .await?;

    Ok(())
}

pub struct Foo;

impl DataSource for Foo {}

// You can setup more than one DataSources if you need.
// pub struct Bar;
// impl DataSource for Bar {}
```

## Select
```rust
async fn select() -> Result<(), Box<dyn Error>> {
    let list: Vec<User> = Foo::select()
        .and_where_between("id", 100, 200)
        .and_where_ne("name", "Jack")
        .fetch_all()
        .await?;

    Ok(())
}
```

## Insert
```rust
async fn insert() -> Result<(), Box<dyn Error>> {
    // Insert one
    let user = User { id: 1, name: "Bob".to_owned(), };
    let result = Foo::insert(&user).execute().await?;
    assert_eq!(result.rows_affected(), 1);

    // Insert multiple
    let users = [
        User { id: 2, name: "Sam".to_owned() },
        User { id: 3, name: "Jack".to_owned() }
    ];
    let result = Foo::insert_bulk(&users).execute().await?;
    assert_eq!(result.rows_affected(), 2);

    // insert replace, insert ignore etc.
    
    Ok(())
}
```

## Delete

```rust
async fn delete() -> Result<(), Box<dyn Error>> {
    let result = Foo::delete::<User>()
        .and_where_eq("id", 100)
        .execute()
        .await?;

    Ok(())
}
```

## Update

```rust
async fn update() -> Result<(), Box<dyn Error>> {
    let result = Foo::update::<User>()
        .set("name", "New Name")
        .or_where_lt("id", 100)
        .or_where_gt("id", 200)
        .execute()
        .await?;

    Ok(())
}
```

## Transaction

```rust
use cherry::Transaction;

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
    Foo::insert_bulk(&users).execute().await?;

    // Auto transaction
    Foo::insert_bulk(&users).execute_tx().await?;

    // Manual transaction
    let mut tx: Transaction = Foo::begin().await?;
    Foo::insert_bulk(&users).execute_with(&mut tx).await?;
    Foo::insert_bulk(&books).execute_with(&mut tx).await?;
    tx.commit().await?;
    // Or tx.rollback().await?;

    Ok(())
}
```

## Custom
```rust
#[derive(Cherry)]
#[cherry(table = "other_user")] // custom table name
struct User {
    id: u64,
    name: String,
}
```

## features
`mysql` is the default database, `postgres`, `sqlite`, `mssql` are other options.

`runtime-async-std-rustls` is the default connection way. `runtime-actix-native-tls`, 
`runtime-async-std-native-tls`,
`runtime-tokio-native-tls`, `runtime-actix-rustls`, `runtime-tokio-rustls` are other options.

See [SQLx Feature Flags](https://github.com/launchbadge/sqlx#cargo-feature-flags) for more details.

## Roadmap
- Custom type without sqlx imported (if possible).
- Rename struct field.
- improve databases support and more test.
- JOIN select.