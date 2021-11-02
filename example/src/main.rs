// #![allow(unused_imports, deprecated, unused_must_use, unused_mut, unused_variables, dead_code)]

#[macro_use]
extern crate cherry_derive;

use std::any::{Any, TypeId};
use std::collections::BTreeMap;
use std::iter::FromIterator;

// use cherry::connection::{self, PoolConfig};
// use cherry::DataSource;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // connection::setup_pools(pool_config()).await?;

    // Update::and_where();


    todo!()
}


// fn pool_config() -> BTreeMap<TypeId, PoolConfig> {
//     BTreeMap::from_iter([
//         (Other.type_id(), PoolConfig {
//             url: "mysql://root:12345678@localhost:3306/other".to_owned(),
//             ..Default::default()
//         }),
//         (Another.type_id(), PoolConfig {
//             url: "mysql://root:12345678@localhost:3306/another".to_owned(),
//             ..Default::default()
//         }),
//     ])
// }

// struct Other;
// struct Another;
//
// impl DataSource for Other {}
// impl DataSource for Another {}

#[derive(Cherry)]
struct User {
    id: u64,
    name: String,
}