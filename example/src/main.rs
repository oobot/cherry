// #![allow(unused_imports, deprecated, unused_must_use, unused_mut, unused_variables, dead_code)]

use std::error::Error;
use cherry_derive::Cherry;

use cherry::{DataSource, PoolConfig, Transaction};

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub async fn setup() -> Result<(), Box<dyn Error>> {
    let foo_conn = PoolConfig {
        url: "mysql://root:12345678@localhost:3306/foo".to_owned(),
        ..Default::default()
    };
    let bar_conn = PoolConfig {
        url: "mysql://root:12345678@localhost:3306/bar".to_owned(),
        ..Default::default()
    };

    // Setup the database connection pools.
    Foo::setup(foo_conn).await?;
    Bar::setup(bar_conn).await?;

    Ok(())
}

pub struct Foo;
pub struct Bar;

impl DataSource for Foo {}
impl DataSource for Bar {}

#[derive(Cherry)]
struct User {
    id: u64,
    name: String,
}

#[derive(Cherry)]
#[cherry(table = "my_book")] // Change the default table name.
struct Book {
    id: u64,
    name: String,
}

/// Insert

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

    Ok(())
}

/// Delete

async fn delete() -> Result<(), Box<dyn Error>> {
    let result = Foo::delete::<User>()
        .and_where_eq("id", 100)
        .execute()
        .await?;

    Ok(())
}

/// Update

async fn update() -> Result<(), Box<dyn Error>> {
    let result = Foo::update::<User>()
        .set("name", "New Name")
        .or_where_lt("id", 100)
        .or_where_gt("id", 200)
        .execute()
        .await?;

    Ok(())
}

/// Select

async fn select() -> Result<(), Box<dyn Error>> {
    // Select optional one.
    let result: Option<User> = Foo::select()
        .and_where_eq("id", 123)
        .fetch()
        .await?;

    // Select list.
    let result: Vec<User> = Foo::select()
        .and_where_between("id", 100, 200)
        .and_where_ne("name", "Jack")
        .fetch_all()
        .await?;

    Ok(())
}

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
